//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct WithdrawInactiveStake {
    /// Stake config account
    pub config: solana_program::pubkey::Pubkey,
    /// Stake account
    pub stake: solana_program::pubkey::Pubkey,
    /// Stake authority
    pub stake_authority: solana_program::pubkey::Pubkey,
    /// Vault authority
    pub vault_authority: solana_program::pubkey::Pubkey,
    /// Vault token account
    pub vault_token: solana_program::pubkey::Pubkey,
    /// Destination token account
    pub destination_token: solana_program::pubkey::Pubkey,
    /// SPL Token program
    pub spl_token_program: solana_program::pubkey::Pubkey,
}

impl WithdrawInactiveStake {
    pub fn instruction(
        &self,
        args: WithdrawInactiveStakeInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: WithdrawInactiveStakeInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.stake, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.stake_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.vault_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault_token,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.destination_token,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.spl_token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = WithdrawInactiveStakeInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::STAKE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct WithdrawInactiveStakeInstructionData {
    discriminator: u8,
}

impl WithdrawInactiveStakeInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 5 }
    }
}

impl Default for WithdrawInactiveStakeInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawInactiveStakeInstructionArgs {
    pub args: u64,
}

/// Instruction builder for `WithdrawInactiveStake`.
///
/// ### Accounts:
///
///   0. `[writable]` config
///   1. `[writable]` stake
///   2. `[signer]` stake_authority
///   3. `[]` vault_authority
///   4. `[writable]` vault_token
///   5. `[writable]` destination_token
///   6. `[optional]` spl_token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct WithdrawInactiveStakeBuilder {
    config: Option<solana_program::pubkey::Pubkey>,
    stake: Option<solana_program::pubkey::Pubkey>,
    stake_authority: Option<solana_program::pubkey::Pubkey>,
    vault_authority: Option<solana_program::pubkey::Pubkey>,
    vault_token: Option<solana_program::pubkey::Pubkey>,
    destination_token: Option<solana_program::pubkey::Pubkey>,
    spl_token_program: Option<solana_program::pubkey::Pubkey>,
    args: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl WithdrawInactiveStakeBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Stake config account
    #[inline(always)]
    pub fn config(&mut self, config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.config = Some(config);
        self
    }
    /// Stake account
    #[inline(always)]
    pub fn stake(&mut self, stake: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake = Some(stake);
        self
    }
    /// Stake authority
    #[inline(always)]
    pub fn stake_authority(
        &mut self,
        stake_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.stake_authority = Some(stake_authority);
        self
    }
    /// Vault authority
    #[inline(always)]
    pub fn vault_authority(
        &mut self,
        vault_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.vault_authority = Some(vault_authority);
        self
    }
    /// Vault token account
    #[inline(always)]
    pub fn vault_token(&mut self, vault_token: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vault_token = Some(vault_token);
        self
    }
    /// Destination token account
    #[inline(always)]
    pub fn destination_token(
        &mut self,
        destination_token: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.destination_token = Some(destination_token);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    /// SPL Token program
    #[inline(always)]
    pub fn spl_token_program(
        &mut self,
        spl_token_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.spl_token_program = Some(spl_token_program);
        self
    }
    #[inline(always)]
    pub fn args(&mut self, args: u64) -> &mut Self {
        self.args = Some(args);
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
        let accounts = WithdrawInactiveStake {
            config: self.config.expect("config is not set"),
            stake: self.stake.expect("stake is not set"),
            stake_authority: self.stake_authority.expect("stake_authority is not set"),
            vault_authority: self.vault_authority.expect("vault_authority is not set"),
            vault_token: self.vault_token.expect("vault_token is not set"),
            destination_token: self
                .destination_token
                .expect("destination_token is not set"),
            spl_token_program: self.spl_token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };
        let args = WithdrawInactiveStakeInstructionArgs {
            args: self.args.clone().expect("args is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `withdraw_inactive_stake` CPI accounts.
pub struct WithdrawInactiveStakeCpiAccounts<'a, 'b> {
    /// Stake config account
    pub config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake account
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake authority
    pub stake_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Vault authority
    pub vault_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Vault token account
    pub vault_token: &'b solana_program::account_info::AccountInfo<'a>,
    /// Destination token account
    pub destination_token: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL Token program
    pub spl_token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `withdraw_inactive_stake` CPI instruction.
pub struct WithdrawInactiveStakeCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake config account
    pub config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake account
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake authority
    pub stake_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Vault authority
    pub vault_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Vault token account
    pub vault_token: &'b solana_program::account_info::AccountInfo<'a>,
    /// Destination token account
    pub destination_token: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL Token program
    pub spl_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: WithdrawInactiveStakeInstructionArgs,
}

impl<'a, 'b> WithdrawInactiveStakeCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: WithdrawInactiveStakeCpiAccounts<'a, 'b>,
        args: WithdrawInactiveStakeInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            config: accounts.config,
            stake: accounts.stake,
            stake_authority: accounts.stake_authority,
            vault_authority: accounts.vault_authority,
            vault_token: accounts.vault_token,
            destination_token: accounts.destination_token,
            spl_token_program: accounts.spl_token_program,
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
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.stake.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.stake_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.vault_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault_token.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.destination_token.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.spl_token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = WithdrawInactiveStakeInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::STAKE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.config.clone());
        account_infos.push(self.stake.clone());
        account_infos.push(self.stake_authority.clone());
        account_infos.push(self.vault_authority.clone());
        account_infos.push(self.vault_token.clone());
        account_infos.push(self.destination_token.clone());
        account_infos.push(self.spl_token_program.clone());
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

/// Instruction builder for `WithdrawInactiveStake` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` config
///   1. `[writable]` stake
///   2. `[signer]` stake_authority
///   3. `[]` vault_authority
///   4. `[writable]` vault_token
///   5. `[writable]` destination_token
///   6. `[]` spl_token_program
#[derive(Clone, Debug)]
pub struct WithdrawInactiveStakeCpiBuilder<'a, 'b> {
    instruction: Box<WithdrawInactiveStakeCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> WithdrawInactiveStakeCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(WithdrawInactiveStakeCpiBuilderInstruction {
            __program: program,
            config: None,
            stake: None,
            stake_authority: None,
            vault_authority: None,
            vault_token: None,
            destination_token: None,
            spl_token_program: None,
            args: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Stake config account
    #[inline(always)]
    pub fn config(
        &mut self,
        config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.config = Some(config);
        self
    }
    /// Stake account
    #[inline(always)]
    pub fn stake(&mut self, stake: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.stake = Some(stake);
        self
    }
    /// Stake authority
    #[inline(always)]
    pub fn stake_authority(
        &mut self,
        stake_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_authority = Some(stake_authority);
        self
    }
    /// Vault authority
    #[inline(always)]
    pub fn vault_authority(
        &mut self,
        vault_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vault_authority = Some(vault_authority);
        self
    }
    /// Vault token account
    #[inline(always)]
    pub fn vault_token(
        &mut self,
        vault_token: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vault_token = Some(vault_token);
        self
    }
    /// Destination token account
    #[inline(always)]
    pub fn destination_token(
        &mut self,
        destination_token: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.destination_token = Some(destination_token);
        self
    }
    /// SPL Token program
    #[inline(always)]
    pub fn spl_token_program(
        &mut self,
        spl_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.spl_token_program = Some(spl_token_program);
        self
    }
    #[inline(always)]
    pub fn args(&mut self, args: u64) -> &mut Self {
        self.instruction.args = Some(args);
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
        let args = WithdrawInactiveStakeInstructionArgs {
            args: self.instruction.args.clone().expect("args is not set"),
        };
        let instruction = WithdrawInactiveStakeCpi {
            __program: self.instruction.__program,

            config: self.instruction.config.expect("config is not set"),

            stake: self.instruction.stake.expect("stake is not set"),

            stake_authority: self
                .instruction
                .stake_authority
                .expect("stake_authority is not set"),

            vault_authority: self
                .instruction
                .vault_authority
                .expect("vault_authority is not set"),

            vault_token: self
                .instruction
                .vault_token
                .expect("vault_token is not set"),

            destination_token: self
                .instruction
                .destination_token
                .expect("destination_token is not set"),

            spl_token_program: self
                .instruction
                .spl_token_program
                .expect("spl_token_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct WithdrawInactiveStakeCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault_token: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    destination_token: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    spl_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    args: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
