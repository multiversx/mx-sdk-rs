use super::edwards25519::{sc_mul_add, sc_reduce};
use crate::crypto::edwards25519::extended_group_element::ExtendedGroupElement;
use anyhow::{anyhow, Result};
use rand::{CryptoRng, RngCore};
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};
use sha2::{Digest, Sha512};

pub const PRIVATE_KEY_LENGTH: usize = 64;
pub const SIGNATURE_LENGTH: usize = 64;
pub const SEED_LENGTH: usize = 32;

#[derive(Copy, Clone, Debug)]
pub struct PrivateKey(pub [u8; PRIVATE_KEY_LENGTH]);

impl PrivateKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<PrivateKey> {
        match bytes.len() {
            SEED_LENGTH => {
                let mut h: Sha512 = Sha512::new();
                let mut hash: [u8; 64] = [0u8; 64];
                let mut digest: [u8; 32] = [0u8; 32];

                h.update(bytes);
                hash.copy_from_slice(h.finalize().as_slice());

                digest.copy_from_slice(&hash[..32]);

                digest[0] &= 248;
                digest[31] &= 127;
                digest[31] |= 64;

                let mut a = ExtendedGroupElement::default();
                a.ge_scalar_mult_base(digest);
                let public_key_bytes = a.to_bytes();

                let merge: Vec<u8> = [bytes.to_vec(), public_key_bytes.to_vec()]
                    .concat()
                    .into_iter()
                    .collect();
                let mut bits: [u8; 64] = [0u8; 64];
                bits.copy_from_slice(&merge[..64]);

                Ok(PrivateKey(bits))
            },
            PRIVATE_KEY_LENGTH => {
                let mut bits: [u8; 64] = [0u8; 64];
                bits.copy_from_slice(&bytes[..64]);

                Ok(PrivateKey(bits))
            },
            _ => Err(anyhow!("Invalid secret key length")),
        }
    }

    pub fn from_hex_str(pk: &str) -> Result<Self> {
        let bytes = hex::decode(pk)?;
        PrivateKey::from_bytes(bytes.as_slice())
    }

    pub fn generate<T>(r: &mut T) -> PrivateKey
    where
        T: CryptoRng + RngCore,
    {
        let mut secret_key = PrivateKey([0u8; 64]);

        r.fill_bytes(&mut secret_key.0);

        secret_key
    }

    pub fn to_bytes(&self) -> [u8; PRIVATE_KEY_LENGTH] {
        self.0
    }

    pub fn as_bytes(&self) -> &[u8; PRIVATE_KEY_LENGTH] {
        &self.0
    }

    pub fn sign(&self, message: Vec<u8>) -> [u8; 64] {
        let mut h: Sha512 = Sha512::new();
        h.update(&self.0[..32]);

        let mut digest1 = [0u8; 64];
        let mut message_digest = [0u8; 64];
        let mut hram_digest = [0u8; 64];
        let mut expanded_secret_key = [0u8; 32];

        digest1.copy_from_slice(h.finalize_reset().as_slice());
        expanded_secret_key.copy_from_slice(&digest1[..32]);
        expanded_secret_key[0] &= 248;
        expanded_secret_key[31] &= 63;
        expanded_secret_key[31] |= 64;

        h.update(&digest1[32..]);
        h.update(&message);
        message_digest.copy_from_slice(h.finalize_reset().as_slice());

        let message_digest_reduced = sc_reduce(message_digest);
        let mut r = ExtendedGroupElement::default();
        r.ge_scalar_mult_base(message_digest_reduced);

        let encoded_r = r.to_bytes();

        h.update(encoded_r);
        h.update(&self.0[32..]);
        h.update(&message);
        hram_digest.copy_from_slice(h.finalize_reset().as_slice());

        let hram_digest_reduced = sc_reduce(hram_digest);

        let s = sc_mul_add(
            hram_digest_reduced,
            expanded_secret_key,
            message_digest_reduced,
        );

        let mut signature = [0u8; 64];

        signature[..32].copy_from_slice(&encoded_r);
        signature[32..].copy_from_slice(&s);

        signature
    }
}

impl ToString for PrivateKey {
    fn to_string(&self) -> String {
        hex::encode(&self.0[..32])
    }
}

impl Serialize for PrivateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for PrivateKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_hex_str(s.as_str()).unwrap())
    }
}
