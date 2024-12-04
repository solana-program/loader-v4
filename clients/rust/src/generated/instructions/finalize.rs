//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct Finalize {
    /// Program account to finalize.
    pub program: solana_program::pubkey::Pubkey,
    /// Program authority.
    pub authority: solana_program::pubkey::Pubkey,
    /// The next version of the program (can be itself).
    pub next_version: solana_program::pubkey::Pubkey,
}

impl Finalize {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.next_version,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = FinalizeInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::LOADER_V4_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct FinalizeInstructionData {
    discriminator: u32,
}

impl FinalizeInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 5 }
    }
}

impl Default for FinalizeInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `Finalize`.
///
/// ### Accounts:
///
///   0. `[writable]` program
///   1. `[signer]` authority
///   2. `[]` next_version
#[derive(Clone, Debug, Default)]
pub struct FinalizeBuilder {
    program: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    next_version: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl FinalizeBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Program account to finalize.
    #[inline(always)]
    pub fn program(&mut self, program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.program = Some(program);
        self
    }
    /// Program authority.
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }
    /// The next version of the program (can be itself).
    #[inline(always)]
    pub fn next_version(&mut self, next_version: solana_program::pubkey::Pubkey) -> &mut Self {
        self.next_version = Some(next_version);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = Finalize {
            program: self.program.expect("program is not set"),
            authority: self.authority.expect("authority is not set"),
            next_version: self.next_version.expect("next_version is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `finalize` CPI accounts.
pub struct FinalizeCpiAccounts<'a, 'b> {
    /// Program account to finalize.
    pub program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Program authority.
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The next version of the program (can be itself).
    pub next_version: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `finalize` CPI instruction.
pub struct FinalizeCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Program account to finalize.
    pub program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Program authority.
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The next version of the program (can be itself).
    pub next_version: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> FinalizeCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: FinalizeCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            program: accounts.program,
            authority: accounts.authority,
            next_version: accounts.next_version,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.next_version.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = FinalizeInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LOADER_V4_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.program.clone());
        account_infos.push(self.authority.clone());
        account_infos.push(self.next_version.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `Finalize` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` program
///   1. `[signer]` authority
///   2. `[]` next_version
#[derive(Clone, Debug)]
pub struct FinalizeCpiBuilder<'a, 'b> {
    instruction: Box<FinalizeCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> FinalizeCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(FinalizeCpiBuilderInstruction {
            __program: program,
            program: None,
            authority: None,
            next_version: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Program account to finalize.
    #[inline(always)]
    pub fn program(
        &mut self,
        program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.program = Some(program);
        self
    }
    /// Program authority.
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }
    /// The next version of the program (can be itself).
    #[inline(always)]
    pub fn next_version(
        &mut self,
        next_version: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.next_version = Some(next_version);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool`
    /// indicating whether the account is writable or not, and a `bool`
    /// indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = FinalizeCpi {
            __program: self.instruction.__program,

            program: self.instruction.program.expect("program is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            next_version: self
                .instruction
                .next_version
                .expect("next_version is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct FinalizeCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    next_version: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
