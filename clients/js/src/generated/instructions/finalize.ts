/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type ReadonlySignerAccount,
  type TransactionSigner,
  type WritableAccount,
} from '@solana/web3.js';
import { LOADER_V4_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const FINALIZE_DISCRIMINATOR = 5;

export function getFinalizeDiscriminatorBytes() {
  return getU8Encoder().encode(FINALIZE_DISCRIMINATOR);
}

export type FinalizeInstruction<
  TProgram extends string = typeof LOADER_V4_PROGRAM_ADDRESS,
  TAccountProgram extends string | IAccountMeta<string> = string,
  TAccountAuthority extends string | IAccountMeta<string> = string,
  TAccountNextVersion extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountProgram extends string
        ? WritableAccount<TAccountProgram>
        : TAccountProgram,
      TAccountAuthority extends string
        ? ReadonlySignerAccount<TAccountAuthority> &
            IAccountSignerMeta<TAccountAuthority>
        : TAccountAuthority,
      TAccountNextVersion extends string
        ? ReadonlyAccount<TAccountNextVersion>
        : TAccountNextVersion,
      ...TRemainingAccounts,
    ]
  >;

export type FinalizeInstructionData = { discriminator: number };

export type FinalizeInstructionDataArgs = {};

export function getFinalizeInstructionDataEncoder(): Encoder<FinalizeInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({ ...value, discriminator: FINALIZE_DISCRIMINATOR })
  );
}

export function getFinalizeInstructionDataDecoder(): Decoder<FinalizeInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getFinalizeInstructionDataCodec(): Codec<
  FinalizeInstructionDataArgs,
  FinalizeInstructionData
> {
  return combineCodec(
    getFinalizeInstructionDataEncoder(),
    getFinalizeInstructionDataDecoder()
  );
}

export type FinalizeInput<
  TAccountProgram extends string = string,
  TAccountAuthority extends string = string,
  TAccountNextVersion extends string = string,
> = {
  /** Program account to finalize. */
  program: Address<TAccountProgram>;
  /** Program authority. */
  authority: TransactionSigner<TAccountAuthority>;
  /** The next version of the program (can be itself). */
  nextVersion: Address<TAccountNextVersion>;
};

export function getFinalizeInstruction<
  TAccountProgram extends string,
  TAccountAuthority extends string,
  TAccountNextVersion extends string,
>(
  input: FinalizeInput<TAccountProgram, TAccountAuthority, TAccountNextVersion>
): FinalizeInstruction<
  typeof LOADER_V4_PROGRAM_ADDRESS,
  TAccountProgram,
  TAccountAuthority,
  TAccountNextVersion
> {
  // Program address.
  const programAddress = LOADER_V4_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    program: { value: input.program ?? null, isWritable: true },
    authority: { value: input.authority ?? null, isWritable: false },
    nextVersion: { value: input.nextVersion ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.program),
      getAccountMeta(accounts.authority),
      getAccountMeta(accounts.nextVersion),
    ],
    programAddress,
    data: getFinalizeInstructionDataEncoder().encode({}),
  } as FinalizeInstruction<
    typeof LOADER_V4_PROGRAM_ADDRESS,
    TAccountProgram,
    TAccountAuthority,
    TAccountNextVersion
  >;

  return instruction;
}

export type ParsedFinalizeInstruction<
  TProgram extends string = typeof LOADER_V4_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Program account to finalize. */
    program: TAccountMetas[0];
    /** Program authority. */
    authority: TAccountMetas[1];
    /** The next version of the program (can be itself). */
    nextVersion: TAccountMetas[2];
  };
  data: FinalizeInstructionData;
};

export function parseFinalizeInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedFinalizeInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 3) {
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
      program: getNextAccount(),
      authority: getNextAccount(),
      nextVersion: getNextAccount(),
    },
    data: getFinalizeInstructionDataDecoder().decode(instruction.data),
  };
}
