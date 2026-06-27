use core::str;
use std::path::Path;

use anyhow::Result;
use multiversx_chain_core::{
    std::{Bech32Address, Bech32Hrp, crypto},
    types::Address,
};
use serde_json::json;

use crate::{
    data::transaction::Transaction,
    wallet::{Mnemonic, PrivateKey, PublicKey, WalletPem, WalletSignature, WalletSource},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Wallet {
    pub private_key: PrivateKey,
    pub address: Address,
    pub source: WalletSource,
}

impl Wallet {
    pub fn new(private_key: PrivateKey, source: WalletSource) -> Self {
        let address = PublicKey::from(&private_key).to_address();
        Wallet {
            private_key,
            address,
            source,
        }
    }
}

impl From<WalletPem> for Wallet {
    fn from(wallet_pem: WalletPem) -> Self {
        Self::new(
            wallet_pem.private_key,
            WalletSource::PemFile(wallet_pem.address.hrp),
        )
    }
}

impl From<PrivateKey> for Wallet {
    fn from(private_key: PrivateKey) -> Self {
        Self::new(private_key, WalletSource::PrivateKey)
    }
}

impl TryFrom<Mnemonic> for Wallet {
    type Error = anyhow::Error;

    /// Derives the wallet at account 0, address index 0 from the mnemonic.
    fn try_from(mnemonic: Mnemonic) -> Result<Self> {
        let private_key = mnemonic.to_private_key(0, 0)?;
        Ok(Self::new(private_key, WalletSource::Mnemonic))
    }
}

impl Wallet {
    #[deprecated(
        since = "0.67.0",
        note = "Use `PrivateKey::from_hex_str(hex).map(Wallet::from)` instead"
    )]
    pub fn from_private_key_hex(priv_key: &str) -> Result<Self> {
        let private_key = PrivateKey::from_hex_str(priv_key)?;
        Ok(Self::new(private_key, WalletSource::PrivateKey))
    }

    pub fn from_pem_file<P>(file_path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(WalletPem::from_pem_file(file_path)?.into())
    }

    pub(crate) fn new_test_wallet(name: &'static str, pem: &str) -> Self {
        let wallet_pem = WalletPem::from_pem_str(pem).unwrap();
        Self::new(wallet_pem.private_key, WalletSource::TestWallet(name))
    }

    #[deprecated(
        since = "0.54.0",
        note = "Renamed to `to_address`, type changed to multiversx_chain_core::types::Address"
    )]
    pub fn address(&self) -> Bech32Address {
        self.to_address().to_bech32_default()
    }

    pub fn to_address(&self) -> Address {
        self.address.clone()
    }

    /// Returns the address as a [`Bech32Address`], using the HRP from the wallet source
    /// (`PemFile` or `Keystore`) when available, and the default HRP (`"erd"`) otherwise.
    pub fn to_bech32(&self) -> Bech32Address {
        let hrp = match &self.source {
            WalletSource::PemFile(hrp) | WalletSource::Keystore(hrp) => *hrp,
            _ => Bech32Hrp::default(),
        };
        Bech32Address::encode_address(hrp, self.address.clone())
    }

    pub fn private_key_hex(&self) -> String {
        self.private_key.to_seed_hex()
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey::from(&self.private_key)
    }

    pub fn public_key_hex(&self) -> String {
        PublicKey::from(&self.private_key).to_hex()
    }

    pub fn sign_tx(&self, unsign_tx: &Transaction) -> WalletSignature {
        let mut unsign_tx = unsign_tx.clone();
        unsign_tx.signature = None;

        let mut tx_bytes = json!(unsign_tx).to_string().as_bytes().to_vec();

        let should_sign_on_tx_hash = unsign_tx.version >= 2 && unsign_tx.options & 1 > 0;
        if should_sign_on_tx_hash {
            tx_bytes = crypto::keccak256(&tx_bytes).to_vec();
        }

        self.private_key.sign(tx_bytes)
    }

    pub fn sign_bytes(&self, data: impl AsRef<[u8]>) -> WalletSignature {
        self.private_key.sign(data)
    }

    pub fn to_pem(&self, hrp: Bech32Hrp) -> WalletPem {
        WalletPem {
            private_key: self.private_key.clone(),
            address: Bech32Address::encode_address(hrp, self.address.clone()),
        }
    }
}
