//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::DataV2;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

/// Accounts.
pub struct UpdateMetadataAccountV2 {
    /// Metadata account
    pub metadata: solana_program::pubkey::Pubkey,
    /// Update authority key
    pub update_authority: solana_program::pubkey::Pubkey,
}

impl UpdateMetadataAccountV2 {
    pub fn instruction(
        &self,
        args: UpdateMetadataAccountV2InstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: UpdateMetadataAccountV2InstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.metadata,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.update_authority,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = UpdateMetadataAccountV2InstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::MPL_TOKEN_METADATA_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct UpdateMetadataAccountV2InstructionData {
    discriminator: u8,
}

impl UpdateMetadataAccountV2InstructionData {
    fn new() -> Self {
        Self { discriminator: 15 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateMetadataAccountV2InstructionArgs {
    pub data: Option<DataV2>,
    pub new_update_authority: Option<Pubkey>,
    pub primary_sale_happened: Option<bool>,
    pub is_mutable: Option<bool>,
}

/// Instruction builder for `UpdateMetadataAccountV2`.
///
/// ### Accounts:
///
///   0. `[writable]` metadata
///   1. `[signer]` update_authority
#[derive(Default)]
pub struct UpdateMetadataAccountV2Builder {
    metadata: Option<solana_program::pubkey::Pubkey>,
    update_authority: Option<solana_program::pubkey::Pubkey>,
    data: Option<DataV2>,
    new_update_authority: Option<Pubkey>,
    primary_sale_happened: Option<bool>,
    is_mutable: Option<bool>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UpdateMetadataAccountV2Builder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Metadata account
    #[inline(always)]
    pub fn metadata(&mut self, metadata: solana_program::pubkey::Pubkey) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    /// Update authority key
    #[inline(always)]
    pub fn update_authority(
        &mut self,
        update_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.update_authority = Some(update_authority);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn data(&mut self, data: DataV2) -> &mut Self {
        self.data = Some(data);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn new_update_authority(&mut self, new_update_authority: Pubkey) -> &mut Self {
        self.new_update_authority = Some(new_update_authority);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn primary_sale_happened(&mut self, primary_sale_happened: bool) -> &mut Self {
        self.primary_sale_happened = Some(primary_sale_happened);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn is_mutable(&mut self, is_mutable: bool) -> &mut Self {
        self.is_mutable = Some(is_mutable);
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
        let accounts = UpdateMetadataAccountV2 {
            metadata: self.metadata.expect("metadata is not set"),
            update_authority: self.update_authority.expect("update_authority is not set"),
        };
        let args = UpdateMetadataAccountV2InstructionArgs {
            data: self.data.clone(),
            new_update_authority: self.new_update_authority.clone(),
            primary_sale_happened: self.primary_sale_happened.clone(),
            is_mutable: self.is_mutable.clone(),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `update_metadata_account_v2` CPI accounts.
pub struct UpdateMetadataAccountV2CpiAccounts<'a, 'b> {
    /// Metadata account
    pub metadata: &'b solana_program::account_info::AccountInfo<'a>,
    /// Update authority key
    pub update_authority: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `update_metadata_account_v2` CPI instruction.
pub struct UpdateMetadataAccountV2Cpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Metadata account
    pub metadata: &'b solana_program::account_info::AccountInfo<'a>,
    /// Update authority key
    pub update_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: UpdateMetadataAccountV2InstructionArgs,
}

impl<'a, 'b> UpdateMetadataAccountV2Cpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: UpdateMetadataAccountV2CpiAccounts<'a, 'b>,
        args: UpdateMetadataAccountV2InstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            metadata: accounts.metadata,
            update_authority: accounts.update_authority,
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
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.metadata.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.update_authority.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = UpdateMetadataAccountV2InstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MPL_TOKEN_METADATA_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(2 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.metadata.clone());
        account_infos.push(self.update_authority.clone());
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

/// Instruction builder for `UpdateMetadataAccountV2` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` metadata
///   1. `[signer]` update_authority
pub struct UpdateMetadataAccountV2CpiBuilder<'a, 'b> {
    instruction: Box<UpdateMetadataAccountV2CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UpdateMetadataAccountV2CpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UpdateMetadataAccountV2CpiBuilderInstruction {
            __program: program,
            metadata: None,
            update_authority: None,
            data: None,
            new_update_authority: None,
            primary_sale_happened: None,
            is_mutable: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Metadata account
    #[inline(always)]
    pub fn metadata(
        &mut self,
        metadata: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.metadata = Some(metadata);
        self
    }
    /// Update authority key
    #[inline(always)]
    pub fn update_authority(
        &mut self,
        update_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.update_authority = Some(update_authority);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn data(&mut self, data: DataV2) -> &mut Self {
        self.instruction.data = Some(data);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn new_update_authority(&mut self, new_update_authority: Pubkey) -> &mut Self {
        self.instruction.new_update_authority = Some(new_update_authority);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn primary_sale_happened(&mut self, primary_sale_happened: bool) -> &mut Self {
        self.instruction.primary_sale_happened = Some(primary_sale_happened);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn is_mutable(&mut self, is_mutable: bool) -> &mut Self {
        self.instruction.is_mutable = Some(is_mutable);
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
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
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
        let args = UpdateMetadataAccountV2InstructionArgs {
            data: self.instruction.data.clone(),
            new_update_authority: self.instruction.new_update_authority.clone(),
            primary_sale_happened: self.instruction.primary_sale_happened.clone(),
            is_mutable: self.instruction.is_mutable.clone(),
        };
        let instruction = UpdateMetadataAccountV2Cpi {
            __program: self.instruction.__program,

            metadata: self.instruction.metadata.expect("metadata is not set"),

            update_authority: self
                .instruction
                .update_authority
                .expect("update_authority is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct UpdateMetadataAccountV2CpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    metadata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    update_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    data: Option<DataV2>,
    new_update_authority: Option<Pubkey>,
    primary_sale_happened: Option<bool>,
    is_mutable: Option<bool>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}