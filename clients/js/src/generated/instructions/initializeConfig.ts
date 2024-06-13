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
  TransactionSigner,
  WritableSignerAccount,
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU16Decoder,
  getU16Encoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
} from '@solana/web3.js';
import { STAKE_PROGRAM_ADDRESS } from '../programs';
import { ResolvedAccount, getAccountMetaFactory } from '../shared';

export type InitializeConfigInstruction<
  TProgram extends string = typeof STAKE_PROGRAM_ADDRESS,
  TAccountConfig extends string | IAccountMeta<string> = string,
  TAccountConfigAuthority extends string | IAccountMeta<string> = string,
  TAccountSlashAuthority extends string | IAccountMeta<string> = string,
  TAccountMint extends string | IAccountMeta<string> = string,
  TAccountVaultToken extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountConfig extends string
        ? WritableSignerAccount<TAccountConfig> &
            IAccountSignerMeta<TAccountConfig>
        : TAccountConfig,
      TAccountConfigAuthority extends string
        ? ReadonlyAccount<TAccountConfigAuthority>
        : TAccountConfigAuthority,
      TAccountSlashAuthority extends string
        ? ReadonlyAccount<TAccountSlashAuthority>
        : TAccountSlashAuthority,
      TAccountMint extends string
        ? ReadonlyAccount<TAccountMint>
        : TAccountMint,
      TAccountVaultToken extends string
        ? ReadonlyAccount<TAccountVaultToken>
        : TAccountVaultToken,
      ...TRemainingAccounts,
    ]
  >;

export type InitializeConfigInstructionData = {
  discriminator: number;
  cooldownTimeSeconds: bigint;
  maxDeactivationBasisPoints: number;
};

export type InitializeConfigInstructionDataArgs = {
  cooldownTimeSeconds: number | bigint;
  maxDeactivationBasisPoints: number;
};

export function getInitializeConfigInstructionDataEncoder(): Encoder<InitializeConfigInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['cooldownTimeSeconds', getU64Encoder()],
      ['maxDeactivationBasisPoints', getU16Encoder()],
    ]),
    (value) => ({ ...value, discriminator: 0 })
  );
}

export function getInitializeConfigInstructionDataDecoder(): Decoder<InitializeConfigInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['cooldownTimeSeconds', getU64Decoder()],
    ['maxDeactivationBasisPoints', getU16Decoder()],
  ]);
}

export function getInitializeConfigInstructionDataCodec(): Codec<
  InitializeConfigInstructionDataArgs,
  InitializeConfigInstructionData
> {
  return combineCodec(
    getInitializeConfigInstructionDataEncoder(),
    getInitializeConfigInstructionDataDecoder()
  );
}

export type InitializeConfigInput<
  TAccountConfig extends string = string,
  TAccountConfigAuthority extends string = string,
  TAccountSlashAuthority extends string = string,
  TAccountMint extends string = string,
  TAccountVaultToken extends string = string,
> = {
  /** Stake config account */
  config: TransactionSigner<TAccountConfig>;
  /** Config authority */
  configAuthority: Address<TAccountConfigAuthority>;
  /** Slash authority */
  slashAuthority: Address<TAccountSlashAuthority>;
  /** Stake token mint */
  mint: Address<TAccountMint>;
  /** Stake token vault */
  vaultToken: Address<TAccountVaultToken>;
  cooldownTimeSeconds: InitializeConfigInstructionDataArgs['cooldownTimeSeconds'];
  maxDeactivationBasisPoints: InitializeConfigInstructionDataArgs['maxDeactivationBasisPoints'];
};

export function getInitializeConfigInstruction<
  TAccountConfig extends string,
  TAccountConfigAuthority extends string,
  TAccountSlashAuthority extends string,
  TAccountMint extends string,
  TAccountVaultToken extends string,
>(
  input: InitializeConfigInput<
    TAccountConfig,
    TAccountConfigAuthority,
    TAccountSlashAuthority,
    TAccountMint,
    TAccountVaultToken
  >
): InitializeConfigInstruction<
  typeof STAKE_PROGRAM_ADDRESS,
  TAccountConfig,
  TAccountConfigAuthority,
  TAccountSlashAuthority,
  TAccountMint,
  TAccountVaultToken
> {
  // Program address.
  const programAddress = STAKE_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    config: { value: input.config ?? null, isWritable: true },
    configAuthority: {
      value: input.configAuthority ?? null,
      isWritable: false,
    },
    slashAuthority: { value: input.slashAuthority ?? null, isWritable: false },
    mint: { value: input.mint ?? null, isWritable: false },
    vaultToken: { value: input.vaultToken ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.config),
      getAccountMeta(accounts.configAuthority),
      getAccountMeta(accounts.slashAuthority),
      getAccountMeta(accounts.mint),
      getAccountMeta(accounts.vaultToken),
    ],
    programAddress,
    data: getInitializeConfigInstructionDataEncoder().encode(
      args as InitializeConfigInstructionDataArgs
    ),
  } as InitializeConfigInstruction<
    typeof STAKE_PROGRAM_ADDRESS,
    TAccountConfig,
    TAccountConfigAuthority,
    TAccountSlashAuthority,
    TAccountMint,
    TAccountVaultToken
  >;

  return instruction;
}

export type ParsedInitializeConfigInstruction<
  TProgram extends string = typeof STAKE_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Stake config account */
    config: TAccountMetas[0];
    /** Config authority */
    configAuthority: TAccountMetas[1];
    /** Slash authority */
    slashAuthority: TAccountMetas[2];
    /** Stake token mint */
    mint: TAccountMetas[3];
    /** Stake token vault */
    vaultToken: TAccountMetas[4];
  };
  data: InitializeConfigInstructionData;
};

export function parseInitializeConfigInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedInitializeConfigInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 5) {
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
      configAuthority: getNextAccount(),
      slashAuthority: getNextAccount(),
      mint: getNextAccount(),
      vaultToken: getNextAccount(),
    },
    data: getInitializeConfigInstructionDataDecoder().decode(instruction.data),
  };
}