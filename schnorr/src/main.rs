use secp256k1::{Parity, PublicKey, Scalar, Secp256k1, SecretKey};

use crate::utils::{generate_challenge_hash, generate_nonce_key, message};

mod utils;

fn main() {
    let data = hex::encode(vec![
        27, 132, 197, 86, 123, 18, 100, 64, 153, 93, 62, 213, 170, 186, 5, 101, 215, 30, 24, 52,
        96, 72, 25, 255, 156, 23, 245, 233, 213, 221, 7, 143, 185, 193, 253, 118, 168, 12, 233,
        140, 246, 230, 56, 184, 91, 156, 64, 126, 215, 97, 23, 190, 41, 160, 218, 203, 114, 37,
        119, 133, 180, 205, 102, 30, 107, 136, 253, 211, 176, 4, 94, 136, 80, 61, 180, 248, 27,
        190, 86, 223, 63, 85, 44, 213, 109, 41, 168, 119, 103, 218, 116, 81, 227, 179, 53, 97, 182,
        225, 109, 39, 172, 90, 180, 39, 167, 246, 137, 0, 172, 85, 89, 206, 39, 45, 198, 195, 124,
        130, 179, 224, 82, 36, 108, 130, 36, 76, 80, 228,
    ]);
    println!("Data: 0x{}", data);

    let secp = Secp256k1::new();
    let mut secret_key = SecretKey::from_slice(&[1u8; 32]).unwrap();
    // Maybe negate the SecretKey
    // ensures the corresponding public key has even Y coordinate
    let (_, parity) = PublicKey::from_secret_key(&secp, &secret_key).x_only_public_key();
    if parity == Parity::Odd {
        secret_key = secret_key.negate();
    }

    let aux: [u8; 32] = [2; 32];
    let msg = message(Some("Hello, world!"));
    let nonce_secret = generate_nonce_key(&aux, &secret_key, &msg);

    // Get public keys
    let nonce_pubkey = PublicKey::from_secret_key(&secp, &nonce_secret);
    let (signer_xonly, _) = PublicKey::from_secret_key(&secp, &secret_key).x_only_public_key();
    let (nonce_xonly, _) = nonce_pubkey.x_only_public_key();

    let pubkey_x = signer_xonly.serialize();
    let rx = nonce_xonly.serialize();

    let challenge = generate_challenge_hash(&rx, &pubkey_x, &msg);
    // Calculate s = r + ed

    let ed = secret_key.mul_tweak(&challenge).expect("Should be valid");
    let ed_scalar = Scalar::from_be_bytes(ed.secret_bytes()).unwrap();
    let s = nonce_secret.add_tweak(&ed_scalar).expect("Should be valid");
    let s = s.secret_bytes();

    // Assemble the inputs
    let mut input = Vec::with_capacity(128);
    input.extend_from_slice(&pubkey_x);
    input.extend_from_slice(&rx);
    input.extend_from_slice(&s);
    input.extend_from_slice(&msg);

    println!("Input Length, {}", input.len());
    println!("Input: 0x{}", hex::encode(input));
}
