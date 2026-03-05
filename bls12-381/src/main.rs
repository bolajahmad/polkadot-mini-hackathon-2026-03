use std::result;

use ark_bls12_381::Fq;
use ark_bls12_381::Fq2;
use ark_bls12_381::Fr;
use ark_bls12_381::G1Affine;
use ark_bls12_381::G1Projective;
use ark_bls12_381::G2Affine;
use ark_bls12_381::G2Projective;
use ark_ec::AffineRepr;
use ark_ec::CurveGroup;
use ark_ff::BigInteger;
use ark_ff::Fp;
use ark_ff::PrimeField;
use ark_ff::Zero;
use ark_std::UniformRand;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

fn seeded_rng() -> ChaCha20Rng {
    ChaCha20Rng::from_seed([62u8; 32])
}

fn encode_fq(f: Fq) -> [u8; 64] {
    let mut out = [0u8; 64];

    let be = f.into_bigint().to_bytes_be();
    let start = 64 - be.len();

    out[start..].copy_from_slice(&be);
    out
}

fn encode_g1(p: G1Affine) -> [u8; 128] {
    let mut out = [0u8; 128];

    out[..64].copy_from_slice(&encode_fq(p.x));
    out[64..].copy_from_slice(&encode_fq(p.y));

    out
}

fn generate_g1_add_params() {
    let mut rng = seeded_rng();

    let point_a = G1Projective::rand(&mut rng);
    let point_b = G1Projective::rand(&mut rng);

    let result = (point_a + point_b).into_affine();

    let mut input = Vec::new();
    input.extend_from_slice(&encode_g1(point_a.into_affine()));
    input.extend_from_slice(&encode_g1(point_b.into_affine()));

    let output = encode_g1(result);

    // println!("G1ADD input: 0x{}", hex::encode(input));
    // println!("G1ADD output: 0x{}", hex::encode(output));
}

fn encode_scalar(f: Fr) -> [u8; 32] {
    let mut out = [0u8; 32];

    let be = f.into_bigint().to_bytes_be();
    let start = 32 - be.len();

    out[start..32].copy_from_slice(&be);

    out
}

fn generate_g1_msm_params(k: usize) {
    let mut rng = seeded_rng();

    let mut points = Vec::with_capacity(k);
    let mut scalars = Vec::with_capacity(k);
    let mut inputs = Vec::with_capacity(k * 160_usize);

    for _ in 0..k {
        let p: G1Affine = G1Projective::rand(&mut rng).into_affine();
        let s = Fp::rand(&mut rng);

        inputs.extend_from_slice(&encode_g1(p));
        inputs.extend_from_slice(&encode_scalar(s));

        points.push(p);
        scalars.push(s);
    }
    println!("MSM input point for index: 0x{}", hex::encode(&inputs));

    let mut acc = G1Projective::zero();
    for (p, s) in points.iter().zip(scalars.iter()) {
        acc += p.mul_bigint(s.into_bigint());
    }

    let result = acc.into_affine();

    println!("MSM output: 0x{}", hex::encode(encode_g1(result)));
    println!("MSM input: 0x{}", hex::encode(inputs));
}

fn encode_fp(f: &Fq) -> [u8; 64] {
    let mut out = [0u8; 64];

    let mut bytes = f.into_bigint().to_bytes_be();
    if bytes.len() < 48 {
        let mut padded = vec![0u8; 48 - bytes.len()];
        padded.extend(bytes);
        bytes = padded;
    }

    out[16..].copy_from_slice(&bytes);
    out
}

fn encode_fp2(f: &Fq2) -> [u8; 128] {
    let mut out = [0u8; 128];

    let c1 = encode_fp(&f.c1);
    let c0 = encode_fp(&f.c0);

    out[0..64].copy_from_slice(&c1);
    out[64..128].copy_from_slice(&c0);

    out
}

fn encode_g2(p: &G2Affine) -> [u8; 256] {
    if p.x.is_zero() && p.y.is_zero() {
        return [0u8; 256];
    }

    let mut out = [0u8; 256];

    let x = encode_fp2(&p.x);
    let y = encode_fp2(&p.y);

    out[0..128].copy_from_slice(&x);
    out[128..256].copy_from_slice(&y);

    out
}

pub fn generate_g2_add_test() {
    let mut rng = seeded_rng();

    let p = G2Projective::rand(&mut rng).into_affine();
    let q = G2Projective::rand(&mut rng).into_affine();

    let result = (p + q).into_affine();

    let mut input = Vec::new();

    input.extend_from_slice(&encode_g2(&p));
    input.extend_from_slice(&encode_g2(&q));

    let output = encode_g2(&result);

    println!("INPUT: 0x{}", hex::encode(input));
    println!("OUTPUT: 0x{}", hex::encode(output));
}

pub fn generate_g2_msm_test(k: usize) {
    let mut rng = seeded_rng();

    let mut points = Vec::with_capacity(k);
    let mut scalars = Vec::with_capacity(k);
    let mut inputs = Vec::with_capacity(k * 288);

    for _ in 0..k {
        let p = G2Projective::rand(&mut rng).into_affine();
        let s = Fp::rand(&mut rng);
        points.push(p);
        scalars.push(s);

        inputs.extend_from_slice(&encode_g2(&p));
        inputs.extend_from_slice(&encode_scalar(s));
    }

    let mut acc = G2Projective::zero();
    for (p, s) in points.iter().zip(scalars.iter()) {
        acc += p.mul_bigint(s.into_bigint());
    }

    let result = acc.into_affine();

    println!("MSM output: 0x{}", hex::encode(encode_g2(&result)));
    println!("MSM input: 0x{}", hex::encode(inputs));
}

fn main() {
    // generate_g1_add_params();
    // generate_g1_msm_params(2);

    // generate_g2_add_test();
    generate_g2_msm_test(4);
}
