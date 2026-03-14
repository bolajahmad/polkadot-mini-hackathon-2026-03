use ark_bls12_381::{Fr, G1Affine, G2Affine};
use bls12_381::{
    compute_g1_msm, compute_g2_msm, encode_g1_msm_input, encode_g2_msm_input,
    generate_g1_msm_testdata, generate_g2_msm_testdata,
};
use serde::Serialize;

use super::{
    points::{print_g1_point, print_g2_point},
    shared::{
        exit_with_error, g1_to_output, g2_to_output, parse_g1_msm_pairs, parse_g2_msm_pairs,
        scalar_to_hex,
    },
};

#[derive(Serialize)]
struct G1MsmOut {
    points: Vec<super::shared::G1PointOut>,
    scalars: Vec<String>,
}

#[derive(Serialize)]
struct G2MsmOut {
    points: Vec<super::shared::G2PointOut>,
    scalars: Vec<String>,
}

pub(super) fn cmd_g1_msm_testdata(pairs: usize, output: String) {
    let (pairs_data, result) =
        generate_g1_msm_testdata(pairs).unwrap_or_else(|e| exit_with_error(&e));

    let encoded_input = encode_g1_msm_input(&pairs_data);
    println!("=== BLS G1 MSM Test Data ===");
    println!("Pairs: {}", pairs_data.len());

    match output.as_str() {
        "hex" => print_g1_msm_hex(&encoded_input, &result),
        "json" => print_g1_msm_json(&pairs_data, &result),
        _ => {
            print_g1_msm_hex(&encoded_input, &result);
            print_g1_msm_json(&pairs_data, &result);
        }
    }
}

pub(super) fn cmd_g2_msm_testdata(pairs: usize, output: String) {
    let (pairs_data, result) =
        generate_g2_msm_testdata(pairs).unwrap_or_else(|e| exit_with_error(&e));

    let encoded_input = encode_g2_msm_input(&pairs_data);
    println!("=== BLS G2 MSM Test Data ===");
    println!("Pairs: {}", pairs_data.len());

    match output.as_str() {
        "hex" => print_g2_msm_hex(&encoded_input, &result),
        "json" => print_g2_msm_json(&pairs_data, &result),
        _ => {
            print_g2_msm_hex(&encoded_input, &result);
            print_g2_msm_json(&pairs_data, &result);
        }
    }
}

pub(super) fn cmd_g1_msm(data: String) {
    let pairs_data = parse_g1_msm_pairs(&data)
        .unwrap_or_else(|e| exit_with_error(&format!("invalid g1 msm data: {}", e)));
    let result = compute_g1_msm(&pairs_data).unwrap_or_else(|e| exit_with_error(&e));

    println!("=== BLS G1 MSM ===");
    println!("Input pairs validated: {}", pairs_data.len());
    print_g1_point("Result", &result);
}

pub(super) fn cmd_g2_msm(data: String) {
    let pairs_data = parse_g2_msm_pairs(&data)
        .unwrap_or_else(|e| exit_with_error(&format!("invalid g2 msm data: {}", e)));
    let result = compute_g2_msm(&pairs_data).unwrap_or_else(|e| exit_with_error(&e));

    println!("=== BLS G2 MSM ===");
    println!("Input pairs validated: {}", pairs_data.len());
    print_g2_point("Result", &result);
}

pub(super) fn cmd_g1_msm_validate(data: String) {
    let pairs_data = parse_g1_msm_pairs(&data)
        .unwrap_or_else(|e| exit_with_error(&format!("invalid g1 msm data: {}", e)));
    println!("VALID G1 MSM input with {} point-scalar pairs", pairs_data.len());
}

pub(super) fn cmd_g2_msm_validate(data: String) {
    let pairs_data = parse_g2_msm_pairs(&data)
        .unwrap_or_else(|e| exit_with_error(&format!("invalid g2 msm data: {}", e)));
    println!("VALID G2 MSM input with {} point-scalar pairs", pairs_data.len());
}

fn print_g1_msm_hex(input: &[u8], result: &G1Affine) {
    println!("Input (hex): 0x{}", hex::encode(input));
    print_g1_point("MSM Result", result);
}

fn print_g2_msm_hex(input: &[u8], result: &G2Affine) {
    println!("Input (hex): 0x{}", hex::encode(input));
    print_g2_point("MSM Result", result);
}

fn print_g1_msm_json(pairs: &[(G1Affine, Fr)], result: &G1Affine) {
    let out = G1MsmOut {
        points: pairs.iter().map(|(p, _)| g1_to_output(*p)).collect(),
        scalars: pairs.iter().map(|(_, s)| scalar_to_hex(*s)).collect(),
    };

    println!(
        "JSON: {}",
        serde_json::to_string_pretty(&out).expect("serialize g1 msm json")
    );
    print_g1_point("MSM Result", result);
}

fn print_g2_msm_json(pairs: &[(G2Affine, Fr)], result: &G2Affine) {
    let out = G2MsmOut {
        points: pairs.iter().map(|(p, _)| g2_to_output(*p)).collect(),
        scalars: pairs.iter().map(|(_, s)| scalar_to_hex(*s)).collect(),
    };

    println!(
        "JSON: {}",
        serde_json::to_string_pretty(&out).expect("serialize g2 msm json")
    );
    print_g2_point("MSM Result", result);
}
