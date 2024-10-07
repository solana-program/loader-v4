//! Program processor.

use {
    crate::{
        instruction::LoaderV4Instruction,
        state::{LoaderV4State, LoaderV4Status, DEPLOYMENT_COOLDOWN_IN_SLOTS},
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::{Clock, Slot},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        sysvar::Sysvar,
    },
};

// Keep in sync with the constant from the program-runtime.
// https://github.com/anza-xyz/agave/blob/1e389f48636cf7e710f38f154b9d683c15d1cb0c/program-runtime/src/loaded_programs.rs#L37
pub const DELAY_VISIBILITY_SLOT_OFFSET: Slot = 1;

// [Core BPF]: Locally-implemented
// `solana_sdk::program_utils::limited_deserialize`.
fn limited_deserialize<T>(input: &[u8]) -> Result<T, ProgramError>
where
    T: serde::de::DeserializeOwned,
{
    solana_program::program_utils::limited_deserialize(
        input, 1232, // [Core BPF]: See `solana_sdk::packet::PACKET_DATA_SIZE`
    )
    .map_err(|_| ProgramError::InvalidInstructionData)
}

fn check_program_account(
    program_id: &Pubkey,
    program_info: &AccountInfo,
    authority_info: &AccountInfo,
) -> Result<LoaderV4State, ProgramError> {
    if program_info.owner != program_id {
        msg!("Program not owned by loader");
        return Err(ProgramError::InvalidAccountOwner);
    }
    let data = program_info.try_borrow_data()?;
    let state = LoaderV4State::unpack(&data)?;
    if !program_info.is_writable {
        msg!("Program is not writeable");
        return Err(ProgramError::InvalidArgument);
    }
    if !authority_info.is_signer {
        msg!("Authority did not sign");
        return Err(ProgramError::MissingRequiredSignature);
    }
    if state.authority_address_or_next_version != *authority_info.key {
        msg!("Incorrect authority provided");
        return Err(ProgramError::IncorrectAuthority);
    }
    if matches!(state.status, LoaderV4Status::Finalized) {
        msg!("Program is finalized");
        return Err(ProgramError::Immutable);
    }
    Ok(*state)
}

/// Processes an
/// [Write](enum.LoaderV4Instruction.html)
/// instruction.
fn process_write(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    offset: u32,
    bytes: Vec<u8>,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let program_info = next_account_info(accounts_iter)?;
    let authority_info = next_account_info(accounts_iter)?;

    let state = check_program_account(program_id, program_info, authority_info)?;

    if !matches!(state.status, LoaderV4Status::Retracted) {
        msg!("Program is not retracted");
        return Err(ProgramError::InvalidArgument);
    }

    let end_offset = (offset as usize).saturating_add(bytes.len());

    program_info
        .try_borrow_mut_data()?
        .get_mut(
            LoaderV4State::program_data_offset().saturating_add(offset as usize)
                ..LoaderV4State::program_data_offset().saturating_add(end_offset),
        )
        .ok_or_else(|| {
            msg!("Write out of bounds");
            ProgramError::AccountDataTooSmall
        })?
        .copy_from_slice(&bytes);
    Ok(())
}

