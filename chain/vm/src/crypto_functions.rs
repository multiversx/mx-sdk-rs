use multiversx_chain_core::std::crypto;


pub fn sha256(data: &[u8]) -> [u8; crypto::SHA256_RESULT_LEN] {
    crypto::sha256(data)
}

pub fn keccak256(data: &[u8]) -> [u8; crypto::KECCAK256_RESULT_LEN] {
    crypto::keccak256(data)
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
