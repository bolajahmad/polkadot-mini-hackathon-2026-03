use bls12_381::{
    generate_random_fp2_input, generate_random_fp_input, map_fp2_to_g2, map_fp_to_g1,
};

use super::{
    points::{print_g1_point, print_g2_point},
    shared::{decode_fp2_input_or_exit, decode_fp_input_or_exit, exit_with_error, parse_fp2_input, parse_fp_input},
};

pub(super) fn cmd_map_fp_to_g1(fp: Option<String>) {
    let fp_input = parse_fp_input(fp, generate_random_fp_input())
        .unwrap_or_else(|e| exit_with_error(&format!("invalid fp input: {}", e)));
    let fp_value = decode_fp_input_or_exit(&fp_input);
    let mapped = map_fp_to_g1(fp_value).unwrap_or_else(|e| exit_with_error(&e));

    println!("=== BLS Map Fp -> G1 ===");
    println!("Input Fp: 0x{}", hex::encode(fp_input));
    print_g1_point("Mapped G1", &mapped);
}

pub(super) fn cmd_map_fp2_to_g2(fp2: Option<String>) {
    let fp2_input = parse_fp2_input(fp2, generate_random_fp2_input())
        .unwrap_or_else(|e| exit_with_error(&format!("invalid fp2 input: {}", e)));
    let fp2_value = decode_fp2_input_or_exit(&fp2_input);
    let mapped = map_fp2_to_g2(fp2_value).unwrap_or_else(|e| exit_with_error(&e));

    println!("=== BLS Map Fp2 -> G2 ===");
    println!("Input Fp2: 0x{}", hex::encode(fp2_input));
    print_g2_point("Mapped G2", &mapped);
}
