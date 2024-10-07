//! Program instruction types.

use {
    serde::{Deserialize, Serialize},
    shank::ShankInstruction,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
};

/// Instructions supported by the Solana BPF Loader v4 program.
#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ShankInstruction)]
pub enum LoaderV4Instruction {
    /// Write ELF data into an undeployed program account.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` Program account to write to.
    /// 1. `[s]` Program authority.
    #[account(
        0,
        writable,
        name = "program",
        desc = "Program account to write to."
    )]
    #[account(
        1,
        signer,
        name = "authority",
        desc = "Program authority."
    )]
    Write {
        /// Offset at which to write the given bytes.
        offset: u32,
        /// Serialized program data.
        bytes: Vec<u8>,
    },

    /// Changes the size of an undeployed program account.
    ///
    /// A program account is automatically initialized when its size is first
    /// increased.
    /// In this initial truncate, the program account needs to be a signer and
    /// it also sets the authority needed for subsequent operations.
    /// Decreasing to size zero closes the program account and resets it
    /// into an uninitialized state.
    /// Providing additional lamports upfront might be necessary to reach rent
    /// exemption.
    /// Superflous funds are transferred to the recipient account.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w, s]` Program account to change the size of.
    /// 1. `[s]` Program authority.
    /// 2. `[w]` Destination account for reclaimed lamports (optional).
    #[account(
        0,
        writable,
        signer,
        name = "program",
        desc = "Program account to change the size of."
    )]
    #[account(
        1,
        signer,
        name = "authority",
        desc = "Program authority."
    )]
    #[account(
        2,
        writable,
        optional,
        name = "destination",
        desc = "Destination account for reclaimed lamports (optional)."
    )]
    Truncate {
        /// The new size after the operation.
        new_size: u32,
    },

    /// Verify the data of a program account to be a valid ELF.
    ///
    /// If this succeeds the program becomes executable, and is ready to use.
    /// A source program account can be provided to overwrite the data before
    /// deployment in one step, instead retracting the program and writing to
    /// it and redeploying it.
    /// The source program is truncated to zero (thus closed) and lamports
    /// necessary for rent exemption are transferred, in case that the source
    /// was bigger than the program.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` Program account to deploy.
    /// 1. `[s]` Program authority.
    /// 2. `[w]` Undeployed source program account to take data and lamports
    ///    from (optional).
    #[account(
        0,
        writable,
        name = "program",
        desc = "Program account to deploy."
    )]
    #[account(
        1,
        signer,
        name = "authority",
        desc = "Program authority."
    )]
    #[account(
        2,
        writable,
        optional,
        name = "source",
        desc = "Undeployed source program account to take data and lamports from (optional)."
    )]
    Deploy,

    /// Undo the deployment of a program account.
    ///
    /// The program is no longer executable and goes into maintenance.
    /// Necessary for writing data and truncating.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` Program account to retract.
    /// 1. `[s]` Program authority.
    #[account(
        0,
        writable,
        name = "program",
        desc = "Program account to retract."
    )]
    #[account(
        1,
        signer,
        name = "authority",
        desc = "Program authority."
    )]
    Retract,

    /// Transfers the authority over a program account.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` Program account to change the authority of.
    /// 1. `[s]` Current program authority.
    /// 2. `[s]` New program authority.
    #[account(
        0,
        writable,
        name = "program",
        desc = "Program account to change the authority of."
    )]
    #[account(
        1,
        signer,
        name = "current_authority",
        desc = "Current program authority."
    )]
    #[account(
        2,
        signer,
        name = "new_authority",
        desc = "New program authority."
    )]
    TransferAuthority,

    /// Finalizes the program account, rendering it immutable.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` Program account to finalize.
    /// 1. `[s]` Program authority.
    /// 2. `[ ]` The next version of the program (can be itself).
    #[account(
        0,
        writable,
        name = "program",
        desc = "Program account to finalize."
    )]
    #[account(
        1,
        signer,
        name = "authority",
        desc = "Program authority."
    )]
    #[account(
        2,
        name = "next_version",
        desc = "The next version of the program (can be itself)."
    )]
    Finalize,
}

/// Creates a
/// [Write](enum.LoaderV4Instruction.html)
/// instruction.
pub fn write(
    program_address: &Pubkey,
    authority_address: &Pubkey,
    offset: u32,
    bytes: Vec<u8>,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*program_address, false),
        AccountMeta::new_readonly(*authority_address, true),
    ];
    Instruction::new_with_bincode(
        crate::id(),
        &LoaderV4Instruction::Write { offset, bytes },
        accounts,
    )
}

/// Creates a
/// [Truncate](enum.LoaderV4Instruction.html)
/// instruction.
pub fn truncate(
    program_address: &Pubkey,
    authority_address: &Pubkey,
    destination_address: Option<&Pubkey>,
    new_size: u32,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(*program_address, true),
        AccountMeta::new_readonly(*authority_address, true),
    ];
    if let Some(destination_address) = destination_address {
        accounts.push(AccountMeta::new(*destination_address, false));
    }
    Instruction::new_with_bincode(
        crate::id(),
        &LoaderV4Instruction::Truncate { new_size },
        accounts,
    )
}

/// Creates a
/// [Deploy](enum.LoaderV4Instruction.html)
/// instruction.
pub fn deploy(
    program_address: &Pubkey,
    authority_address: &Pubkey,
    source_address: Option<&Pubkey>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(*program_address, false),
        AccountMeta::new_readonly(*authority_address, true),
    ];
    if let Some(source_address) = source_address {
        accounts.push(AccountMeta::new(*source_address, false));
    }
    Instruction::new_with_bincode(crate::id(), &LoaderV4Instruction::Deploy, accounts)
}

/// Creates a
/// [Retract](enum.LoaderV4Instruction.html)
/// instruction.
pub fn retract(program_address: &Pubkey, authority_address: &Pubkey) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*program_address, false),
        AccountMeta::new_readonly(*authority_address, true),
    ];
    Instruction::new_with_bincode(crate::id(), &LoaderV4Instruction::Retract, accounts)
}

/// Creates a
/// [TransferAuthority](enum.LoaderV4Instruction.html)
/// instruction.
pub fn transfer_authority(
    program_address: &Pubkey,
    current_authority_address: &Pubkey,
    new_authority_address: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*program_address, false),
        AccountMeta::new_readonly(*current_authority_address, true),
        AccountMeta::new_readonly(*new_authority_address, true),
    ];
    Instruction::new_with_bincode(
        crate::id(),
        &LoaderV4Instruction::TransferAuthority,
        accounts,
    )
}

/// Creates a
/// [Finalize](enum.LoaderV4Instruction.html)
/// instruction.
pub fn finalize(
    program_address: &Pubkey,
    authority_address: &Pubkey,
    next_version_address: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*program_address, false),
        AccountMeta::new_readonly(*authority_address, true),
        AccountMeta::new_readonly(*next_version_address, false),
    ];
    Instruction::new_with_bincode(crate::id(), &LoaderV4Instruction::Finalize, accounts)
}
