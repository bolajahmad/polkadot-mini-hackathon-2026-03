# Building With PVM Precompiles

This guide explains how developers can integrate the cryptographic precompiles into their smart contracts.

Instead of manually interacting with precompile addresses and encoding raw calldata, this repository provides Solidity helper libraries that make integration straightforward.

---

# Using the Solidity Library

Import the helper library:

`import "pvm-precompiles/BLS.sol";`


The library wraps the low-level precompile calls and exposes easy-to-use functions.

Example:
```
BLS.G1Point memory result = BLS.g1Add(p1, p2);
```

The library handles:

• ABI encoding  
• precompile address routing  
• return value decoding  

---

# Example Contract

```
contract Example {
    function addPoints(
    BLS.G1Point memory a,
    BLS.G1Point memory b
    ) public returns (BLS.G1Point memory) {
        return BLS.g1Add(a, b);
    }
}
```

---

# CLI Tool

The `pvm-cli` tool can be used to generate valid inputs for precompiles.

Example:
`pvm-cli bls g1-add`


This generates encoded inputs and expected outputs for testing.

---

# Test Vector Generation

The Rust crates inside the repository generate deterministic cryptographic test vectors.

These vectors are useful for:

• validating precompile correctness  
• writing contract integration tests  
• debugging encoding issues  

---

# Recommended Workflow

1. generate test vectors with CLI
2. write Solidity contract
3. call helper library
4. compare results with expected output

---

# Example Use Cases

Developers can use these precompiles to build:

• aggregated signature verification  
• zk proof verification  
• decentralized identity systems  
• rollup verification contracts  

These primitives unlock powerful cryptographic functionality that would otherwise be too expensive in Solidity.