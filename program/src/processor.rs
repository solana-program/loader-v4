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
fn process_truncate(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _new_size: u32,
) -> ProgramResult {
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
