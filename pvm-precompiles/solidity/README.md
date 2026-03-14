# Solidity Precompile Libraries

This directory contains Solidity libraries that simplify interacting with the PVM cryptographic precompiles.

Instead of manually constructing low-level calls, developers can import these libraries and call strongly-typed functions.

The libraries handle:

• ABI encoding  
• precompile address routing  
• decoding results  

Example usage:

```
import "pvm-precompiles/BLS.sol";

BLS.G1Point memory result = BLS.g1Add(a, b);
```

This greatly improves developer experience when integrating advanced cryptographic primitives into smart contracts.

Example contracts demonstrating usage are included in this directory.
