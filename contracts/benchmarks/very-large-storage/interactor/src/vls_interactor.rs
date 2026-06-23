mod vls_interactor_cli;

use clap::Parser;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use very_large_storage::very_large_storage_proxy;

/// Very Large Storage Interact configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    pub connection: ConnectionConfig,
    pub owner: WalletConfig,
}

impl InteractorConfig for Config {
    fn connection(&self) -> &ConnectionConfig {
        &self.connection
    }

    fn register_wallets(&self) -> Vec<Wallet> {
        vec![self.owner.wallet().clone()]
    }
}

/// Very Large Storage Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub contract_address: Option<Bech32Address>,
}

impl State {
    /// Returns the contract address
    pub fn current_contract_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known contract, deploy first")
    }
}

const CODE_PATH: MxscPath = MxscPath::new("../output/very-large-storage.mxsc.json");

pub async fn very_large_storage_cli() {
    env_logger::init();

    let mut basic_interact = VeryLargeStorageInteractor::new().await;

    let cli = vls_interactor_cli::InteractCli::parse();
    match &cli.command {
        Some(vls_interactor_cli::InteractCliCommand::Deploy) => {
            basic_interact.deploy().await;
        }
        Some(vls_interactor_cli::InteractCliCommand::Append(args)) => {
            basic_interact.append(args.num_bytes).await;
        }
        None => {}
    }
}

pub struct VeryLargeStorageInteractor {
    pub interactor: Interactor,
    pub config: Config,
    pub state: AutoSave<State>,
}

impl VeryLargeStorageInteractor {
    pub async fn new() -> Self {
        let mut interactor = Interactor::empty().with_current_dir(env!("CARGO_MANIFEST_DIR"));
        let config: Config = interactor.load_config_toml().await;
        let state = interactor.load_state::<State>();
        Self {
            interactor,
            config,
            state,
        }
    }

    pub async fn deploy(&mut self) {
        let owner_address = self.config.owner.address();
        let new_address = self
            .interactor
            .tx()
            .from(&owner_address)
            .gas(10_000_000u64)
            .typed(very_large_storage_proxy::VeryLargeStorageProxy)
            .init()
            .code(CODE_PATH)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.contract_address = Some(new_address);
    }

    pub async fn append(&mut self, num_bytes: u64) {
        let owner_address = self.config.owner.address();
        let gas_used = self
            .interactor
            .tx()
            .from(&owner_address)
            .to(self.state.current_contract_address())
            .gas(SimulateGas)
            .typed(very_large_storage_proxy::VeryLargeStorageProxy)
            .append(num_bytes)
            .returns(ReturnsGasUsed)
            .run()
            .await;

        let x_len = self.get_x_len().await;
        println!(
            "Successfully appended {num_bytes} bytes (gas used: {gas_used}, total storage size: {x_len} bytes)"
        );
    }

    pub async fn get_x_len(&mut self) -> usize {
        self.interactor
            .query()
            .to(self.state.current_contract_address())
            .typed(very_large_storage_proxy::VeryLargeStorageProxy)
            .x_len()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }
}
