use bls_binary_rust::{G1, G2};
use sha2::Sha256;
use sha3::{Digest, Keccak256};

pub const SHA256_RESULT_LEN: usize = 32;
pub const KECCAK256_RESULT_LEN: usize = 32;
pub const G2_STR: &str= "1 2345388737500083945391657505708625859903954047151773287623537600586029428359739211026111121073980842558223033704140 3558041178357727243543283929018475959655787667816024413880422701270944718005964809191925861299660390662341819212979 1111454484298065649047920916747797835589661734985194316226909186591481448224600088430816898704234962594609579273169 3988173108836042169913782128392219399166696378042311135661652175544044220584995583525611110036064603671142074680982";

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
    if key.len() != 96 || signature.len() != 48 || message.is_empty() {
        return false;
    }

    let mut public_key = G2::default();
    public_key.set_str(G2_STR);
    public_key.deserialize_g2(key);

    let mut sign = G1::default();
    sign.deserialize(signature);

    sign.verify(public_key, message)
}

pub fn verify_bls_aggregated_signature(
    _keys: Vec<Vec<u8>>,
    _message: &[u8],
    _signature: &[u8],
) -> bool {
    true
}
