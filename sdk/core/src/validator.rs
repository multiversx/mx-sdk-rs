use core::str;

use anyhow::{Result, anyhow};
use multiversx_chain_core::types::BLSKey;

/// A validator (node) key, as loaded from a validator PEM file.
///
/// The PEM label holds the BLS public key (hex), the content holds the BLS secret key (hex).
#[derive(Clone, Debug)]
pub struct Validator {
    /// The PEM content: the BLS secret key, as a hex string (ASCII bytes).
    pub private_key: Vec<u8>,
    pub public_key: BLSKey,
}

impl Validator {
    /// Loads the first validator key from a PEM file.
    pub fn from_pem_file(file_path: &str) -> Result<Self> {
        let contents = std::fs::read_to_string(file_path)?;
        Self::from_pem_file_contents(contents)
    }

    /// Loads the first validator key from PEM contents.
    pub fn from_pem_file_contents(contents: String) -> Result<Self> {
        let pem = pem::parse(contents).map_err(|e| anyhow!("failed to parse PEM: {e}"))?;
        Self::from_pem(&pem)
    }

    /// Loads all validator keys from a PEM file. Validators PEM files may contain several keys.
    pub fn from_pem_file_all(file_path: &str) -> Result<Vec<Self>> {
        let contents = std::fs::read_to_string(file_path)?;
        Self::from_pem_file_contents_all(&contents)
    }

    /// Loads all validator keys from PEM contents.
    pub fn from_pem_file_contents_all(contents: &str) -> Result<Vec<Self>> {
        let pems = pem::parse_many(contents).map_err(|e| anyhow!("failed to parse PEM: {e}"))?;
        if pems.is_empty() {
            return Err(anyhow!("no validator keys found in PEM"));
        }
        pems.iter().map(Self::from_pem).collect()
    }

    fn from_pem(pem: &pem::Pem) -> Result<Self> {
        let public_key_str = pem
            .tag()
            .rsplit(' ')
            .next()
            .ok_or_else(|| anyhow!("failed to extract public key from PEM label"))?;

        let public_key = hex::decode(public_key_str)?;
        let private_key = pem.contents();

        Ok(Validator {
            private_key: private_key.to_vec(),
            public_key: BLSKey::from_vec(public_key)
                .ok_or_else(|| anyhow!("bad BLS public key length in PEM label"))?,
        })
    }

    /// The BLS secret key bytes, decoded from the hex PEM content.
    pub fn secret_key_bytes(&self) -> Result<Vec<u8>> {
        let hex_str = str::from_utf8(&self.private_key)?;
        Ok(hex::decode(hex_str.trim())?)
    }
}

#[cfg(feature = "bls")]
mod bls_sign {
    use std::sync::Mutex;

    use anyhow::{Result, anyhow};
    use multiversx_bls::SecretKey;
    use multiversx_chain_core::types::BLSSignature;

    use super::Validator;

    /// The BLS library is not guaranteed to be thread-safe.
    static BLS_MUTEX: Mutex<()> = Mutex::new(());

    impl Validator {
        /// Signs a message with the BLS secret key.
        ///
        /// The validator system SC expects each BLS key to sign the address of the staker
        /// (the sender of `stake`, or the delegation contract for `addNodes`).
        ///
        /// Fails if the secret key does not correspond to the public key in the PEM label.
        pub fn sign(&self, message: &[u8]) -> Result<BLSSignature> {
            let secret_key_bytes = self.secret_key_bytes()?;

            let _guard = BLS_MUTEX
                .lock()
                .map_err(|_| anyhow!("BLS mutex poisoned"))?;

            let secret_key = SecretKey::from_serialized(&secret_key_bytes)
                .map_err(|e| anyhow!("invalid BLS secret key: {e:?}"))?;

            let public_key = secret_key
                .get_public_key()
                .serialize()
                .map_err(|e| anyhow!("cannot serialize BLS public key: {e:?}"))?;
            if public_key != self.public_key.to_vec() {
                return Err(anyhow!(
                    "BLS secret key does not correspond to public key {}",
                    hex::encode(self.public_key.to_vec())
                ));
            }

            let signature = secret_key
                .sign(message)
                .serialize()
                .map_err(|e| anyhow!("cannot serialize BLS signature: {e:?}"))?;
            BLSSignature::from_vec(signature).ok_or_else(|| anyhow!("bad BLS signature length"))
        }
    }
}
