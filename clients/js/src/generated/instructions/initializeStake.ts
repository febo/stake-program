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
  IInstruction,
  IInstructionWithAccounts,
  IInstructionWithData,
  ReadonlyAccount,
  WritableAccount,
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
} from '@solana/web3.js';
import { STAKE_PROGRAM_ADDRESS } from '../programs';
import { ResolvedAccount, getAccountMetaFactory } from '../shared';

export type InitializeStakeInstruction<
  TProgram extends string = typeof STAKE_PROGRAM_ADDRESS,
  TAccountConfig extends string | IAccountMeta<string> = string,
  TAccountStake extends string | IAccountMeta<string> = string,
  TAccountValidatorVote extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountConfig extends string
        ? ReadonlyAccount<TAccountConfig>
        : TAccountConfig,
      TAccountStake extends string
        ? WritableAccount<TAccountStake>
        : TAccountStake,
      TAccountValidatorVote extends string
        ? ReadonlyAccount<TAccountValidatorVote>
        : TAccountValidatorVote,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type InitializeStakeInstructionData = { discriminator: number };

export type InitializeStakeInstructionDataArgs = {};

export function getInitializeStakeInstructionDataEncoder(): Encoder<InitializeStakeInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({ ...value, discriminator: 1 })
  );
}

export function getInitializeStakeInstructionDataDecoder(): Decoder<InitializeStakeInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getInitializeStakeInstructionDataCodec(): Codec<
  InitializeStakeInstructionDataArgs,
  InitializeStakeInstructionData
> {
  return combineCodec(
    getInitializeStakeInstructionDataEncoder(),
    getInitializeStakeInstructionDataDecoder()
  );
}

export type InitializeStakeInput<
  TAccountConfig extends string = string,
  TAccountStake extends string = string,
  TAccountValidatorVote extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  /** Stake config account */
  config: Address<TAccountConfig>;
  /** Validator stake account (pda of `['stake', config, validator]`) */
  stake: Address<TAccountStake>;
  /** Validator vote account */
  validatorVote: Address<TAccountValidatorVote>;
  /** System program account */
  systemProgram?: Address<TAccountSystemProgram>;
};

export function getInitializeStakeInstruction<
  TAccountConfig extends string,
  TAccountStake extends string,
  TAccountValidatorVote extends string,
  TAccountSystemProgram extends string,
>(
  input: InitializeStakeInput<
    TAccountConfig,
    TAccountStake,
    TAccountValidatorVote,
    TAccountSystemProgram
  >
): InitializeStakeInstruction<
  typeof STAKE_PROGRAM_ADDRESS,
  TAccountConfig,
  TAccountStake,
  TAccountValidatorVote,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress = STAKE_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    config: { value: input.config ?? null, isWritable: false },
    stake: { value: input.stake ?? null, isWritable: true },
    validatorVote: { value: input.validatorVote ?? null, isWritable: false },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.config),
      getAccountMeta(accounts.stake),
      getAccountMeta(accounts.validatorVote),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getInitializeStakeInstructionDataEncoder().encode({}),
  } as InitializeStakeInstruction<
    typeof STAKE_PROGRAM_ADDRESS,
    TAccountConfig,
    TAccountStake,
    TAccountValidatorVote,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedInitializeStakeInstruction<
  TProgram extends string = typeof STAKE_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Stake config account */
    config: TAccountMetas[0];
    /** Validator stake account (pda of `['stake', config, validator]`) */
    stake: TAccountMetas[1];
    /** Validator vote account */
    validatorVote: TAccountMetas[2];
    /** System program account */
    systemProgram: TAccountMetas[3];
  };
  data: InitializeStakeInstructionData;
};

export function parseInitializeStakeInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedInitializeStakeInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 4) {
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
      validatorVote: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getInitializeStakeInstructionDataDecoder().decode(instruction.data),
  };
}