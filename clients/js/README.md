# JavaScript client

A generated JavaScript library for the Solana Loader V4 program.

## Getting started

To build and test your JavaScript client from the root of the repository, you may use the following command. It starts a local validator, runs the test suite, then stops the validator.

```sh
make test-js-clients-js
```

## Available client scripts.

Alternatively, you can start a validator and run the tests in the client directory directly.

```sh
# Start the validator.
make restart-test-validator

# Go into the client directory and run the tests.
cd clients/js
pnpm install
pnpm build
pnpm test
```

You may also use the following scripts to lint and/or format your JavaScript client.

```sh
pnpm lint
pnpm lint:fix
pnpm format
pnpm format:fix
```
