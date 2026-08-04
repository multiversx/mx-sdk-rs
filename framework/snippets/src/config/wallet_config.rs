use multiversx_sc_scenario::imports::Bech32Address;
use serde::Deserialize;
use std::{path::PathBuf, sync::OnceLock};

use crate::sdk::{
    test_wallets,
    wallet::{Keystore, Wallet},
};

use super::ConfigPath;

/// Raw helper used only for serde deserialization.
#[derive(Deserialize)]
struct WalletConfigRaw {
    test_wallet: Option<String>,
    pem: Option<ConfigPath>,
    keyfile: Option<ConfigPath>,
    keystore_password: Option<String>,
    #[serde(default)]
    ledger: bool,
    #[serde(default)]
    ledger_address_index: u32,
}

/// Wallet configuration embeddable in a TOML/JSON config file.
/// Mirrors `SenderArgs` from the CLI but uses `serde` instead of `clap`.
#[derive(Debug, Deserialize)]
#[serde(from = "WalletConfigRaw")]
pub struct WalletConfig {
    /// Name of a built-in test wallet (e.g. "alice", "bob", "mike").
    /// See `multiversx_sdk::test_wallets::valid_names()` for the full list.
    pub test_wallet: Option<String>,

    /// Path to a PEM wallet file.
    pub pem: Option<PathBuf>,

    /// Path to a JSON keystore wallet file.
    pub keyfile: Option<PathBuf>,

    /// Keystore password (plain text). Required when `keyfile` is set.
    pub keystore_password: Option<String>,

    /// Use the Ledger hardware device for signing.
    pub ledger: bool,

    /// Address index to use on the Ledger device (default: 0).
    pub ledger_address_index: u32,

    cache: OnceLock<Wallet>,
}

impl From<WalletConfigRaw> for WalletConfig {
    fn from(raw: WalletConfigRaw) -> Self {
        WalletConfig {
            test_wallet: raw.test_wallet,
            pem: raw.pem.map(Into::into),
            keyfile: raw.keyfile.map(Into::into),
            keystore_password: raw.keystore_password,
            ledger: raw.ledger,
            ledger_address_index: raw.ledger_address_index,
            cache: OnceLock::new(),
        }
    }
}

impl WalletConfig {
    /// Creates a `WalletConfig` from a built-in test wallet name.
    pub fn from_test_wallet(name: impl Into<String>) -> Self {
        WalletConfig {
            test_wallet: Some(name.into()),
            pem: None,
            keyfile: None,
            keystore_password: None,
            ledger: false,
            ledger_address_index: 0,
            cache: OnceLock::new(),
        }
    }

    /// Creates a `WalletConfig` from a PEM file path.
    pub fn from_pem(path: impl Into<PathBuf>) -> Self {
        WalletConfig {
            test_wallet: None,
            pem: Some(path.into()),
            keyfile: None,
            keystore_password: None,
            ledger: false,
            ledger_address_index: 0,
            cache: OnceLock::new(),
        }
    }

    /// Returns the wallet, loading and caching it on first call.
    ///
    /// Priority: `test_wallet` > `pem` > `keyfile` > `ledger`.
    /// Panics if none of the sources are set, or if loading fails.
    pub fn wallet(&self) -> &Wallet {
        self.cache.get_or_init(|| self.load_wallet())
    }

    /// Returns the on-chain address derived from this wallet config.
    pub fn address(&self) -> Bech32Address {
        self.wallet().to_bech32()
    }

    fn load_wallet(&self) -> Wallet {
        if let Some(name) = &self.test_wallet {
            test_wallets::by_name(name.as_str())
                .unwrap_or_else(|| panic!("unknown test wallet name: '{name}'"))
        } else if let Some(pem) = &self.pem {
            Wallet::from_pem_file(pem).expect("failed to load PEM wallet")
        } else if let Some(keyfile) = &self.keyfile {
            let password = self
                .keystore_password
                .as_deref()
                .expect("keystore_password is required when using keyfile");
            let keystore = Keystore::from_file(keyfile).expect("failed to load keystore file");
            keystore
                .decrypt_wallet(password)
                .expect("failed to decrypt wallet")
        } else if self.ledger {
            self.load_ledger_wallet()
        } else {
            panic!("WalletConfig requires one of: `test_wallet`, `pem`, `keyfile`, or `ledger`")
        }
    }

    #[cfg(feature = "ledger")]
    fn load_ledger_wallet(&self) -> Wallet {
        use crate::sdk::wallet::ledger::LedgerApp;
        use multiversx_sdk::chain_core::std::Bech32Hrp;

        let mut app = LedgerApp::new().expect("failed to connect to Ledger device");
        let bech32_str = app
            .get_address(self.ledger_address_index)
            .expect("failed to get address from Ledger");
        let bech32_addr =
            multiversx_sdk::chain_core::std::Bech32Address::from_bech32_string(bech32_str);
        let hrp: Bech32Hrp = bech32_addr.hrp;
        let address = bech32_addr.address;
        Wallet::new_ledger(address, hrp, self.ledger_address_index)
    }

    #[cfg(not(feature = "ledger"))]
    fn load_ledger_wallet(&self) -> Wallet {
        panic!("Ledger support is not compiled in; rebuild with the `ledger` feature enabled")
    }
}
