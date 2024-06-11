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

export type InactivateStakeInstruction<
  TProgram extends string = typeof STAKE_PROGRAM_ADDRESS,
  TAccountConfig extends string | IAccountMeta<string> = string,
  TAccountStake extends string | IAccountMeta<string> = string,
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
      ...TRemainingAccounts,
    ]
  >;

export type InactivateStakeInstructionData = { discriminator: number };

export type InactivateStakeInstructionDataArgs = {};

export function getInactivateStakeInstructionDataEncoder(): Encoder<InactivateStakeInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({ ...value, discriminator: 4 })
  );
}

export function getInactivateStakeInstructionDataDecoder(): Decoder<InactivateStakeInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getInactivateStakeInstructionDataCodec(): Codec<
  InactivateStakeInstructionDataArgs,
  InactivateStakeInstructionData
> {
  return combineCodec(
    getInactivateStakeInstructionDataEncoder(),
    getInactivateStakeInstructionDataDecoder()
  );
}

export type InactivateStakeInput<
  TAccountConfig extends string = string,
  TAccountStake extends string = string,
> = {
  /** Stake config account */
  config: Address<TAccountConfig>;
  /** Validator stake account */
  stake: Address<TAccountStake>;
};

export function getInactivateStakeInstruction<
  TAccountConfig extends string,
  TAccountStake extends string,
>(
  input: InactivateStakeInput<TAccountConfig, TAccountStake>
): InactivateStakeInstruction<
  typeof STAKE_PROGRAM_ADDRESS,
  TAccountConfig,
  TAccountStake
> {
  // Program address.
  const programAddress = STAKE_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    config: { value: input.config ?? null, isWritable: true },
    stake: { value: input.stake ?? null, isWritable: true },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [getAccountMeta(accounts.config), getAccountMeta(accounts.stake)],
    programAddress,
    data: getInactivateStakeInstructionDataEncoder().encode({}),
  } as InactivateStakeInstruction<
    typeof STAKE_PROGRAM_ADDRESS,
    TAccountConfig,
    TAccountStake
  >;

  return instruction;
}

export type ParsedInactivateStakeInstruction<
  TProgram extends string = typeof STAKE_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Stake config account */
    config: TAccountMetas[0];
    /** Validator stake account */
    stake: TAccountMetas[1];
  };
  data: InactivateStakeInstructionData;
};

export function parseInactivateStakeInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedInactivateStakeInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 2) {
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
    },
    data: getInactivateStakeInstructionDataDecoder().decode(instruction.data),
  };
}
