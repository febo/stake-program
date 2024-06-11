/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  Address,
  Codec,
  Decoder,
  Encoder,
  IAccountMeta,
  IAccountSignerMeta,
  IInstruction,
  IInstructionWithAccounts,
  IInstructionWithData,
  ReadonlyAccount,
  ReadonlySignerAccount,
  TransactionSigner,
  WritableAccount,
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
} from '@solana/web3.js';
import { STAKE_PROGRAM_ADDRESS } from '../programs';
import { ResolvedAccount, getAccountMetaFactory } from '../shared';

export type WithdrawInactiveStakeInstruction<
  TProgram extends string = typeof STAKE_PROGRAM_ADDRESS,
  TAccountConfig extends string | IAccountMeta<string> = string,
  TAccountStake extends string | IAccountMeta<string> = string,
  TAccountStakeAuthority extends string | IAccountMeta<string> = string,
  TAccountVaultAuthority extends string | IAccountMeta<string> = string,
  TAccountVaultToken extends string | IAccountMeta<string> = string,
  TAccountDestinationToken extends string | IAccountMeta<string> = string,
  TAccountSplTokenProgram extends
    | string
    | IAccountMeta<string> = 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountConfig extends string
        ? WritableAccount<TAccountConfig>
        : TAccountConfig,
      TAccountStake extends string
        ? WritableAccount<TAccountStake>
        : TAccountStake,
      TAccountStakeAuthority extends string
        ? ReadonlySignerAccount<TAccountStakeAuthority> &
            IAccountSignerMeta<TAccountStakeAuthority>
        : TAccountStakeAuthority,
      TAccountVaultAuthority extends string
        ? ReadonlyAccount<TAccountVaultAuthority>
        : TAccountVaultAuthority,
      TAccountVaultToken extends string
        ? WritableAccount<TAccountVaultToken>
        : TAccountVaultToken,
      TAccountDestinationToken extends string
        ? WritableAccount<TAccountDestinationToken>
        : TAccountDestinationToken,
      TAccountSplTokenProgram extends string
        ? ReadonlyAccount<TAccountSplTokenProgram>
        : TAccountSplTokenProgram,
      ...TRemainingAccounts,
    ]
  >;

export type WithdrawInactiveStakeInstructionData = {
  discriminator: number;
  args: bigint;
};

export type WithdrawInactiveStakeInstructionDataArgs = {
  args: number | bigint;
};

export function getWithdrawInactiveStakeInstructionDataEncoder(): Encoder<WithdrawInactiveStakeInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['args', getU64Encoder()],
    ]),
    (value) => ({ ...value, discriminator: 5 })
  );
}

export function getWithdrawInactiveStakeInstructionDataDecoder(): Decoder<WithdrawInactiveStakeInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['args', getU64Decoder()],
  ]);
}

export function getWithdrawInactiveStakeInstructionDataCodec(): Codec<
  WithdrawInactiveStakeInstructionDataArgs,
  WithdrawInactiveStakeInstructionData
> {
  return combineCodec(
    getWithdrawInactiveStakeInstructionDataEncoder(),
    getWithdrawInactiveStakeInstructionDataDecoder()
  );
}

export type WithdrawInactiveStakeInput<
  TAccountConfig extends string = string,
  TAccountStake extends string = string,
  TAccountStakeAuthority extends string = string,
  TAccountVaultAuthority extends string = string,
  TAccountVaultToken extends string = string,
  TAccountDestinationToken extends string = string,
  TAccountSplTokenProgram extends string = string,
> = {
  /** Stake config account */
  config: Address<TAccountConfig>;
  /** Stake account */
  stake: Address<TAccountStake>;
  /** Stake authority */
  stakeAuthority: TransactionSigner<TAccountStakeAuthority>;
  /** Vault authority */
  vaultAuthority: Address<TAccountVaultAuthority>;
  /** Vault token account */
  vaultToken: Address<TAccountVaultToken>;
  /** Destination token account */
  destinationToken: Address<TAccountDestinationToken>;
  /** SPL Token program */
  splTokenProgram?: Address<TAccountSplTokenProgram>;
  args: WithdrawInactiveStakeInstructionDataArgs['args'];
};

