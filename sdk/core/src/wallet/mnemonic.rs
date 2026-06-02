use std::fmt;

use anyhow::Result;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha512;

use super::private_key::PrivateKey;

const EGLD_COIN_TYPE: u32 = 508;
const HARDENED: u32 = 0x80000000;

/// A BIP-39 mnemonic used to derive [`PrivateKey`]s via the MultiversX HD path.
///
/// Wraps [`bip39::Mnemonic`] and provides MultiversX-specific key derivation.
#[derive(Clone)]
pub struct Mnemonic(bip39::Mnemonic);

impl Mnemonic {
    /// Parses a mnemonic phrase.
    ///
    /// Embedded newlines are stripped before parsing, so file contents can be
    /// passed directly without pre-processing.
    pub fn parse(s: &str) -> Result<Self> {
        Ok(Mnemonic(bip39::Mnemonic::parse(
            s.replace('\n', "").trim(),
        )?))
    }

    /// Derives a [`PrivateKey`] using the MultiversX HD path
    /// `m/44'/508'/<account>'/0'/<address_index>'`.
    pub fn to_private_key(&self, account: u32, address_index: u32) -> PrivateKey {
        let seed = self.to_bip39_seed();
        self.bip32_derive(&seed, account, address_index)
    }

    fn to_bip39_seed(&self) -> [u8; 64] {
        self.0.to_seed_normalized("")
    }

    fn bip32_derive(&self, seed: &[u8; 64], account: u32, address_index: u32) -> PrivateKey {
        let serialized_key_len = 32;
        let hardened_child_padding: u8 = 0;

        let mut digest =
            Hmac::<Sha512>::new_from_slice(b"ed25519 seed").expect("HMAC can take key of any size");
        digest.update(seed);
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

        let seed_bytes: &[u8; 32] = key
            .as_slice()
            .try_into()
            .expect("BIP32 derived key has unexpected length");
        PrivateKey::from_seed_bytes(seed_bytes)
    }
}

impl From<bip39::Mnemonic> for Mnemonic {
    fn from(m: bip39::Mnemonic) -> Self {
        Mnemonic(m)
    }
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
