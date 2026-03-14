# Project Roadmap

The long-term goal of this project is to provide a complete toolkit for interacting with PVM cryptographic precompiles.

## Delivered

- Schnorr sign, verify, and Solidity-ready test-data tooling
- Optional `secret_key` and `nonce` overrides for Schnorr test-data generation
- BLS random G1/G2 generation and point addition
- BLS G1/G2 MSM testdata generation, execution, and strict validation
- BLS map-to-curve utilities (`Fp -> G1`, `Fp2 -> G2`)
- BLS sign/verify, batch testdata generation, aggregation, verification, and smoke flow
- Modularized BLS CLI command architecture
- Expanded unit test coverage across core crates and CLI command handlers

## Next Priorities

- Additional precompile helpers beyond Schnorr and BLS
- More integration tests that connect CLI vectors to Solidity contract tests
- Extended docs around precompile payload schemas and compatibility guarantees
- Optional direct on-chain contract interaction commands in CLI
- CI workflows for automated vector verification across crates/CLI/solidity

The project will continue to evolve alongside the runtime precompile implementations in the Polkadot SDK.