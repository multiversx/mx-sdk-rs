//! Thin wrappers over [`multiversx_chain_core::std::crypto`] for convenient use in the VM.

use multiversx_chain_core::std::crypto;

/// Computes the SHA-256 hash of `data`.
/// Returns a 32-byte digest.
///
/// Wraps [`multiversx_chain_core::std::crypto::sha256`].
pub fn sha256(data: &[u8]) -> [u8; crypto::SHA256_RESULT_LEN] {
    crypto::sha256(data)
}

/// Computes the Keccak-256 hash of `data`.
/// Returns a 32-byte digest.
///
/// Wraps [`multiversx_chain_core::std::crypto::keccak256`].
pub fn keccak256(data: &[u8]) -> [u8; crypto::KECCAK256_RESULT_LEN] {
    crypto::keccak256(data)
}

/// Verifies an Ed25519 signature.
///
/// Wraps [`multiversx_chain_core::std::crypto::ed25519::Ed25519VerifyingKey::verify`].
///
/// Returns `true` if `signature` is a valid Ed25519 signature of `message`
/// under the public key `key`.
///
/// Returns `false` if:
/// - `key` is not exactly 32 bytes
/// - `signature` is not exactly 64 bytes
/// - `key` does not represent a valid curve point
/// - the signature does not verify
pub fn verify_ed25519(key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    let Ok(key_32) = key.try_into() else {
        return false;
    };
    let Ok(sig_64) = signature.try_into() else {
        return false;
    };
    let Some(verifying_key) = crypto::ed25519::Ed25519VerifyingKey::from_bytes(key_32) else {
        return false;
    };
    let sig = crypto::ed25519::Ed25519Signature::from_bytes(sig_64);
    verifying_key.verify(message, &sig)
}
