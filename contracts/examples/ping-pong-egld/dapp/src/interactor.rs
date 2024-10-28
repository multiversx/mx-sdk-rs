use multiversx_sc_snippets_dapp::imports::*;
use serde::{Deserialize, Serialize};

const GATEWAY: &str = multiversx_sc_snippets_dapp::sdk::core::gateway::DEVNET_GATEWAY;
const CONTRACT_ADDRESS: &str = "erd1qqqqqqqqqqqqqpgq6tqvj5f59xrgxwrtwy30elgpu7l4zrv6d8ssnjdwxq";
const PING_PONG_CODE: &[u8] = include_bytes!("../ping-pong-egld.wasm");

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    gateway: String,
    contract_address: String,
}

impl Config {
    // Deserializes state from file
    pub fn new() -> Self {
        Config {
            gateway: GATEWAY.to_string(),
            contract_address: CONTRACT_ADDRESS.to_string(),
        }
    }

    /// Sets the contract address
    #[allow(unused)]
    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = address.to_string()
    }

    /// Returns the contract address
    pub fn current_address(&self) -> &String {
        &self.contract_address
    }
}

pub struct ContractInteract {
    pub interactor: DappInteractor,
    pub wallet_address: Address,
    pub contract_code: BytesValue,
    pub config: Config,
}

impl ContractInteract {
    pub async fn new() -> Self {
        let config = Config::new();
        let mut interactor = DappInteractor::new(&config.gateway, false).await;
        interactor.set_current_dir_from_workspace("contracts/examples/ping-pong-egld/dapp");
        let wallet_address = interactor.register_wallet(test_wallets::mike()).await;

        let contract_code = BytesValue::from(PING_PONG_CODE);

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            config,
        }
    }
}
