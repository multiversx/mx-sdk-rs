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

#[cfg(feature = "bls")]
pub fn verify_bls_signature(key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    use blst::{
        min_pk::{PublicKey, Signature},
        BLST_ERROR,
    };

    let public_key = PublicKey::from_bytes(key).unwrap();

    let sig = Signature::from_bytes(signature).unwrap();

    let verify_response = sig.verify(true, message, BLS_DST_VALUE, &[], &public_key, true);

    matches!(verify_response, BLST_ERROR::BLST_SUCCESS)
}

#[cfg(feature = "bls")]
pub fn verify_multi_bls_signature(
    public_keys: Vec<&[u8]>,
    message: &[u8],
    aggregated_signature: &[u8],
) -> bool {
    use blst::{
        min_pk::{AggregatePublicKey, AggregateSignature, PublicKey, Signature},
        BLST_ERROR,
    };

    let mut aggregate_pk: AggregatePublicKey =
        AggregatePublicKey::from_public_key(&PublicKey::default());

    let public_keys_converted: Vec<PublicKey> = public_keys
        .iter()
        .filter_map(|key| PublicKey::from_bytes(key).ok())
        .collect();

    public_keys_converted
        .iter()
        .for_each(|pk| aggregate_pk.add_public_key(pk, true).unwrap());

    let aggregate_sig: Signature =
        AggregateSignature::from_signature(&Signature::from_bytes(aggregated_signature).unwrap())
            .to_signature();

    let verify_response = aggregate_sig.verify(
        true,
        message,
        BLS_DST_VALUE,
        &[],
        &aggregate_pk.to_public_key(),
        true,
    );

    matches!(verify_response, BLST_ERROR::BLST_SUCCESS)
}
