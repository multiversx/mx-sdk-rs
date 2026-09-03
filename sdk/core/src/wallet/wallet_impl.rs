use core::str;
use std::path::Path;

use anyhow::{Result, anyhow};
use multiversx_chain_core::{
    std::{Bech32Address, Bech32Hrp, crypto},
    types::Address,
};
use serde_json::json;

use crate::{
    data::transaction::Transaction,
    wallet::{Mnemonic, PrivateKey, PublicKey, Signer, WalletPem, WalletSignature, WalletSource},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Wallet {
    pub signer: Signer,
    pub address: Address,
    pub source: WalletSource,
}

impl Wallet {
    /// Creates a wallet backed by a local private key.
    /// The on-chain address is derived automatically from the key.
    pub fn new(private_key: PrivateKey, source: WalletSource) -> Self {
        let address = PublicKey::from(&private_key).to_address();
        Wallet {
            signer: Signer::PrivateKey(Box::new(private_key)),
            address,
            source,
        }
    }

    /// Creates a wallet backed by a Ledger hardware device.
    ///
    /// The `address` must already be fetched from the device (via
    /// [`LedgerApp::get_address`]); it is stored as-is and used for
    /// transaction-sender validation.
    pub fn new_ledger(address: Address, hrp: Bech32Hrp, address_index: u32) -> Self {
        Wallet {
            signer: Signer::Ledger { address_index },
            address,
            source: WalletSource::Ledger { address_index, hrp },
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

    /// Returns the address as a [`Bech32Address`], using the HRP from the wallet
    /// source (`PemFile`, `Keystore`, or `Ledger`) when available, and the
    /// default HRP (`"erd"`) otherwise.
    pub fn to_bech32(&self) -> Bech32Address {
        let hrp = match &self.source {
            WalletSource::PemFile(hrp)
            | WalletSource::Keystore(hrp)
            | WalletSource::Ledger { hrp, .. } => *hrp,
            _ => Bech32Hrp::default(),
        };
        Bech32Address::encode_address(hrp, self.address.clone())
    }

    /// Returns a reference to the private key, or `None` for Ledger wallets.
    pub fn private_key(&self) -> Option<&PrivateKey> {
        match &self.signer {
            Signer::PrivateKey(pk) => Some(pk),
            Signer::Ledger { .. } => None,
        }
    }

    /// Returns the private key as a hex-encoded seed string, or `None` for Ledger wallets.
    pub fn private_key_hex(&self) -> Option<String> {
        self.private_key().map(|pk| pk.to_seed_hex())
    }

    /// Returns the public key derived from the private key, or `None` for Ledger wallets.
    pub fn public_key(&self) -> Option<PublicKey> {
        self.private_key().map(PublicKey::from)
    }

    /// Returns the public key as a hex string, or `None` for Ledger wallets.
    pub fn public_key_hex(&self) -> Option<String> {
        self.public_key().map(|pk| pk.to_hex())
    }

    /// Signs a transaction.
    ///
    /// For `PrivateKey` wallets the signing is local and infallible (barring
    /// bugs). For `Ledger` wallets the device must be connected and the user
    /// must confirm on-screen; errors are returned as `Err`.
    pub fn sign_tx(&self, unsign_tx: &Transaction) -> Result<WalletSignature> {
        let mut unsign_tx = unsign_tx.clone();
        unsign_tx.signature = None;
        unsign_tx.relayer_signature = None;

        match &self.signer {
            Signer::PrivateKey(pk) => {
                let mut tx_bytes = json!(unsign_tx).to_string().into_bytes();
                if unsign_tx.should_sign_with_hash() {
                    tx_bytes = crypto::keccak256(&tx_bytes).to_vec();
                }
                Ok(pk.sign(tx_bytes))
            }
            Signer::Ledger { address_index } => ledger_sign_tx(*address_index, &unsign_tx),
        }
    }

    /// Signs arbitrary bytes.
    ///
    /// For `Ledger` wallets the message bytes are prefixed with a 4-byte
    /// big-endian length before being sent to the device (matching the Python SDK).
    pub fn sign_bytes(&self, data: impl AsRef<[u8]>) -> Result<WalletSignature> {
        match &self.signer {
            Signer::PrivateKey(pk) => Ok(pk.sign(data)),
            Signer::Ledger { address_index } => ledger_sign_bytes(*address_index, data),
        }
    }

    /// Converts this wallet to a PEM representation.
    /// Returns an error for Ledger wallets (the private key is never exported).
    pub fn to_pem(&self, hrp: Bech32Hrp) -> Result<WalletPem> {
        let pk = self.private_key().ok_or_else(|| {
            anyhow!("cannot export a Ledger wallet to PEM: the private key is not available")
        })?;
        Ok(WalletPem {
            private_key: pk.clone(),
            address: Bech32Address::encode_address(hrp, self.address.clone()),
        })
    }
}

#[cfg(feature = "ledger")]
fn ledger_sign_tx(address_index: u32, tx: &Transaction) -> Result<WalletSignature> {
    use crate::wallet::ledger::LedgerApp;
    use anyhow::Context as _;

    if !tx.should_sign_with_hash() {
        return Err(anyhow!(
            "Ledger signing requires hash-based options: \
             set version to V2 and options to SIGN_WITH_HASH"
        ));
    }

    let tx_bytes = json!(tx).to_string().into_bytes();
    let mut app = LedgerApp::new()
        .context("failed to open Ledger device — check that it is plugged in, unlocked, and the MultiversX app is open")?;
    app.set_address(address_index)
        .with_context(|| format!("failed to set Ledger address index {address_index}"))?;
    let sig_bytes = app
        .sign_transaction(&tx_bytes)
        .context("Ledger rejected the transaction — confirm it on the device screen or check that the transaction is valid")?;
    Ok(WalletSignature::from_bytes(sig_bytes))
}

#[cfg(not(feature = "ledger"))]
fn ledger_sign_tx(_address_index: u32, _tx: &Transaction) -> Result<WalletSignature> {
    Err(anyhow!(
        "Ledger signing is not available; recompile with the `ledger` feature enabled"
    ))
}

#[cfg(feature = "ledger")]
fn ledger_sign_bytes(address_index: u32, data: impl AsRef<[u8]>) -> Result<WalletSignature> {
    use crate::wallet::ledger::LedgerApp;

    let data_ref = data.as_ref();
    let len_prefix = (data_ref.len() as u32).to_be_bytes();
    let mut msg_bytes = Vec::with_capacity(4 + data_ref.len());
    msg_bytes.extend_from_slice(&len_prefix);
    msg_bytes.extend_from_slice(data_ref);

    let mut app = LedgerApp::new().map_err(|e| anyhow!("{e}"))?;
    app.set_address(address_index).map_err(|e| anyhow!("{e}"))?;
    let sig_bytes = app.sign_message(&msg_bytes).map_err(|e| anyhow!("{e}"))?;
    Ok(WalletSignature::from_bytes(sig_bytes))
}

#[cfg(not(feature = "ledger"))]
fn ledger_sign_bytes(_address_index: u32, _data: impl AsRef<[u8]>) -> Result<WalletSignature> {
    Err(anyhow!(
        "Ledger signing is not available; recompile with the `ledger` feature enabled"
    ))
}