/// Processes a
/// [Truncate](enum.LoaderV4Instruction.html)
/// instruction.
fn process_truncate(program_id: &Pubkey, accounts: &[AccountInfo], new_size: u32) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let program_info = next_account_info(accounts_iter)?;
    let authority_info = next_account_info(accounts_iter)?;

    let is_initialization =
        new_size > 0 && program_info.data_len() < LoaderV4State::program_data_offset();

    if is_initialization {
        if program_info.owner != program_id {
            msg!("Program not owned by loader");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if !program_info.is_writable {
            msg!("Program is not writeable");
            return Err(ProgramError::InvalidArgument);
        }
        if !program_info.is_signer {
            msg!("Program did not sign");
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !authority_info.is_signer {
            msg!("Authority did not sign");
            return Err(ProgramError::MissingRequiredSignature);
        }
    } else {
        let state = check_program_account(program_id, program_info, authority_info)?;
        if !matches!(state.status, LoaderV4Status::Retracted) {
            msg!("Program is not retracted");
            return Err(ProgramError::InvalidArgument);
        }
    }

    let required_lamports = if new_size == 0 {
        0
    } else {
        let rent = <Rent as Sysvar>::get()?;
        rent.minimum_balance(LoaderV4State::program_data_offset().saturating_add(new_size as usize))
            .max(1)
    };

    match program_info.lamports().cmp(&required_lamports) {
        std::cmp::Ordering::Less => {
            msg!("Insufficient lamports, {} are required.", required_lamports);
            return Err(ProgramError::InsufficientFunds);
        }
        std::cmp::Ordering::Greater => {
            let destination_info = next_account_info(accounts_iter)?;
            if !destination_info.is_writable {
                msg!("Recipient is not writeable");
                return Err(ProgramError::InvalidArgument);
            }
            let lamports_to_receive = program_info.lamports().saturating_sub(required_lamports);
            let new_destination_lamports = destination_info
                .lamports()
                .saturating_add(lamports_to_receive);
            **program_info.try_borrow_mut_lamports()? = required_lamports;
            **destination_info.try_borrow_mut_lamports()? = new_destination_lamports;
        }
        std::cmp::Ordering::Equal => {}
    }

    if new_size == 0 {
        program_info.realloc(0, true)?;
    } else {
        program_info.realloc(
            LoaderV4State::program_data_offset().saturating_add(new_size as usize),
            true,
        )?;
        if is_initialization {
            let mut data = program_info.try_borrow_mut_data()?;
            let state = LoaderV4State::unpack_mut(&mut data)?;
            state.slot = 0;
            state.status = LoaderV4Status::Retracted;
            state.authority_address_or_next_version = *authority_info.key;
        }
    }

    Ok(())
}

/// Processes a
/// [Deploy](enum.LoaderV4Instruction.html)
/// instruction.
fn process_deploy(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let program_info = next_account_info(accounts_iter)?;
    let authority_info = next_account_info(accounts_iter)?;
    let source_info = next_account_info(accounts_iter).ok();

    let state = check_program_account(program_id, program_info, authority_info)?;

    let current_slot = <Clock as Sysvar>::get()?.slot;

    // Slot = 0 indicates that the program hasn't been deployed yet. So no need
    // to check for the cooldown slots.
    // (Without this check, the program deployment is failing in freshly
    // started test validators. That's because at startup current_slot is 0,
    // which is < DEPLOYMENT_COOLDOWN_IN_SLOTS).
    if state.slot != 0 && state.slot.saturating_add(DEPLOYMENT_COOLDOWN_IN_SLOTS) > current_slot {
        msg!("Program was deployed recently, cooldown still in effect");
        return Err(ProgramError::InvalidArgument);
    }

    if !matches!(state.status, LoaderV4Status::Retracted) {
        msg!("Destination program is not retracted");
        return Err(ProgramError::InvalidArgument);
    }

    let buffer_info = if let Some(ref source_program) = source_info {
        let source_state = check_program_account(program_id, source_program, authority_info)?;
        if !matches!(source_state.status, LoaderV4Status::Retracted) {
            msg!("Source program is not retracted");
            return Err(ProgramError::InvalidArgument);
        }
        source_program
    } else {
        &program_info
    };

    let _programdata = buffer_info
        .try_borrow_data()?
        .get(LoaderV4State::program_data_offset()..)
        .ok_or(ProgramError::AccountDataTooSmall)?;

    let deployment_slot = state.slot;
    let _effective_slot = deployment_slot.saturating_add(DELAY_VISIBILITY_SLOT_OFFSET);

    // [CORE BPF]: We'll see what happens with on-chain verification...
    // Something like this would be nice:
    // invoke(
    //     &solana_bpf_verify_program::instruction::verify(buffer_info.key),
    //     &[buffer_info.clone()],
    // )?;

    if let Some(source_info) = source_info {
        let rent = <Rent as Sysvar>::get()?;
        let required_lamports = rent.minimum_balance(source_info.data_len());
        let transfer_lamports = required_lamports.saturating_sub(program_info.lamports());
        let new_program_lamports = program_info.lamports().saturating_add(transfer_lamports);
        let new_source_lamports = source_info.lamports().saturating_sub(transfer_lamports);

        {
            if program_info.data_len() < source_info.data_len() {
                program_info.realloc(source_info.data_len(), true)?;
            }
            let mut program_data = program_info.try_borrow_mut_data()?;
            let source_data = source_info.try_borrow_mut_data()?;
            program_data[..].copy_from_slice(&source_data[..]);
        }
        source_info.realloc(0, true)?;

        **program_info.try_borrow_mut_lamports()? = new_program_lamports;
        **source_info.try_borrow_mut_lamports()? = new_source_lamports;
    }
    let mut data = program_info.try_borrow_mut_data()?;
    let state = LoaderV4State::unpack_mut(&mut data)?;
    state.slot = current_slot;
    state.status = LoaderV4Status::Deployed;

    // [CORE BPF]: Store modified entry in program cache.

    Ok(())
}

/// Processes a
/// [Retract](enum.LoaderV4Instruction.html)
/// instruction.
fn process_retract(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let program_info = next_account_info(accounts_iter)?;
    let authority_info = next_account_info(accounts_iter)?;

    let state = check_program_account(program_id, program_info, authority_info)?;

    let current_slot = <Clock as Sysvar>::get()?.slot;

    if state.slot.saturating_add(DEPLOYMENT_COOLDOWN_IN_SLOTS) > current_slot {
        msg!("Program was deployed recently, cooldown still in effect");
        return Err(ProgramError::InvalidArgument);
    }

    if !matches!(state.status, LoaderV4Status::Deployed) {
        msg!("Program is not deployed");
        return Err(ProgramError::InvalidArgument);
    }

    let mut data = program_info.try_borrow_mut_data()?;
    let state = LoaderV4State::unpack_mut(&mut data)?;
    state.status = LoaderV4Status::Retracted;

    // [CORE BPF]: Store modified entry in program cache.

    Ok(())
}

/// Processes a
/// [TransferAuthority](enum.LoaderV4Instruction.html)
/// instruction.
fn process_transfer_authority(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

/// Processes a
/// [Finalize](enum.LoaderV4Instruction.html)
/// instruction.
fn process_finalize(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

/// Processes a
/// [LoaderV4Instruction](enum.LoaderV4Instruction.html).
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    match limited_deserialize(input)? {
        LoaderV4Instruction::Write { offset, bytes } => {
            msg!("Instruction: Write");
            process_write(program_id, accounts, offset, bytes)
        }
        LoaderV4Instruction::Truncate { new_size } => {
            msg!("Instruction: Truncate");
            process_truncate(program_id, accounts, new_size)
        }
        LoaderV4Instruction::Deploy => {
            msg!("Instruction: Deploy");
            process_deploy(program_id, accounts)
        }
        LoaderV4Instruction::Retract => {
            msg!("Instruction: Retract");
            process_retract(program_id, accounts)
        }
        LoaderV4Instruction::TransferAuthority => {
            msg!("Instruction: TransferAuthority");
            process_transfer_authority(program_id, accounts)
        }
        LoaderV4Instruction::Finalize => {
            msg!("Instruction: Finalize");
            process_finalize(program_id, accounts)
        }
    }
}
