//! Program processor.

use {
    crate::instruction::LoaderV4Instruction,
    solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey},
};

/// Processes an
/// [Write](enum.LoaderV4Instruction.html)
/// instruction.
fn process_write(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _offset: u32,
    _bytes: Vec<u8>,
) -> ProgramResult {
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
    let instruction = LoaderV4Instruction::unpack(input)?;
    match instruction {
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
