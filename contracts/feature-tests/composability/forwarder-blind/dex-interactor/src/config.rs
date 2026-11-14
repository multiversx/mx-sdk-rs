use multiversx_sc_snippets::imports::*;
use serde::Deserialize;

/// General settings for the forwarder-blind dex interact.
#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub contract_path: ConfigPath,
    pub wegld_address: Bech32Address,
    pub pair_address: Bech32Address,
    pub wegld_token_id: String,
    pub usdc_token_id: String,
}

/// Contract Interact configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    pub connection: ConnectionConfig,
    pub general: GeneralConfig,
    pub wallets: Vec<WalletConfig>,
    /// Forwarder contract addresses to target for all swap transactions.
    #[serde(default)]
    pub contract_addresses: Vec<Bech32Address>,
}

impl InteractorConfig for Config {
    fn connection(&self) -> &ConnectionConfig {
        &self.connection
    }

    fn register_wallets(&self) -> Vec<Wallet> {
        self.wallets.iter().map(|w| w.wallet().clone()).collect()
    }
}
