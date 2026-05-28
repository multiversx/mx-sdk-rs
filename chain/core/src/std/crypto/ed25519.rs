use ed25519_dalek::{Signer, Verifier};

pub const SIGNATURE_LENGTH: usize = 64;

pub type Ed25519SigningKey = ed25519_dalek::SigningKey;
pub type Ed25519VerifyingKey = ed25519_dalek::VerifyingKey;
pub type Ed25519Signature = ed25519_dalek::Signature;

/// Constructs a signing key from a 32-byte seed.
pub fn signing_key_from_seed(seed: &[u8; 32]) -> Ed25519SigningKey {
    Ed25519SigningKey::from_bytes(seed)
}

/// Derives the verifying (public) key from a 32-byte signing seed.
pub fn verifying_key_from_seed(seed: &[u8; 32]) -> Ed25519VerifyingKey {
    signing_key_from_seed(seed).verifying_key()
}

/// Constructs a verifying key from its 32-byte compressed representation.
/// Returns `None` if the bytes do not represent a valid curve point.
pub fn verifying_key_from_bytes(bytes: &[u8; 32]) -> Option<Ed25519VerifyingKey> {
    Ed25519VerifyingKey::from_bytes(bytes).ok()
}

/// Constructs an Ed25519 signature from its 64-byte representation.
pub fn signature_from_bytes(bytes: &[u8; 64]) -> Ed25519Signature {
    Ed25519Signature::from_bytes(bytes)
}

/// Signs a message with the given signing key.
pub fn sign(signing_key: &Ed25519SigningKey, message: &[u8]) -> Ed25519Signature {
    signing_key.sign(message)
}

/// Verifies an Ed25519 signature. Returns `true` if the signature is valid.
pub fn verify(
    verifying_key: &Ed25519VerifyingKey,
    message: &[u8],
    signature: &Ed25519Signature,
) -> bool {
    verifying_key.verify(message, signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Known-good vectors generated with ed25519-dalek 2.2.0 and verified via
    // the roundtrip test below.  The seed is arbitrary; the pk and sig are
    // the values the library produces for (seed, empty message).
    const SEED: [u8; 32] =
        hex_literal::hex!("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae3d55");
    const PK_BYTES: [u8; 32] =
        hex_literal::hex!("700e2ce7c4b674427eab27ba820bcf6f0faebe68e09fe8564292114e41dc6a41");
    const SIG_BYTES: [u8; 64] = hex_literal::hex!(
        "37b4bd5f28b61f55dc9673ae2895baceb863d9cf51780d040f98ad8cdc896cf5\
         be46be655a863525da0959f7f373611585e437e28ec971b7bd206ff9bd26e803"
    );

    #[test]
    fn verifying_key_derived_from_seed() {
        assert_eq!(verifying_key_from_seed(&SEED).to_bytes(), PK_BYTES);
    }

    #[test]
    fn signing_key_from_seed_roundtrip() {
        let sk = signing_key_from_seed(&SEED);
        assert_eq!(sk.verifying_key().to_bytes(), PK_BYTES);
    }

    #[test]
    fn sign_produces_known_signature() {
        let sk = signing_key_from_seed(&SEED);
        assert_eq!(sign(&sk, b"").to_bytes(), SIG_BYTES);
    }

    #[test]
    fn verify_valid_signature() {
        let vk = verifying_key_from_seed(&SEED);
        let sig = signature_from_bytes(&SIG_BYTES);
        assert!(verify(&vk, b"", &sig));
    }

    #[test]
    fn verify_rejects_wrong_message() {
        let vk = verifying_key_from_seed(&SEED);
        let sig = signature_from_bytes(&SIG_BYTES);
        assert!(!verify(&vk, b"wrong", &sig));
    }

    #[test]
    fn verify_rejects_wrong_signature() {
        let vk = verifying_key_from_seed(&SEED);
        let mut bad_bytes = SIG_BYTES;
        bad_bytes[0] ^= 0xff;
        let bad_sig = signature_from_bytes(&bad_bytes);
        assert!(!verify(&vk, b"", &bad_sig));
    }

    #[test]
    fn verifying_key_from_bytes_roundtrip() {
        let vk = verifying_key_from_bytes(&PK_BYTES).unwrap();
        assert_eq!(vk.to_bytes(), PK_BYTES);
    }

    #[test]
    fn verifying_key_from_bytes_different_seeds_differ() {
        let vk1 = verifying_key_from_bytes(&PK_BYTES).unwrap();
        let vk2 = verifying_key_from_seed(&[0x42u8; 32]);
        assert_ne!(vk1.to_bytes(), vk2.to_bytes());
    }

    #[test]
    fn sign_then_verify_roundtrip() {
        let seed: [u8; 32] = [0x42u8; 32];
        let sk = signing_key_from_seed(&seed);
        let vk = sk.verifying_key();
        let msg = b"hello multiversx";
        let sig = sign(&sk, msg);
        assert!(verify(&vk, msg, &sig));
        assert!(!verify(&vk, b"other message", &sig));
    }
}
