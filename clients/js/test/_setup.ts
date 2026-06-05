import { createClient, lamports } from '@solana/kit';
import { solanaLocalRpc } from '@solana/kit-plugin-rpc';
import { airdropSigner, generatedSigner } from '@solana/kit-plugin-signer';

import { loaderV4Program } from '../src';

// The loader-v4 program (`CoreBPFLoaderV4`) is a native runtime program that is
// not available in LiteSVM, so its tests run against a local validator via the
// RPC plugin. Start one with `pnpm validator:restart` before running the tests.
export const createTestClient = () => {
  return createClient()
    .use(generatedSigner())
    .use(solanaLocalRpc())
    .use(airdropSigner(lamports(1_000_000_000n)))
    .use(loaderV4Program());
};
