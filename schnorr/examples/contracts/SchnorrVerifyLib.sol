// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

library SchnorrCompat {
    address internal constant SCHNORR_ADDR =
        0x0000000000000000000000000000000000000905;
    bytes4 internal constant VERIFY_SELECTOR = 0x8e760afe; // verify(bytes)

    function verify(bytes memory input) internal view returns (bool) {
        (bool ok, bytes memory ret) =
            SCHNORR_ADDR.staticcall(abi.encodeWithSelector(VERIFY_SELECTOR, input));
        require(ok, "Schnorr precompile reverted");

        // Canonical ABI bool (32 bytes)
        if (ret.length == 32) {
            return abi.decode(ret, (bool));
        }

        // Temporary compatibility for observed 4-byte return (e.g. 0x01000000 / 0x00000000)
        if (ret.length == 4) {
            bytes4 r4 = bytes4(ret);
            if (r4 == 0x01000000 || r4 == 0x00000001) return true;
            if (r4 == 0x00000000) return false;
            revert("Invalid 4-byte bool payload");
        }

        // Optional extra tolerance
        if (ret.length == 1) {
            return uint8(ret[0]) != 0;
        }

        revert("Unexpected Schnorr return length");
    }
}

contract SchnorrVerifyCompat {
    function verifySchnorrSignature(bytes calldata input) external view returns (bool) {
        return SchnorrCompat.verify(input);
    }
}