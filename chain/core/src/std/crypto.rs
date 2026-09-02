//! Standard crypto functions and constants used across many MultiversX crates.
//!
//! TODO: move here the rest of the functions and the Wallet, with signing.

pub const SHA256_RESULT_LEN: usize = 32;
pub const KECCAK256_RESULT_LEN: usize = 32;

/// Computes the SHA-256 hash of the given data.
///
/// Returns a 32-byte array containing the hash digest.
pub fn sha256(data: &[u8]) -> [u8; SHA256_RESULT_LEN] {
    use sha2::Digest;
    sha2::Sha256::digest(data).into()
}

/// Computes the Keccak-256 hash of the given data.
///
/// Returns a 32-byte array containing the hash digest.
pub fn keccak256(data: &[u8]) -> [u8; KECCAK256_RESULT_LEN] {
    use sha3::Digest;
    sha3::Keccak256::digest(data).into()
}
