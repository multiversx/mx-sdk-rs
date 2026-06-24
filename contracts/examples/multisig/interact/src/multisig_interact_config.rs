use multiversx_sc_scenario::imports::Bech32Address;
use multiversx_sc_snippets::imports::{ConnectionConfig, Wallet, WalletConfig};
use multiversx_sc_snippets::InteractorConfig;
use serde::Deserialize;

/// General settings for the multisig interact.
#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub contract_path: String,
    pub quorum: usize,
    pub wegld_address: Bech32Address,
}

/// Multisig Interact configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    pub connection: ConnectionConfig,
    pub general: GeneralConfig,
    pub wallet: WalletConfig,
    pub board: Vec<WalletConfig>,
}

impl InteractorConfig for Config {
    fn connection(&self) -> &ConnectionConfig {
        &self.connection
    }

    fn register_wallets(&self) -> Vec<Wallet> {
        self.board.iter().map(|w| w.wallet().clone()).collect()
    }
}
