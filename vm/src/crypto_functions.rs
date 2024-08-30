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

pub fn verify_bls_signature(key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    use blst::{
        min_pk::{PublicKey, Signature},
        BLST_ERROR,
    };

    let public_key = PublicKey::from_bytes(key).unwrap();

    let sig = Signature::from_bytes(signature).unwrap();

    let verify_response = sig.verify(true, message, BLS_DST_VALUE, &[], &public_key, true);

    match verify_response {
        BLST_ERROR::BLST_SUCCESS => true,
        _ => false,
    }
}

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

    match verify_response {
        BLST_ERROR::BLST_SUCCESS => true,
        _ => false,
    }
}
