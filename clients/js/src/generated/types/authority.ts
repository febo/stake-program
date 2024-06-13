/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  Codec,
  Decoder,
  Encoder,
  combineCodec,
  getEnumDecoder,
  getEnumEncoder,
} from '@solana/web3.js';

export enum Authority {
  Config,
  Slash,
  Stake,
}

export type AuthorityArgs = Authority;

export function getAuthorityEncoder(): Encoder<AuthorityArgs> {
  return getEnumEncoder(Authority);
}

export function getAuthorityDecoder(): Decoder<Authority> {
  return getEnumDecoder(Authority);
}

export function getAuthorityCodec(): Codec<AuthorityArgs, Authority> {
  return combineCodec(getAuthorityEncoder(), getAuthorityDecoder());
}