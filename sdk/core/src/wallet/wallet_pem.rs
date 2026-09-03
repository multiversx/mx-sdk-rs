use std::path::Path;

use anyhow::{Result, anyhow};
use multiversx_chain_core::std::Bech32Address;

use multiversx_chain_core::std::{base64_decode, base64_encode};

use super::{PrivateKey, PublicKey};

const PEM_BEGIN_PREFIX: &str = "-----BEGIN PRIVATE KEY for ";
const PEM_END_PREFIX: &str = "-----END PRIVATE KEY for ";
const PEM_MARKER_SUFFIX: &str = "-----";

/// A wallet loaded from or destined for the PEM format.
///
/// The address field carries both the 32-byte address and the HRP, so a
/// round-trip through [`WalletPem::from_pem_str`] / [`WalletPem::to_pem_str`]
/// preserves the original human-readable part (e.g. `"erd"` or a custom one).
pub struct WalletPem {
    pub private_key: PrivateKey,
    pub address: Bech32Address,
}

impl WalletPem {
    /// Parses a PEM string into a `WalletPem`.
    ///
    /// The first line must be `-----BEGIN PRIVATE KEY for {bech32}-----`, which
    /// supplies the address together with its HRP. The base64-encoded body
    /// contains the concatenated private-key hex and public-key hex.
    pub fn from_pem_str(pem_str: &str) -> Result<Self> {
        let mut lines = pem_str.lines();

        let header = lines.next().ok_or_else(|| anyhow!("PEM string is empty"))?;
        let bech32_str = header
            .strip_prefix(PEM_BEGIN_PREFIX)
            .and_then(|s| s.strip_suffix(PEM_MARKER_SUFFIX))
            .ok_or_else(|| anyhow!("invalid PEM header: {header}"))?;
        let address = Bech32Address::from_bech32_str(bech32_str);

        let b64_body: String = lines
            .take_while(|line| !line.starts_with(PEM_END_PREFIX))
            .collect();

        let decoded =
            base64_decode(b64_body).map_err(|e| anyhow!("invalid base64 in PEM body: {e}"))?;
        let private_key_bytes = &decoded[..decoded.len() / 2];
        let private_key_str = std::str::from_utf8(private_key_bytes)
            .map_err(|e| anyhow!("invalid UTF-8 in private key: {e}"))?;
        let private_key = PrivateKey::from_seed_hex_str(private_key_str)?;

        Ok(WalletPem {
            private_key,
            address,
        })
    }

    /// Reads a PEM file from disk and parses it with [`WalletPem::from_pem_str`].
    pub fn from_pem_file<P>(file_path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let contents = std::fs::read_to_string(file_path)?;
        Self::from_pem_str(&contents)
    }

    /// Produces a PEM string from this `WalletPem`.
    ///
    /// Output format:
    /// ```text
    /// -----BEGIN PRIVATE KEY for {bech32}-----
    /// {base64(private_key_hex + public_key_hex)}
    /// -----END PRIVATE KEY for {bech32}-----
    /// ```
    pub fn to_pem_str(&self) -> String {
        let private_key = self.private_key_hex();
        let public_key = self.public_key_hex();
        let b64 = base64_encode(format!("{private_key}{public_key}"));

        let formatted_key = b64
            .as_bytes()
            .chunks(64)
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect::<Vec<_>>()
            .join("\n");

        let bech32 = self.address.to_bech32_str();
        format!(
            "{PEM_BEGIN_PREFIX}{bech32}{PEM_MARKER_SUFFIX}\n{formatted_key}\n{PEM_END_PREFIX}{bech32}{PEM_MARKER_SUFFIX}\n"
        )
    }

    pub fn private_key_hex(&self) -> String {
        self.private_key.to_seed_hex()
    }

    pub fn public_key_hex(&self) -> String {
        PublicKey::from(&self.private_key).to_hex()
    }
}
