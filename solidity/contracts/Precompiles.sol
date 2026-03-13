// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import "./modules/BLS.sol";

library Precompiles {
    function blsAggregate(bytes memory input) internal view returns (bytes memory) {
        return BLS.g1Add(input);
    }
}