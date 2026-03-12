// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

contract BlsContract {
    address constant BLS_12381_G1_ADD_PRECOMPILE_ADDRESS = address(0x0b);
    address constant BLS_12381_G1_MSM_PRECOMPILE_ADDRESS = address(0x0c);
    address constant BLS_12381_G2_ADD_PRECOMPILE_ADDRESS = address(0x0d);
    address constant BLS_12381_G2_MSM_PRECOMPILE_ADDRESS = address(0x0e);
    address constant BLS_12381_FIELD_PAIRING_PRECOMPILE_ADDRESS = address(0x0f);
    address constant BLS_12381_FP_TO_G1_PRECOMPILE_ADDRESS = address(0x10);
    address constant BLS_12381_FP2_TO_G2_PRECOMPILE_ADDRESS = address(0x11);

    event Bls12G1Added(bytes data);
    event Bls12G1MSMPerformed(bytes data);
    event Bls12G2Added(bytes data);
    event Bls12G2MSMPerformed(bytes data);
    event MappedFpToG1(bytes scalar, bytes output);
    event MappedFp2ToG2(bytes scalar, bytes output);
    event FieldPairingDone(bytes input, bytes output);

    constructor() {}

    function addBlsG1Points(
        bytes memory point1,
        bytes memory point2,
        bytes memory expectedSum
    ) public returns (bytes memory) {
        // Each G1 point must be exactly 128 bytes (x||y, each 64 bytes EIP-2537 format)
        require(point1.length == 128, "point1 must be 128 bytes");
        require(point2.length == 128, "point2 must be 128 bytes");
        require(expectedSum.length == 128, "Sum must be 128 bytes");

        bytes memory input = abi.encodePacked(point1, point2); // total = 256 bytes

        (
            bool success,
            bytes memory output
        ) = BLS_12381_G1_ADD_PRECOMPILE_ADDRESS.staticcall(input);

        require(success, "BLS G1 addition failed");
        require(output.length == 128, "invalid precompile output");

        emit Bls12G1Added(output);

        require(
            keccak256(output) == keccak256(expectedSum),
            "BLS G1 addition result does not match expected sum"
        );
        return output;
    }

    function performG1Msm(bytes memory inputs, bytes memory expectedOut) public returns (bytes memory) {
        require(inputs.length % 160 == 0, "Input length must be a multiple of 160 bytes (point + scalar)");
        require(expectedOut.length == 128, "Expected output must be 128 bytes");

        (
            bool success,
            bytes memory output
        ) = BLS_12381_G1_MSM_PRECOMPILE_ADDRESS.staticcall(inputs);
        require(success, "Call should succeed!");
        require(output.length == 128, "invalid precompile output");
        require(
            keccak256(output) == keccak256(expectedOut),
            "BLS G1 MSM result does not match expected output"
        );
        emit Bls12G1MSMPerformed(output);

        return output;
    }

    function addBlsG2Points(
        bytes memory point1,
        bytes memory point2,
        bytes memory expectedSum
    ) public returns (bytes memory) {
        // Each G2 point must be exactly 256 bytes (x_im||x_re||y_im||y_re, each 64 bytes EIP-2537 format)
        require(point1.length == 256, "point1 must be 256 bytes");
        require(point2.length == 256, "point2 must be 256 bytes");

        bytes memory input = abi.encodePacked(point1, point2); // total = 512 bytes

        (bool success, bytes memory output) = BLS_12381_G2_ADD_PRECOMPILE_ADDRESS.staticcall(input);
        require(success, "BLS G2 addition failed");
        // require(output.length == 256, "invalid precompile output");

        // require(
        //     keccak256(output) == keccak256(expectedSum),
        //     "BLS G2 addition result does not match expected sum"
        // );
        emit Bls12G2Added(output);
        return output;
    }

    function performG2Msm(bytes memory inputs, bytes memory expectedOut) public returns (bytes memory) {
        require(inputs.length % 288 == 0, "Input length must be a multiple of 288 bytes (point + scalar)");
        require(expectedOut.length == 256, "Expected output must be 256 bytes");

        (
            bool success,
            bytes memory output
        ) = BLS_12381_G2_MSM_PRECOMPILE_ADDRESS.staticcall(inputs);
        require(success, "Call should succeed!");
        require(output.length == 256, "invalid precompile output");
        require(
            keccak256(output) == keccak256(expectedOut),
            "BLS G2 MSM result does not match expected output"
        );
        emit Bls12G2MSMPerformed(output);

        return output;
    }

    function mapFpToG1Point(bytes memory input, bytes memory expectedOut) public returns (bytes memory) {
        require(input.length == 64, "Input must be 64 bytes (EIP-2537 format)");
        require(expectedOut.length == 128, "Expected output must be 128 bytes");

        (
            bool success,
            bytes memory output
        ) = BLS_12381_FP_TO_G1_PRECOMPILE_ADDRESS.staticcall(input);
        require(success, "Call should succeed!");
        // require(output.length == 128, "invalid precompile output");
        // require(
        //     keccak256(output) == keccak256(expectedOut),
        //     "FP to G1 mapping result does not match expected output"
        // );

        emit MappedFpToG1(input, output);

        return output;
    }

    function mapFp2ToG2Point(bytes memory input, bytes memory expectedOut) public returns (bytes memory) {
        require(input.length == 128, "Input must be 128 bytes (EIP-2537 format)");
        require(expectedOut.length == 256, "Expected output must be 256 bytes");

        (
            bool success,
            bytes memory output
        ) = BLS_12381_FP2_TO_G2_PRECOMPILE_ADDRESS.staticcall(input);
        require(success, "Call should succeed!");
        // require(output.length == 256, "invalid precompile output");
        // require(
        //     keccak256(output) == keccak256(expectedOut),
        //     "FP2 to G2 mapping result does not match expected output"
        // );

        emit MappedFp2ToG2(input, output);

        return output;
    }

    function fieldPairing(bytes memory input) public {
        require(input.length % 384 == 0, "Input must be multiple of 384 bytes (G1 point and G2 point)");

        (bool success, bytes memory output) = BLS_12381_FIELD_PAIRING_PRECOMPILE_ADDRESS.staticcall(input);
        emit FieldPairingDone(input, output);
        require(success, "Call should succeed!");
        // require(output.length == 32, "invalid precompile output");
    }
}
