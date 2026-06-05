import { expect, it } from 'vitest';

import { createTestClient } from './_setup';

it('sets up a Kit client with the loader-v4 program', async () => {
  // Given a test client whose payer is funded with SOL.
  const client = await createTestClient();

  // Then the client exposes the loader-v4 program plugin.
  expect(client.loaderV4).toBeDefined();

  // And the payer was funded by the local validator.
  const { value: balance } = await client.rpc
    .getBalance(client.payer.address)
    .send();
  expect(balance).toBeGreaterThan(0n);
});
