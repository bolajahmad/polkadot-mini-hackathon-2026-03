# PVM Precompiles Developer Toolkit

This repository contains the developer tooling, Solidity libraries, and test infrastructure for interacting with cryptographic precompiles implemented for the PVM runtime.

The goal of this project is to make advanced cryptographic functionality accessible to smart contracts running on parachains that integrate `pallet-revive`. These precompiles expose high-performance primitives such as BLS operations and Schnorr signature verification directly to smart contracts, enabling efficient on-chain verification that would otherwise be impractical to implement in Solidity.

The runtime implementations of these precompiles live inside a fork of the Polkadot SDK and integrates directly with the PVM runtime.

---

## Runtime Precompile Implementations

The core precompile logic is implemented within the `pallet-revive` runtime module in the Polkadot SDK fork.

Key implementations include:

**Schnorr Signature Verification *(Built based on BIP-340)***
https://github.com/bolajahmad/polkadot-sdk/blob/2fdb7206d942fc4a7a677261131ba4fa30c4b54f/substrate/frame/revive/src/precompiles/builtin/schnorr.rs

**BLS12-381 Cryptographic Operations *(Built based on EIP-2537)***
https://github.com/bolajahmad/polkadot-sdk/blob/2fdb7206d942fc4a7a677261131ba4fa30c4b54f/substrate/frame/revive/src/precompiles/builtin/bls12.rs

These implementations expose efficient cryptographic primitives such as:

- BLS12-381 G1 and G2 elliptic curve operations
- Multi-scalar multiplication (MSM)
- Pairing checks
- Schnorr signature verification

These operations are exposed as **native precompiles**, allowing smart contracts to execute them with significantly lower gas costs than equivalent Solidity implementations.

---

## Purpose of This Repository

While the runtime implementation lives in the Polkadot SDK, this repository provides the **developer-facing tooling required to interact with and test these precompiles**.

This includes:

- [Solidity helper libraries](pvm-precompiles/solidity/README.md) for smart contract integration
- CLI tooling for interacting with precompile inputs and outputs
- [Rust utilities](pvm-precompiles/crates) for generating deterministic cryptographic test vectors
- [Example implementations](pvm-precompiles/solidity/contracts/examples) demonstrating how the primitives work

The goal is to provide developers with a **complete toolkit** for building applications that rely on these cryptographic primitives.

---

# Repository Structure

```
root
├─ crates
│ └─ Rust programs for generating cryptographic test vectors
│
├─ pvm-cli
│ └─ Command line tooling for interacting with the precompiles
│
└─ solidity
| └─ Solidity libraries and examples for contract integration

```

---

## `crates/`

This directory contains standalone Rust programs designed to help developers understand and test the cryptographic primitives used by the precompiles.

Each crate focuses on a specific primitive or operation.

Examples include:

- Generating valid Schnorr signatures
- Producing deterministic test vectors
- Demonstrating elliptic curve operations used by the precompiles

These utilities are especially useful for:

- verifying runtime implementations
- producing reproducible test data
- understanding the encoding and input formats expected by the precompiles

---

## `pvm-cli`

The `pvm-cli` tool provides a command line interface for interacting with the cryptographic primitives and generated test vectors.

It allows developers to:

- generate valid input data for precompiles
- inspect encoded inputs and outputs
- experiment with signature generation and verification
- prepare test vectors for integration testing

This tool acts as a **developer utility layer** for working with the runtime precompiles.

---

## `solidity`

The Solidity directory contains libraries and examples that make it easy for Solidity developers to interact with the precompiles.

Instead of manually constructing calldata and interacting with raw precompile addresses, developers can import these libraries and call strongly-typed helper functions.

The library handles:

- ABI encoding of inputs
- calling the correct precompile addresses
- decoding returned results

Example usage patterns are included to demonstrate how contracts can integrate with the precompiles.

---

# Example Use Cases

These precompiles enable several powerful cryptographic use cases inside smart contracts:

### Signature Verification

Efficient verification of Schnorr signatures directly from Solidity contracts.

### BLS Signature Aggregation

BLS primitives allow many signatures to be aggregated into a single verification, significantly reducing on-chain verification costs.

### Zero-Knowledge Proof Verification

BLS pairings and multi-scalar multiplication are key components of many zk-proof systems.

### Advanced Cryptographic Applications

Developers can build new protocols relying on efficient elliptic curve operations without re-implementing complex cryptography in Solidity.

These use-cases would naturally be impossible (or way too expensive) to implement on-chain, but with these precompiles and PVM, this won't be a problem.

---

# Architecture Overview

```
Solidity Contracts
│
▼
Solidity Helper Library
│
▼
PVM Precompile Interface
│
▼
Runtime Precompile Implementation
(pallet-revive)
│
▼
Optimized Cryptographic Primitives
```


---

# Project Status

The runtime precompile implementations are actively being developed and tested within the Polkadot SDK fork.

This repository currently focuses on:

- improving developer experience
- providing tooling for testing and interaction
- documenting the usage of the precompiles

Additional tooling and examples will continue to be added. CUrrently, the precompiles library expose precompoles logic for the new Schnorr and BLS signatures, but the aim is to make it also expose helpers for other available precompiles too.

---

# License

MIT