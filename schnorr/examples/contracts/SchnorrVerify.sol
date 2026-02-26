// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import "./interface/ISchnorrVerify.sol";
import "./interface/ISystem.sol";

contract SchnorrVerify {
    function verifySchnorrSignature(bytes calldata input) public view returns (bool valid) {
        valid = ISchnorr(SCHNORR_ADDR).verify(input);
    }

    function callHashBlake256(bytes memory input) public pure returns (bytes32) {
        return ISystem(SYSTEM_ADDR).hashBlake256(input);
    }
}
