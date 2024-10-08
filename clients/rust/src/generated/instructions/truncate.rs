//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct Truncate {
    /// Program account to change the size of.
    pub program: solana_program::pubkey::Pubkey,
    /// Program authority.
    pub authority: solana_program::pubkey::Pubkey,
    /// Destination account for reclaimed lamports (optional).
    pub destination: Option<solana_program::pubkey::Pubkey>,
}

impl Truncate {
    pub fn instruction(
        &self,
        args: TruncateInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: TruncateInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.program,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        if let Some(destination) = self.destination {
            accounts.push(solana_program::instruction::AccountMeta::new(
                destination,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::LOADER_V4_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let mut data = TruncateInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::LOADER_V4_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TruncateInstructionData {
    discriminator: u8,
}

impl TruncateInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 1 }
    }
}

impl Default for TruncateInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TruncateInstructionArgs {
    pub new_size: u32,
}

/// Instruction builder for `Truncate`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` program
///   1. `[signer]` authority
///   2. `[writable, optional]` destination
#[derive(Clone, Debug, Default)]
pub struct TruncateBuilder {
    program: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    destination: Option<solana_program::pubkey::Pubkey>,
    new_size: Option<u32>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl TruncateBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Program account to change the size of.
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
    /// `[optional account]`
    /// Destination account for reclaimed lamports (optional).
    #[inline(always)]
    pub fn destination(
        &mut self,
        destination: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.destination = destination;
        self
    }
    #[inline(always)]
    pub fn new_size(&mut self, new_size: u32) -> &mut Self {
        self.new_size = Some(new_size);
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
        let accounts = Truncate {
            program: self.program.expect("program is not set"),
            authority: self.authority.expect("authority is not set"),
            destination: self.destination,
        };
        let args = TruncateInstructionArgs {
            new_size: self.new_size.clone().expect("new_size is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `truncate` CPI accounts.
pub struct TruncateCpiAccounts<'a, 'b> {
    /// Program account to change the size of.
    pub program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Program authority.
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Destination account for reclaimed lamports (optional).
    pub destination: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `truncate` CPI instruction.
pub struct TruncateCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Program account to change the size of.
    pub program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Program authority.
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Destination account for reclaimed lamports (optional).
    pub destination: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The arguments for the instruction.
    pub __args: TruncateInstructionArgs,
}

impl<'a, 'b> TruncateCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: TruncateCpiAccounts<'a, 'b>,
        args: TruncateInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            program: accounts.program,
            authority: accounts.authority,
            destination: accounts.destination,
            __args: args,
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
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        if let Some(destination) = self.destination {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *destination.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::LOADER_V4_ID,
                false,
            ));
        }
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = TruncateInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LOADER_V4_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.program.clone());
        account_infos.push(self.authority.clone());
        if let Some(destination) = self.destination {
            account_infos.push(destination.clone());
        }
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

/// Instruction builder for `Truncate` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` program
///   1. `[signer]` authority
///   2. `[writable, optional]` destination
#[derive(Clone, Debug)]
pub struct TruncateCpiBuilder<'a, 'b> {
    instruction: Box<TruncateCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> TruncateCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(TruncateCpiBuilderInstruction {
            __program: program,
            program: None,
            authority: None,
            destination: None,
            new_size: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Program account to change the size of.
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
    /// `[optional account]`
    /// Destination account for reclaimed lamports (optional).
    #[inline(always)]
    pub fn destination(
        &mut self,
        destination: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.destination = destination;
        self
    }
    #[inline(always)]
    pub fn new_size(&mut self, new_size: u32) -> &mut Self {
        self.instruction.new_size = Some(new_size);
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
        let args = TruncateInstructionArgs {
            new_size: self
                .instruction
                .new_size
                .clone()
                .expect("new_size is not set"),
        };
        let instruction = TruncateCpi {
            __program: self.instruction.__program,

            program: self.instruction.program.expect("program is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            destination: self.instruction.destination,
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct TruncateCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    destination: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    new_size: Option<u32>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
