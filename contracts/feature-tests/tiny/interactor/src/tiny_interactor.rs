mod tiny_interactor_cli;

use clap::Parser;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};

/// Tiny Interact configuration
#[derive(Debug, Deserialize)]
pub struct Config {
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

/// Tiny Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub sc_address: Option<Bech32Address>,
}

impl State {
    /// Returns the contract address
    pub fn current_contract_address(&self) -> &Bech32Address {
        self.sc_address
            .as_ref()
            .expect("no known contract, deploy first")
    }
}

const CODE_PATH: FilePath = FilePath("../tiny.wasm");

pub async fn tiny_interactor_cli() {
    env_logger::init();

    let mut basic_interact = TinyInteractor::new().await;

    let cli = tiny_interactor_cli::InteractCli::parse();
    match &cli.command {
        Some(tiny_interactor_cli::InteractCliCommand::Deploy) => {
            basic_interact.deploy().await;
        }
        Some(tiny_interactor_cli::InteractCliCommand::Nothing) => {
            basic_interact.call_x().await;
        }
        None => {}
    }
}

pub struct TinyInteractor {
    pub interactor: Interactor,
    pub config: Config,
    pub state: AutoSave<State>,
}

impl TinyInteractor {
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

    pub async fn generate_blocks(&self, num_blocks: i32) {
        self.interactor
            .generate_blocks(num_blocks as u64)
            .await
            .unwrap();
    }

    pub async fn deploy(&mut self) {
        let owner_address = self.config.owner.address();
        let new_address = self
            .interactor
            .tx()
            .from(&owner_address.clone())
            .gas(600_000)
            .raw_deploy()
            .code(CODE_PATH)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.sc_address = Some(new_address);
    }

    pub async fn call_x(&mut self) {
        let wallet_address = self.config.wallet.address();
        self.interactor
            .tx()
            .from(&wallet_address)
            .to(self.state.current_contract_address())
            .gas(1_100_000)
            .raw_call("x")
            .run()
            .await;

        println!("Successfully called x");
    }
}
