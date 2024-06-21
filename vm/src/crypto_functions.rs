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

#[cfg(feature = "wasm-incopatible")]
pub fn verify_ed25519(key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    use ed25519_dalek::*;

    let public = PublicKey::from_bytes(key);
    if public.is_err() {
        return false;
    }

    let sig = Signature::from_bytes(signature);
    if sig.is_err() {
        return false;
    }

    public.unwrap().verify(message, &sig.unwrap()).is_ok()
}

#[cfg(not(feature = "wasm-incopatible"))]
pub fn verify_ed25519(_key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
    panic!("verify_ed25519 not supported for wasm builds, feature `wasm-incopatible` needs to be enabled")
}
