//! Standard crypto functions and constants used across many MultiversX crates.
//!
//! TODO: move here the rest of the function,s and the Wallet, with signing.

use sha2::Sha256;
use sha3::{Digest, Keccak256};

pub const SHA256_RESULT_LEN: usize = 32;
pub const KECCAK256_RESULT_LEN: usize = 32;

/// Computes the SHA-256 hash of the given data.
///
/// Returns a 32-byte array containing the hash digest.
pub fn sha256(data: &[u8]) -> [u8; SHA256_RESULT_LEN] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Computes the Keccak-256 hash of the given data.
///
/// Returns a 32-byte array containing the hash digest.
pub fn keccak256(data: &[u8]) -> [u8; KECCAK256_RESULT_LEN] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}
