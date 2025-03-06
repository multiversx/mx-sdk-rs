use blst::{
    min_pk::{AggregatePublicKey, PublicKey, Signature},
    BLST_ERROR,
};
use sha2::Sha256;
use sha3::{Digest, Keccak256};

pub const SHA256_RESULT_LEN: usize = 32;
pub const KECCAK256_RESULT_LEN: usize = 32;
pub const BLS_DST_VALUE: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_POP_";

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
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

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

pub fn verify_bls(key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    let public_key = PublicKey::from_bytes(key)
        .unwrap_or_else(|e| panic!("Failed to deserialize public key: {key:?}. Error: {e:?}"));

    let sig = Signature::from_bytes(signature)
        .unwrap_or_else(|e| panic!("Failed to deserialize signature: {signature:?}. Error: {e:?}"));

    let verify_response = sig.verify(true, message, BLS_DST_VALUE, &[], &public_key, true);

    matches!(verify_response, BLST_ERROR::BLST_SUCCESS)
}

pub fn verify_bls_aggregated_signature(
    keys: Vec<Vec<u8>>,
    message: &[u8],
    signature: &[u8],
) -> bool {
    let mut aggregate_pk = AggregatePublicKey::from_public_key(&PublicKey::default());

    for (i, key) in keys.iter().enumerate() {
        let public_key = PublicKey::from_bytes(key).unwrap_or_else(|e| {
            panic!("Failed to deserialize public key at index {i}: {key:?}. Error: {e:?}")
        });

        aggregate_pk
            .add_public_key(&public_key, true)
            .unwrap_or_else(|e| {
                panic!("Failed to add public key at index {i} to aggregate. Error: {e:?}")
            });
    }

    let signature = Signature::from_bytes(signature)
        .unwrap_or_else(|e| panic!("Failed to deserialize signature: {signature:?}. Error: {e:?}"));

    let verify_response = signature.verify(
        true,
        message,
        BLS_DST_VALUE,
        &[],
        &aggregate_pk.to_public_key(),
        true,
    );

    matches!(verify_response, BLST_ERROR::BLST_SUCCESS)
}
