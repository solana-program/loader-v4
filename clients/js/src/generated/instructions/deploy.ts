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
  type ReadonlySignerAccount,
  type TransactionSigner,
  type WritableAccount,
} from '@solana/web3.js';
import { LOADER_V4_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const DEPLOY_DISCRIMINATOR = 2;

export function getDeployDiscriminatorBytes() {
  return getU8Encoder().encode(DEPLOY_DISCRIMINATOR);
}

export type DeployInstruction<
  TProgram extends string = typeof LOADER_V4_PROGRAM_ADDRESS,
  TAccountProgram extends string | IAccountMeta<string> = string,
  TAccountAuthority extends string | IAccountMeta<string> = string,
  TAccountSource extends string | IAccountMeta<string> = string,
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
      TAccountSource extends string
        ? WritableAccount<TAccountSource>
        : TAccountSource,
      ...TRemainingAccounts,
    ]
  >;

export type DeployInstructionData = { discriminator: number };

export type DeployInstructionDataArgs = {};

export function getDeployInstructionDataEncoder(): Encoder<DeployInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({ ...value, discriminator: DEPLOY_DISCRIMINATOR })
  );
}

export function getDeployInstructionDataDecoder(): Decoder<DeployInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getDeployInstructionDataCodec(): Codec<
  DeployInstructionDataArgs,
  DeployInstructionData
> {
  return combineCodec(
    getDeployInstructionDataEncoder(),
    getDeployInstructionDataDecoder()
  );
}

export type DeployInput<
  TAccountProgram extends string = string,
  TAccountAuthority extends string = string,
  TAccountSource extends string = string,
> = {
  /** Program account to deploy. */
  program: Address<TAccountProgram>;
  /** Program authority. */
  authority: TransactionSigner<TAccountAuthority>;
  /** Undeployed source program account to take data and lamports from (optional). */
  source?: Address<TAccountSource>;
};

export function getDeployInstruction<
  TAccountProgram extends string,
  TAccountAuthority extends string,
  TAccountSource extends string,
>(
  input: DeployInput<TAccountProgram, TAccountAuthority, TAccountSource>
): DeployInstruction<
  typeof LOADER_V4_PROGRAM_ADDRESS,
  TAccountProgram,
  TAccountAuthority,
  TAccountSource
> {
  // Program address.
  const programAddress = LOADER_V4_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    program: { value: input.program ?? null, isWritable: true },
    authority: { value: input.authority ?? null, isWritable: false },
    source: { value: input.source ?? null, isWritable: true },
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
      getAccountMeta(accounts.source),
    ],
    programAddress,
    data: getDeployInstructionDataEncoder().encode({}),
  } as DeployInstruction<
    typeof LOADER_V4_PROGRAM_ADDRESS,
    TAccountProgram,
    TAccountAuthority,
    TAccountSource
  >;

  return instruction;
}

export type ParsedDeployInstruction<
  TProgram extends string = typeof LOADER_V4_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Program account to deploy. */
    program: TAccountMetas[0];
    /** Program authority. */
    authority: TAccountMetas[1];
    /** Undeployed source program account to take data and lamports from (optional). */
    source?: TAccountMetas[2] | undefined;
  };
  data: DeployInstructionData;
};

export function parseDeployInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedDeployInstruction<TProgram, TAccountMetas> {
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
  const getNextOptionalAccount = () => {
    const accountMeta = getNextAccount();
    return accountMeta.address === LOADER_V4_PROGRAM_ADDRESS
      ? undefined
      : accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      program: getNextAccount(),
      authority: getNextAccount(),
      source: getNextOptionalAccount(),
    },
    data: getDeployInstructionDataDecoder().decode(instruction.data),
  };
}