export function getWithdrawInactiveStakeInstruction<
  TAccountConfig extends string,
  TAccountStake extends string,
  TAccountStakeAuthority extends string,
  TAccountVaultAuthority extends string,
  TAccountVaultToken extends string,
  TAccountDestinationToken extends string,
  TAccountSplTokenProgram extends string,
>(
  input: WithdrawInactiveStakeInput<
    TAccountConfig,
    TAccountStake,
    TAccountStakeAuthority,
    TAccountVaultAuthority,
    TAccountVaultToken,
    TAccountDestinationToken,
    TAccountSplTokenProgram
  >
): WithdrawInactiveStakeInstruction<
  typeof STAKE_PROGRAM_ADDRESS,
  TAccountConfig,
  TAccountStake,
  TAccountStakeAuthority,
  TAccountVaultAuthority,
  TAccountVaultToken,
  TAccountDestinationToken,
  TAccountSplTokenProgram
> {
  // Program address.
  const programAddress = STAKE_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    config: { value: input.config ?? null, isWritable: true },
    stake: { value: input.stake ?? null, isWritable: true },
    stakeAuthority: { value: input.stakeAuthority ?? null, isWritable: false },
    vaultAuthority: { value: input.vaultAuthority ?? null, isWritable: false },
    vaultToken: { value: input.vaultToken ?? null, isWritable: true },
    destinationToken: {
      value: input.destinationToken ?? null,
      isWritable: true,
    },
    splTokenProgram: {
      value: input.splTokenProgram ?? null,
      isWritable: false,
    },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  // Resolve default values.
  if (!accounts.splTokenProgram.value) {
    accounts.splTokenProgram.value =
      'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA' as Address<'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.config),
      getAccountMeta(accounts.stake),
      getAccountMeta(accounts.stakeAuthority),
      getAccountMeta(accounts.vaultAuthority),
      getAccountMeta(accounts.vaultToken),
      getAccountMeta(accounts.destinationToken),
      getAccountMeta(accounts.splTokenProgram),
    ],
    programAddress,
    data: getWithdrawInactiveStakeInstructionDataEncoder().encode(
      args as WithdrawInactiveStakeInstructionDataArgs
    ),
  } as WithdrawInactiveStakeInstruction<
    typeof STAKE_PROGRAM_ADDRESS,
    TAccountConfig,
    TAccountStake,
    TAccountStakeAuthority,
    TAccountVaultAuthority,
    TAccountVaultToken,
    TAccountDestinationToken,
    TAccountSplTokenProgram
  >;

  return instruction;
}

export type ParsedWithdrawInactiveStakeInstruction<
  TProgram extends string = typeof STAKE_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Stake config account */
    config: TAccountMetas[0];
    /** Stake account */
    stake: TAccountMetas[1];
    /** Stake authority */
    stakeAuthority: TAccountMetas[2];
    /** Vault authority */
    vaultAuthority: TAccountMetas[3];
    /** Vault token account */
    vaultToken: TAccountMetas[4];
    /** Destination token account */
    destinationToken: TAccountMetas[5];
    /** SPL Token program */
    splTokenProgram: TAccountMetas[6];
  };
  data: WithdrawInactiveStakeInstructionData;
};

export function parseWithdrawInactiveStakeInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedWithdrawInactiveStakeInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 7) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      config: getNextAccount(),
      stake: getNextAccount(),
      stakeAuthority: getNextAccount(),
      vaultAuthority: getNextAccount(),
      vaultToken: getNextAccount(),
      destinationToken: getNextAccount(),
      splTokenProgram: getNextAccount(),
    },
    data: getWithdrawInactiveStakeInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
