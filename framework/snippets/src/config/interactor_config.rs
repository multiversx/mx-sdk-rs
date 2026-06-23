use std::path::PathBuf;

use crate::sdk::wallet::Wallet;

use super::ConnectionConfig;

/// Trait implemented by a contract-specific `Config` struct to tell the
/// [`HttpInteractorBuilder`](crate::HttpInteractorBuilder) how to initialise the interactor.
///
/// The builder calls `connection()` to open the gateway and calls `register_wallets()` to
/// register every wallet that will sign transactions.
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

pub fn load_toml_config<C>(config_path: &PathBuf) -> C
where
    C: InteractorConfig + serde::de::DeserializeOwned,
{
    let mut file = std::fs::File::open(config_path)
        .unwrap_or_else(|e| panic!("cannot open {}: {e}", config_path.display()));
    let mut content = String::new();
    use std::io::Read;
    file.read_to_string(&mut content)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", config_path.display()));
    toml::from_str(&content)
        .unwrap_or_else(|e| panic!("cannot parse {}: {e}", config_path.display()))
}
