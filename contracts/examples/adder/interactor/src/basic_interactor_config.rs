use multiversx_sc_snippets::{ConnectionConfig, InteractorConfig, WalletConfig, imports::Wallet};
use serde::Deserialize;

/// Adder Interact configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    pub contract_path: String,
    pub connection: ConnectionConfig,
    pub owner: WalletConfig,
    pub wallet: WalletConfig,
}

impl InteractorConfig for Config {
    fn connection(&self) -> &ConnectionConfig {
        &self.connection
    }

    fn register_wallets(&self) -> Vec<Wallet> {
        vec![self.owner.wallet().clone(), self.wallet.wallet().clone()]
    }
}

impl Config {}
