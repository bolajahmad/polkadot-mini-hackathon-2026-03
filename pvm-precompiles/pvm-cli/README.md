# PVM CLI

The PVM CLI is a developer utility for interacting with cryptographic precompiles.

It provides commands that generate valid inputs and expected outputs for various cryptographic operations.

## Installation

```bash
cd pvm-cli
cargo build --release
```

The binary will be available at `target/release/pvmcli`.

## Usage

### Schnorr Signature Operations

#### Generate a Signature

```bash
# Basic usage
pvmcli schnorr sign --secret-key <HEX> --message "Hello, world!"

# With custom aux (nonce randomness)
pvmcli schnorr sign \
  --secret-key 0101010101010101010101010101010101010101010101010101010101010101 \
  --message "Hello, world!" \
  --aux 0202020202020202020202020202020202020202020202020202020202020202

# JSON output
pvmcli schnorr sign --secret-key <HEX> --message "Test" --output json
```

**Options:**
- `-s, --secret-key` — Secret key as hex (32 bytes / 64 hex chars)
- `-m, --message` — Message to sign (will be hashed with keccak256)
- `-a, --aux` — Optional auxiliary randomness for nonce (32 bytes hex)
- `-o, --output` — Output format: `hex` (default) or `json`

#### Verify a Signature

```bash
pvmcli schnorr verify \
  --pubkey <PUBKEY_X_HEX> \
  --nonce <R_X_HEX> \
  --signature <S_HEX> \
  --message "Hello, world!"
```

**Options:**
- `-p, --pubkey` — Public key x-coordinate (32 bytes hex)
- `-n, --nonce` — Nonce point R x-coordinate (32 bytes hex)
- `-s, --signature` — Signature scalar s (32 bytes hex)
- `-m, --message` — Original message (will be hashed)

#### Generate Test Data for Solidity

```bash
pvmcli schnorr test-data --message "Hello, world!"

# Optional deterministic test vector overrides
pvmcli schnorr test-data \
  --message "Hello, world!" \
  --secret-key 0101010101010101010101010101010101010101010101010101010101010101 \
  --nonce 0202020202020202020202020202020202020202020202020202020202020202
```

This generates ready-to-use Solidity constants and struct initialization code for testing.

**Options:**
- `-m, --message` — Message to sign (will be hashed with keccak256)
- `-s, --secret-key` — Optional secret key override (32 bytes / 64 hex chars)
- `-n, --nonce` — Optional nonce seed override (32 bytes / 64 hex chars)

## Examples

### Sign and Verify Flow

```bash
# Generate signature
$ pvmcli schnorr sign \
  --secret-key 0101010101010101010101010101010101010101010101010101010101010101 \
  --message "Test message"

=== Schnorr Signature Generated ===
Public Key (x):     0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798
Nonce (R_x):        0x...
Signature (s):      0x...
Message Hash:       0x...
---
Precompile Input (128 bytes):
0x...

# Verify the signature
$ pvmcli schnorr verify \
  --pubkey 79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798 \
  --nonce <NONCE_FROM_ABOVE> \
  --signature <SIGNATURE_FROM_ABOVE> \
  --message "Test message"

✓ Signature is VALID
```

### Generate Solidity Test Data

```bash
$ pvmcli schnorr test-data --message "Hello"

=== Schnorr Test Data for Solidity ===
Message: "Hello"

// Solidity test data
bytes32 constant PUBKEY_X = 0x...;
bytes32 constant NONCE_RX = 0x...;
bytes32 constant SIGNATURE_S = 0x...;
bytes32 constant MESSAGE_HASH = 0x...;

// Full precompile input (128 bytes)
bytes constant PRECOMPILE_INPUT = hex"...";

// For SchnorrSignature struct
SchnorrSignature memory sig = SchnorrSignature({
    pubkey: 0x...,
    nonce: 0x...,
    s: 0x...,
    message: 0x...
});
```

## Planned Features

- BLS12-381 operations (G1/G2 point generation, MSM, pairing)
- Direct contract interaction
- Batch test vector generation
- Automated integration testing