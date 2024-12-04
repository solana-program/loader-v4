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
  getU32Decoder,
  getU32Encoder,
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
  type ReadonlySignerAccount,
  type TransactionSigner,
  type WritableAccount,
} from '@solana/web3.js';
import { LOADER_V4_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const RETRACT_DISCRIMINATOR = 3;

export function getRetractDiscriminatorBytes() {
  return getU32Encoder().encode(RETRACT_DISCRIMINATOR);
}

export type RetractInstruction<
  TProgram extends string = typeof LOADER_V4_PROGRAM_ADDRESS,
  TAccountProgram extends string | IAccountMeta<string> = string,
  TAccountAuthority extends string | IAccountMeta<string> = string,
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
      ...TRemainingAccounts,
    ]
  >;

export type RetractInstructionData = { discriminator: number };

export type RetractInstructionDataArgs = {};

export function getRetractInstructionDataEncoder(): Encoder<RetractInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU32Encoder()]]),
    (value) => ({ ...value, discriminator: RETRACT_DISCRIMINATOR })
  );
}

export function getRetractInstructionDataDecoder(): Decoder<RetractInstructionData> {
  return getStructDecoder([['discriminator', getU32Decoder()]]);
}

export function getRetractInstructionDataCodec(): Codec<
  RetractInstructionDataArgs,
  RetractInstructionData
> {
  return combineCodec(
    getRetractInstructionDataEncoder(),
    getRetractInstructionDataDecoder()
  );
}

export type RetractInput<
  TAccountProgram extends string = string,
  TAccountAuthority extends string = string,
> = {
  /** Program account to retract. */
  program: Address<TAccountProgram>;
  /** Program authority. */
  authority: TransactionSigner<TAccountAuthority>;
};

export function getRetractInstruction<
  TAccountProgram extends string,
  TAccountAuthority extends string,
>(
  input: RetractInput<TAccountProgram, TAccountAuthority>
): RetractInstruction<
  typeof LOADER_V4_PROGRAM_ADDRESS,
  TAccountProgram,
  TAccountAuthority
> {
  // Program address.
  const programAddress = LOADER_V4_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    program: { value: input.program ?? null, isWritable: true },
    authority: { value: input.authority ?? null, isWritable: false },
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
    ],
    programAddress,
    data: getRetractInstructionDataEncoder().encode({}),
  } as RetractInstruction<
    typeof LOADER_V4_PROGRAM_ADDRESS,
    TAccountProgram,
    TAccountAuthority
  >;

  return instruction;
}

export type ParsedRetractInstruction<
  TProgram extends string = typeof LOADER_V4_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Program account to retract. */
    program: TAccountMetas[0];
    /** Program authority. */
    authority: TAccountMetas[1];
  };
  data: RetractInstructionData;
};

export function parseRetractInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedRetractInstruction<TProgram, TAccountMetas> {
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
      program: getNextAccount(),
      authority: getNextAccount(),
    },
    data: getRetractInstructionDataDecoder().decode(instruction.data),
  };
}
