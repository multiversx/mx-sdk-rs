use std::path::PathBuf;

use crate::sdk::wallet::Wallet;

use super::ConnectionConfig;

/// Trait implemented by a contract-specific `Config` struct to provide initialization
/// details to the interactor.
///
/// The `InteractorBase` calls `connection()` to configure the gateway and calls
/// `register_wallets()` to register every wallet that will sign transactions.
pub trait InteractorConfig {
    /// Returns the connection settings (gateway URI, chain type).
    fn connection(&self) -> &ConnectionConfig;

    /// Returns the wallets to be registered with the interactor during setup.
    ///
    /// Optional — defaults to no wallets. Additional wallets can always be
    /// registered manually on the interactor after construction.
    fn register_wallets(&self) -> Vec<Wallet> {
        Vec::new()
    }
}

/// A bare [`ConnectionConfig`] can be used directly as an [`InteractorConfig`]
/// when no wallets or custom directory are needed.
impl InteractorConfig for ConnectionConfig {
    fn connection(&self) -> &ConnectionConfig {
        self
    }
}

/// Deserializes a TOML config file into a value of type `C`.
///
/// Before parsing, sets the config file's parent directory as the base for
/// resolving relative [`WalletConfig`] paths (see [`WalletConfig::set_config_base_dir`]).
/// The base directory is cleared after parsing.
///
/// # Panics
///
/// Panics if the file cannot be opened, read, or parsed as valid TOML for `C`.
pub fn load_toml_config<C>(config_path: &PathBuf) -> C
where
    C: InteractorConfig + serde::de::DeserializeOwned,
{
    use super::wallet_config::WalletConfig;
    use std::io::Read;

    let base_dir = config_path.parent().map(|p| p.to_path_buf());
    WalletConfig::set_config_base_dir(base_dir);

    let mut file = std::fs::File::open(config_path)
        .unwrap_or_else(|e| panic!("cannot open {}: {e}", config_path.display()));
    let mut content = String::new();
    file.read_to_string(&mut content)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", config_path.display()));
    let result = toml::from_str(&content)
        .unwrap_or_else(|e| panic!("cannot parse {}: {e}", config_path.display()));

    WalletConfig::set_config_base_dir(None);

    result
}
