use std::fmt::Display;

use super::edwards25519::{sc_mul_add, sc_reduce};
use crate::crypto::edwards25519::extended_group_element::ExtendedGroupElement;
use anyhow::{Result, anyhow};
use bip39::Mnemonic;
use hmac::{Hmac, KeyInit, Mac};
use pbkdf2::pbkdf2;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};
use sha2::{Digest, Sha512};
use zeroize::Zeroize;

pub const PRIVATE_KEY_LENGTH: usize = 64;
pub const SIGNATURE_LENGTH: usize = 64;
pub const SEED_LENGTH: usize = 32;

const EGLD_COIN_TYPE: u32 = 508;
const HARDENED: u32 = 0x80000000;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct PrivateKey(pub [u8; PRIVATE_KEY_LENGTH]);

impl PrivateKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<PrivateKey> {
        match bytes.len() {
            SEED_LENGTH => {
                let mut h: Sha512 = Sha512::new();
                let mut hash: [u8; 64] = [0u8; 64];
                let mut digest: [u8; 32] = [0u8; 32];

                h.update(bytes);
                hash.copy_from_slice(h.finalize().as_ref());

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
            }
            PRIVATE_KEY_LENGTH => {
                let mut bits: [u8; 64] = [0u8; 64];
                bits.copy_from_slice(&bytes[..64]);

                Ok(PrivateKey(bits))
            }
            _ => Err(anyhow!("Invalid secret key length")),
        }
    }

    pub fn from_hex_str(pk: &str) -> Result<Self> {
        let bytes = hex::decode(pk)?;
        PrivateKey::from_bytes(bytes.as_slice())
    }

    fn seed_from_mnemonic(mnemonic: Mnemonic, password: &str) -> [u8; 64] {
        let mut salt = String::with_capacity(8 + password.len());
        salt.push_str("mnemonic");
        salt.push_str(password);

        let mut seed = [0u8; 64];

        let _ = pbkdf2::<Hmac<Sha512>>(
            mnemonic.to_string().as_bytes(),
            salt.as_bytes(),
            2048,
            &mut seed,
        );

        salt.zeroize();

        seed
    }

    pub fn from_mnemonic(mnemonic: Mnemonic, account: u32, address_index: u32) -> PrivateKey {
        let seed = Self::seed_from_mnemonic(mnemonic, "");

        let serialized_key_len = 32;
        let hardened_child_padding: u8 = 0;

        let mut digest =
            Hmac::<Sha512>::new_from_slice(b"ed25519 seed").expect("HMAC can take key of any size");
        digest.update(&seed);
        let intermediary: Vec<u8> = digest.finalize().into_bytes().into_iter().collect();
        let mut key = intermediary[..serialized_key_len].to_vec();
        let mut chain_code = intermediary[serialized_key_len..].to_vec();

        for child_idx in [
            44 | HARDENED,
            EGLD_COIN_TYPE | HARDENED,
            account | HARDENED,
            HARDENED,
            address_index | HARDENED,
        ] {
            let mut buff = [vec![hardened_child_padding], key.clone()].concat();
            buff.push((child_idx >> 24) as u8);
            buff.push((child_idx >> 16) as u8);
            buff.push((child_idx >> 8) as u8);
            buff.push(child_idx as u8);

            digest =
                Hmac::<Sha512>::new_from_slice(&chain_code).expect("HMAC can take key of any size");
            digest.update(&buff);
            let intermediary: Vec<u8> = digest.finalize().into_bytes().into_iter().collect();
            key = intermediary[..serialized_key_len].to_vec();
            chain_code = intermediary[serialized_key_len..].to_vec();
        }

        PrivateKey::from_bytes(key.as_slice()).unwrap()
    }

    pub fn to_bytes(&self) -> [u8; PRIVATE_KEY_LENGTH] {
        self.0
    }

    pub fn as_bytes(&self) -> &[u8; PRIVATE_KEY_LENGTH] {
        &self.0
    }

    pub fn to_hex(&self) -> String {
        hex::encode(&self.0[..32])
    }

    pub fn sign(&self, message: Vec<u8>) -> [u8; 64] {
        let mut h: Sha512 = Sha512::new();
        h.update(&self.0[..32]);

        let mut digest1 = [0u8; 64];
        let mut message_digest = [0u8; 64];
        let mut hram_digest = [0u8; 64];
        let mut expanded_secret_key = [0u8; 32];

        digest1.copy_from_slice(h.finalize_reset().as_ref());
        expanded_secret_key.copy_from_slice(&digest1[..32]);
        expanded_secret_key[0] &= 248;
        expanded_secret_key[31] &= 63;
        expanded_secret_key[31] |= 64;

        h.update(&digest1[32..]);
        h.update(&message);
        message_digest.copy_from_slice(h.finalize_reset().as_ref());

        let message_digest_reduced = sc_reduce(message_digest);
        let mut r = ExtendedGroupElement::default();
        r.ge_scalar_mult_base(message_digest_reduced);

        let encoded_r = r.to_bytes();

        h.update(encoded_r);
        h.update(&self.0[32..]);
        h.update(&message);
        hram_digest.copy_from_slice(h.finalize_reset().as_ref());

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

impl Display for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_hex().fmt(f)
    }
}

impl std::fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PrivateKey({})", self)
    }
}

impl Serialize for PrivateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_hex().as_str())
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
