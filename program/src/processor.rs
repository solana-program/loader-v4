//! Program processor.

use {
    crate::{
        instruction::LoaderV4Instruction,
        state::{LoaderV4State, LoaderV4Status},
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        sysvar::Sysvar,
    },
};

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
            msg!("Insufficient lamports, {} are required", required_lamports);
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
fn process_deploy(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

/// Processes a
/// [Retract](enum.LoaderV4Instruction.html)
/// instruction.
fn process_retract(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
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
