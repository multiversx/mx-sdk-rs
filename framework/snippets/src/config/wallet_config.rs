use multiversx_sc_scenario::imports::Bech32Address;
use serde::Deserialize;
use std::{cell::RefCell, path::PathBuf, sync::OnceLock};

use crate::sdk::{
    test_wallets,
    wallet::{Keystore, Wallet},
};

thread_local! {
    /// Set by [`load_toml_config`] to the parent directory of the config file
    /// before deserialization begins. Consumed by [`WalletConfig`]'s `From` impl
    /// to resolve relative `pem` / `keyfile` paths automatically, with no
    /// per-config boilerplate needed.
    pub static CONFIG_BASE_DIR: RefCell<Option<PathBuf>> = RefCell::new(None);
}

/// Raw helper used only for serde deserialization.
#[derive(Deserialize)]
struct WalletConfigRaw {
    test_wallet: Option<String>,
    pem: Option<PathBuf>,
    keyfile: Option<PathBuf>,
    keystore_password: Option<String>,
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

    cache: OnceLock<Wallet>,
}

impl From<WalletConfigRaw> for WalletConfig {
    fn from(raw: WalletConfigRaw) -> Self {
        let mut config = WalletConfig {
            test_wallet: raw.test_wallet,
            pem: raw.pem,
            keyfile: raw.keyfile,
            keystore_password: raw.keystore_password,
            cache: OnceLock::new(),
        };
        CONFIG_BASE_DIR.with(|cell| {
            if let Some(ref base_dir) = *cell.borrow() {
                config.set_base_dir(base_dir);
            }
        });
        config
    }
}

impl WalletConfig {
    /// Sets the base directory used to resolve relative `pem` / `keyfile` paths
    /// during deserialization. Called by [`load_toml_config`] before parsing the
    /// config file; pass `None` to clear after parsing.
    pub(crate) fn set_config_base_dir(base_dir: Option<PathBuf>) {
        CONFIG_BASE_DIR.with(|cell| *cell.borrow_mut() = base_dir);
    }

    /// Creates a `WalletConfig` from a built-in test wallet name.
    pub fn from_test_wallet(name: impl Into<String>) -> Self {
        WalletConfig {
            test_wallet: Some(name.into()),
            pem: None,
            keyfile: None,
            keystore_password: None,
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
            cache: OnceLock::new(),
        }
    }

    /// Resolves any relative `pem` / `keyfile` paths against `base_dir`.
    ///
    /// Call this after deserializing a `WalletConfig` from a TOML/JSON file,
    /// passing the directory that contains the config file as `base_dir`.
    /// Absolute paths and `test_wallet` names are left unchanged.
    pub fn set_base_dir(&mut self, base_dir: &std::path::Path) {
        if let Some(pem) = &self.pem {
            if pem.is_relative() {
                self.pem = Some(base_dir.join(pem));
            }
        }
        if let Some(keyfile) = &self.keyfile {
            if keyfile.is_relative() {
                self.keyfile = Some(base_dir.join(keyfile));
            }
        }
    }

    /// Returns the wallet, loading and caching it on first call.
    ///
    /// Priority: `test_wallet` > `pem` > `keyfile`.
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
        } else {
            panic!("WalletConfig requires one of: `test_wallet`, `pem`, or `keyfile`")
        }
    }
}
