use ed25519_dalek::*;
use sha2::Sha256;
use sha3::{Digest, Keccak256};

pub const SHA256_RESULT_LEN: usize = 32;
pub const KECCAK256_RESULT_LEN: usize = 32;

pub fn sha256(data: &[u8]) -> [u8; SHA256_RESULT_LEN] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn keccak256(data: &[u8]) -> [u8; KECCAK256_RESULT_LEN] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn verify_ed25519(key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    let key_32: [u8; 32] = if let Ok(key_32) = key.try_into() {
        key_32
    } else {
        return false;
    };
    let signature_64: [u8; 64] = if let Ok(signature_64) = signature.try_into() {
        signature_64
    } else {
        return false;
    };

    let verifying_key_result = VerifyingKey::from_bytes(&key_32);
    let verifying_key = if let Ok(verifying_key) = verifying_key_result {
        verifying_key
    } else {
        return false;
    };

    let sig = Signature::from_bytes(&signature_64);

    let result = verifying_key.verify(message, &sig);
    result.is_ok()
}
