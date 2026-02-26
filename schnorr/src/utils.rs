use alloy_primitives::Keccak256;
use secp256k1::{Parity, PublicKey, Scalar, Secp256k1, SecretKey};

pub fn generate_nonce_key(aux: &[u8; 32], priv_key: &SecretKey, msg: &[u8; 32]) -> SecretKey {
    let secp = Secp256k1::new();

    let mut hasher = Keccak256::new();
    hasher.update("PIP/aux");
    hasher.update(aux);
    let result = hasher.finalize();
    let aux_bytes = *result;

    let t = {
        let mut bytes = [0u8; 32];
        let sk_bytes = priv_key.secret_bytes();
        for i in 0..bytes.len() {
            bytes[i] = aux_bytes[i] ^ sk_bytes[i];
        }
        bytes
    };

    let mut nonce_bytes = Vec::with_capacity(96);
    nonce_bytes.extend_from_slice(&t);
    nonce_bytes.extend_from_slice(msg);
    let mut hasher = Keccak256::new();
    hasher.update("PIP/nonce");
    hasher.update(&nonce_bytes);
    let result = hasher.finalize();
    let nonce_bytes = *result;

    let mut nonce_sk = SecretKey::from_slice(&nonce_bytes).unwrap();
    let (_, parity) = PublicKey::from_secret_key(&secp, &nonce_sk).x_only_public_key();
    if parity == Parity::Odd {
        nonce_sk = nonce_sk.negate();
    }

    nonce_sk
}

pub fn message(text: Option<&str>) -> [u8; 32] {
    let value = text.unwrap_or("Hello, world!");
    let mut hasher = Keccak256::new();
    hasher.update(value);
    hasher.finalize().into()
}

pub fn generate_challenge_hash(rx: &[u8; 32], pubkey_x: &[u8; 32], msg: &[u8; 32]) -> Scalar {
    let mut hasher = Keccak256::new();
    hasher.update("PIP/challenge");
    hasher.update(rx);
    hasher.update(pubkey_x);
    hasher.update(msg);
    let digest = hasher.finalize();

    Scalar::from_be_bytes(*digest).unwrap()
}