# Schnorr Signatures for PVM (Specification Notes)

## Abstract

This document describes the Schnorr verification precompile used by `pallet-revive` in the PVM runtime.

The design is inspired by BIP-340, but differs in hashing details and tagged-hash construction.

## Motivation

Solidity contracts can verify ECDSA signatures via `ecrecover`, but advanced use cases such as efficient aggregation-friendly verification benefit from Schnorr-style constructions.

Providing Schnorr verification as a runtime precompile allows contracts to verify signatures with predictable cost and without implementing complex elliptic-curve logic in Solidity.

## Precompile Parameters

The precompile operates on `secp256k1`.

```text
Field prime, p = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F
Curve equation: y^2 = x^3 + 7 (mod p)
Gx = 79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798
Gy = 483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8
Group order, n = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141
Precompile address: 0x905
```

## Verification Input Format

The precompile accepts exactly 128 bytes:

```text
Offset  Field
0       pubkey_x
32      r_x
64      s
96      message
```

Interpretation rules:

- `pubkey_x` is interpreted as an x-coordinate on `secp256k1`; derived y must be even.
- `r_x` is interpreted as an x-coordinate for nonce point `R`; derived y must be even.
- `s` must be a valid scalar (`s < n`).

## Verification Algorithm

Given a 128-byte input:

1. Decode into `pubkey_x`, `r_x`, `s`, and `msg`.
2. Reject if `s >= n`.
3. Compute challenge scalar:
   - `e = int(tagged_hash("PIP/challenge", r_x || pubkey_x || msg)) mod n`
4. Check equation:
   - `s * G == R + e * P`
5. Return `1` for valid signature, else `0` (encoded as 32 bytes).

## Signing Notes

Although the precompile verifies only, deterministic signing behavior is defined so generated vectors remain consistent.

Given secret key `d` and message hash `msg`:

1. Derive public key `P`; if `P.y` is odd, negate `d`.
2. Derive nonce using tagged hashing (`PIP/aux`, `PIP/nonce`) and adjust parity if needed.
3. Compute challenge `e` with `PIP/challenge` and `(r_x, pubkey_x, msg)`.
4. Compute `s = r + e * d`.
5. Signature is `(r_x, s)`.

## Tagged Hash Construction

Tagged hashes are domain-separated and used to avoid cross-context collisions:

- `PIP/aux`
- `PIP/nonce`
- `PIP/challenge`

## Differences from BIP-340

1. This implementation uses Keccak256.
2. Tagged-hash handling differs from canonical BIP-340 double-hash style.

## Tooling in This Repository

- `crates/schnorr`: core signing/verifying utilities and precompile input encoders.
- `pvmcli schnorr sign|verify|test-data`: CLI flows for generating vectors and verification payloads.
- `pvmcli schnorr test-data`: supports optional `--secret-key` and `--nonce` overrides for deterministic Solidity test data.
