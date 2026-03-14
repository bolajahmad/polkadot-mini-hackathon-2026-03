# Cryptographic Test Vector Crates

This directory contains Rust crates used to generate deterministic test vectors for the PVM cryptographic precompiles.

Each crate demonstrates how the underlying cryptographic primitives work and produces valid inputs and outputs for testing.

These tools help developers:

• understand how precompile inputs are constructed  
• generate reproducible cryptographic data  
• validate runtime implementations  

Examples include:

• Schnorr signature generation  
• elliptic curve point operations  
• test vector generation for multi-scalar multiplication