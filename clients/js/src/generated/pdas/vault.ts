/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  Address,
  ProgramDerivedAddress,
  getAddressEncoder,
  getProgramDerivedAddress,
  getUtf8Encoder,
} from '@solana/web3.js';

export type VaultSeeds = {
  /** Config authority */
  configAuthority: Address;
};

export async function findVaultPda(
  seeds: VaultSeeds,
  config: { programAddress?: Address | undefined } = {}
): Promise<ProgramDerivedAddress> {
  const {
    programAddress = 'PStake1111111111111111111111111111111111111' as Address<'PStake1111111111111111111111111111111111111'>,
  } = config;
  return await getProgramDerivedAddress({
    programAddress,
    seeds: [
      getUtf8Encoder().encode('token-vault'),
      getAddressEncoder().encode(seeds.configAuthority),
    ],
  });
}