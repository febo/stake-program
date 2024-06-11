//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct HarvestHolderRewards {
    /// Stake config account
    pub config: solana_program::pubkey::Pubkey,
    /// Stake account
    pub stake: solana_program::pubkey::Pubkey,
    /// Vault token account
    pub vault_token: solana_program::pubkey::Pubkey,
    /// Holder rewards account for vault token account
    pub holder_rewards: solana_program::pubkey::Pubkey,
    /// Destination account for withdrawn lamports
    pub destination: solana_program::pubkey::Pubkey,
    /// Stake authority
    pub authority: solana_program::pubkey::Pubkey,
    /// Vault authority
    pub vault_authority: solana_program::pubkey::Pubkey,
    /// Stake token mint
    pub mint: solana_program::pubkey::Pubkey,
    /// SPL Token program
    pub spl_token_program: solana_program::pubkey::Pubkey,
}

impl HarvestHolderRewards {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.stake, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault_token,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.holder_rewards,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.destination,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.vault_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.mint, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.spl_token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = HarvestHolderRewardsInstructionData::new()
            .try_to_vec()
            .unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::STAKE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct HarvestHolderRewardsInstructionData {
    discriminator: u8,
}

impl HarvestHolderRewardsInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 6 }
    }
}

impl Default for HarvestHolderRewardsInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `HarvestHolderRewards`.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` stake
///   2. `[writable]` vault_token
///   3. `[]` holder_rewards
///   4. `[writable]` destination
///   5. `[signer]` authority
///   6. `[signer]` vault_authority
///   7. `[signer]` mint
///   8. `[optional]` spl_token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct HarvestHolderRewardsBuilder {
    config: Option<solana_program::pubkey::Pubkey>,
    stake: Option<solana_program::pubkey::Pubkey>,
    vault_token: Option<solana_program::pubkey::Pubkey>,
    holder_rewards: Option<solana_program::pubkey::Pubkey>,
    destination: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    vault_authority: Option<solana_program::pubkey::Pubkey>,
    mint: Option<solana_program::pubkey::Pubkey>,
    spl_token_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl HarvestHolderRewardsBuilder {
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
    /// Vault token account
    #[inline(always)]
    pub fn vault_token(&mut self, vault_token: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vault_token = Some(vault_token);
        self
    }
    /// Holder rewards account for vault token account
    #[inline(always)]
    pub fn holder_rewards(&mut self, holder_rewards: solana_program::pubkey::Pubkey) -> &mut Self {
        self.holder_rewards = Some(holder_rewards);
        self
    }
    /// Destination account for withdrawn lamports
    #[inline(always)]
    pub fn destination(&mut self, destination: solana_program::pubkey::Pubkey) -> &mut Self {
        self.destination = Some(destination);
        self
    }
    /// Stake authority
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
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
    /// Stake token mint
    #[inline(always)]
    pub fn mint(&mut self, mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
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
        let accounts = HarvestHolderRewards {
            config: self.config.expect("config is not set"),
            stake: self.stake.expect("stake is not set"),
            vault_token: self.vault_token.expect("vault_token is not set"),
            holder_rewards: self.holder_rewards.expect("holder_rewards is not set"),
            destination: self.destination.expect("destination is not set"),
            authority: self.authority.expect("authority is not set"),
            vault_authority: self.vault_authority.expect("vault_authority is not set"),
            mint: self.mint.expect("mint is not set"),
            spl_token_program: self.spl_token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `harvest_holder_rewards` CPI accounts.
pub struct HarvestHolderRewardsCpiAccounts<'a, 'b> {
    /// Stake config account
    pub config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake account
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Vault token account
    pub vault_token: &'b solana_program::account_info::AccountInfo<'a>,
    /// Holder rewards account for vault token account
    pub holder_rewards: &'b solana_program::account_info::AccountInfo<'a>,
    /// Destination account for withdrawn lamports
    pub destination: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake authority
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Vault authority
    pub vault_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake token mint
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL Token program
    pub spl_token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `harvest_holder_rewards` CPI instruction.
pub struct HarvestHolderRewardsCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake config account
    pub config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake account
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Vault token account
    pub vault_token: &'b solana_program::account_info::AccountInfo<'a>,
    /// Holder rewards account for vault token account
    pub holder_rewards: &'b solana_program::account_info::AccountInfo<'a>,
    /// Destination account for withdrawn lamports
    pub destination: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake authority
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Vault authority
    pub vault_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake token mint
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL Token program
    pub spl_token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> HarvestHolderRewardsCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: HarvestHolderRewardsCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            config: accounts.config,
            stake: accounts.stake,
            vault_token: accounts.vault_token,
            holder_rewards: accounts.holder_rewards,
            destination: accounts.destination,
            authority: accounts.authority,
            vault_authority: accounts.vault_authority,
            mint: accounts.mint,
            spl_token_program: accounts.spl_token_program,
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
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.stake.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault_token.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.holder_rewards.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.destination.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.vault_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.mint.key,
            true,
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
        let data = HarvestHolderRewardsInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::STAKE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(9 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.config.clone());
        account_infos.push(self.stake.clone());
        account_infos.push(self.vault_token.clone());
        account_infos.push(self.holder_rewards.clone());
        account_infos.push(self.destination.clone());
        account_infos.push(self.authority.clone());
        account_infos.push(self.vault_authority.clone());
        account_infos.push(self.mint.clone());
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

/// Instruction builder for `HarvestHolderRewards` via CPI.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` stake
///   2. `[writable]` vault_token
///   3. `[]` holder_rewards
///   4. `[writable]` destination
///   5. `[signer]` authority
///   6. `[signer]` vault_authority
///   7. `[signer]` mint
///   8. `[]` spl_token_program
#[derive(Clone, Debug)]
pub struct HarvestHolderRewardsCpiBuilder<'a, 'b> {
    instruction: Box<HarvestHolderRewardsCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> HarvestHolderRewardsCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(HarvestHolderRewardsCpiBuilderInstruction {
            __program: program,
            config: None,
            stake: None,
            vault_token: None,
            holder_rewards: None,
            destination: None,
            authority: None,
            vault_authority: None,
            mint: None,
            spl_token_program: None,
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
    /// Vault token account
    #[inline(always)]
    pub fn vault_token(
        &mut self,
        vault_token: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vault_token = Some(vault_token);
        self
    }
    /// Holder rewards account for vault token account
    #[inline(always)]
    pub fn holder_rewards(
        &mut self,
        holder_rewards: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.holder_rewards = Some(holder_rewards);
        self
    }
    /// Destination account for withdrawn lamports
    #[inline(always)]
    pub fn destination(
        &mut self,
        destination: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.destination = Some(destination);
        self
    }
    /// Stake authority
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
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
    /// Stake token mint
    #[inline(always)]
    pub fn mint(&mut self, mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
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
        let instruction = HarvestHolderRewardsCpi {
            __program: self.instruction.__program,

            config: self.instruction.config.expect("config is not set"),

            stake: self.instruction.stake.expect("stake is not set"),

            vault_token: self
                .instruction
                .vault_token
                .expect("vault_token is not set"),

            holder_rewards: self
                .instruction
                .holder_rewards
                .expect("holder_rewards is not set"),

            destination: self
                .instruction
                .destination
                .expect("destination is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            vault_authority: self
                .instruction
                .vault_authority
                .expect("vault_authority is not set"),

            mint: self.instruction.mint.expect("mint is not set"),

            spl_token_program: self
                .instruction
                .spl_token_program
                .expect("spl_token_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct HarvestHolderRewardsCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault_token: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    holder_rewards: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    destination: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    spl_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
