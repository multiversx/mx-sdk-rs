use ed25519_dalek::{Signer, Verifier};

/// An Ed25519 signing (private) key, wrapping a 32-byte seed.
#[derive(Clone, PartialEq, Eq)]
pub struct Ed25519SigningKey(ed25519_dalek::SigningKey);

/// An Ed25519 verifying (public) key.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Ed25519VerifyingKey(ed25519_dalek::VerifyingKey);

/// An Ed25519 signature.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Ed25519Signature(ed25519_dalek::Signature);

impl Ed25519SigningKey {
    /// Constructs a signing key from a 32-byte seed.
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        Self(ed25519_dalek::SigningKey::from_bytes(seed))
    }

    /// Constructs a signing key from a 64-byte keypair (seed || public key).
    /// Returns an error if the public key half does not match the seed half.
    pub fn from_keypair_bytes(bytes: &[u8; 64]) -> Result<Self, ed25519_dalek::SignatureError> {
        ed25519_dalek::SigningKey::from_keypair_bytes(bytes).map(Self)
    }

    /// Returns the 32-byte seed (secret scalar).
    pub fn to_seed_bytes(&self) -> &[u8; 32] {
        self.0.as_bytes()
    }

    /// Returns the full 64-byte keypair encoding (seed || public key).
    pub fn to_keypair_bytes(&self) -> [u8; 64] {
        self.0.to_keypair_bytes()
    }

    /// Derives the corresponding verifying (public) key.
    pub fn verifying_key(&self) -> Ed25519VerifyingKey {
        Ed25519VerifyingKey(self.0.verifying_key())
    }

    /// Signs a message with this key.
    pub fn sign(&self, message: &[u8]) -> Ed25519Signature {
        Ed25519Signature(self.0.sign(message))
    }
}

impl Ed25519VerifyingKey {
    /// Constructs a verifying key from its 32-byte compressed representation.
    /// Returns `None` if the bytes do not represent a valid curve point.
    pub fn from_bytes(bytes: &[u8; 32]) -> Option<Self> {
        ed25519_dalek::VerifyingKey::from_bytes(bytes)
            .ok()
            .map(Self)
    }

    /// Returns the 32-byte compressed encoding of the public key.
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    /// Returns a reference to the 32-byte compressed encoding of the public key.
    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_bytes()
    }

    /// Verifies an Ed25519 signature. Returns `true` if the signature is valid.
    pub fn verify(&self, message: &[u8], signature: &Ed25519Signature) -> bool {
        self.0.verify(message, &signature.0).is_ok()
    }
}

impl Ed25519Signature {
    /// Constructs a signature from its 64-byte representation.
    pub fn from_bytes(bytes: &[u8; 64]) -> Self {
        Self(ed25519_dalek::Signature::from_bytes(bytes))
    }

    /// Returns the 64-byte encoding of this signature.
    pub fn to_bytes(&self) -> [u8; 64] {
        self.0.to_bytes()
    }
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
        assert_eq!(
            Ed25519SigningKey::from_seed(&SEED)
                .verifying_key()
                .to_bytes(),
            PK_BYTES
        );
    }

    #[test]
    fn signing_key_from_seed_roundtrip() {
        let sk = Ed25519SigningKey::from_seed(&SEED);
        assert_eq!(sk.verifying_key().to_bytes(), PK_BYTES);
    }

    #[test]
    fn sign_produces_known_signature() {
        let sk = Ed25519SigningKey::from_seed(&SEED);
        assert_eq!(sk.sign(b"").to_bytes(), SIG_BYTES);
    }

    #[test]
    fn verify_valid_signature() {
        let vk = Ed25519SigningKey::from_seed(&SEED).verifying_key();
        let sig = Ed25519Signature::from_bytes(&SIG_BYTES);
        assert!(vk.verify(b"", &sig));
    }

    #[test]
    fn verify_rejects_wrong_message() {
        let vk = Ed25519SigningKey::from_seed(&SEED).verifying_key();
        let sig = Ed25519Signature::from_bytes(&SIG_BYTES);
        assert!(!vk.verify(b"wrong", &sig));
    }

    #[test]
    fn verify_rejects_wrong_signature() {
        let vk = Ed25519SigningKey::from_seed(&SEED).verifying_key();
        let mut bad_bytes = SIG_BYTES;
        bad_bytes[0] ^= 0xff;
        let bad_sig = Ed25519Signature::from_bytes(&bad_bytes);
        assert!(!vk.verify(b"", &bad_sig));
    }

    #[test]
    fn verifying_key_from_bytes_roundtrip() {
        let vk = Ed25519VerifyingKey::from_bytes(&PK_BYTES).unwrap();
        assert_eq!(vk.to_bytes(), PK_BYTES);
    }

    #[test]
    fn verifying_key_from_bytes_different_seeds_differ() {
        let vk1 = Ed25519VerifyingKey::from_bytes(&PK_BYTES).unwrap();
        let vk2 = Ed25519SigningKey::from_seed(&[0x42u8; 32]).verifying_key();
        assert_ne!(vk1.to_bytes(), vk2.to_bytes());
    }

    #[test]
    fn sign_then_verify_roundtrip() {
        let seed: [u8; 32] = [0x42u8; 32];
        let sk = Ed25519SigningKey::from_seed(&seed);
        let vk = sk.verifying_key();
        let msg = b"hello multiversx";
        let sig = sk.sign(msg);
        assert!(vk.verify(msg, &sig));
        assert!(!vk.verify(b"other message", &sig));
    }
}
