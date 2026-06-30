mod adder_interactor_cli;

use adder::adder_proxy;
use clap::Parser;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};

/// Adder Interact general settings
#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub contract_path: ConfigPath,
}

/// Adder Interact configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
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

/// Adder Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub adder_address: Option<Bech32Address>,
}

impl State {
    /// Returns the adder contract
    pub fn current_adder_address(&self) -> &Bech32Address {
        self.adder_address
            .as_ref()
            .expect("no known adder contract, deploy first")
    }
}

pub async fn adder_cli() {
    env_logger::init();

    let mut adder_interact = AdderInteractor::from_config().await;

    let cli = adder_interactor_cli::InteractCli::parse();
    match &cli.command {
        Some(adder_interactor_cli::InteractCliCommand::Deploy) => {
            adder_interact.deploy().await;
        }
        Some(adder_interactor_cli::InteractCliCommand::Upgrade(args)) => {
            let owner_address = adder_interact.config.owner.address();
            adder_interact
                .upgrade(args.value, &owner_address, None)
                .await
        }
        Some(adder_interactor_cli::InteractCliCommand::Add(args)) => {
            adder_interact.add(args.value).await;
        }
        Some(adder_interactor_cli::InteractCliCommand::Sum) => {
            let sum = adder_interact.get_sum().await;
            println!("sum: {sum}");
        }
        None => {}
    }
}

pub struct AdderInteractor {
    pub interactor: Interactor,
    pub config: Config,
    pub state: AutoSave<State>,
}

impl AdderInteractor {
    pub async fn from_config() -> Self {
        let mut interactor = Interactor::empty().with_current_dir(env!("CARGO_MANIFEST_DIR"));
        let config: Config = interactor.load_config_toml().await;
        let state = interactor.load_state::<State>();
        AdderInteractor {
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
            .id("interactor deploy")
            .from(&owner_address)
            .gas(100_000_000)
            .typed(adder_proxy::AdderProxy)
            .init(0u64)
            .code(&self.config.general.contract_path)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.adder_address = Some(new_address);
    }

    pub async fn upgrade(&mut self, new_value: u32, sender: &Bech32Address, err: Option<&str>) {
        let response = self
            .interactor
            .tx()
            .from(sender)
            .to(self.state.current_adder_address())
            .gas(6_000_000)
            .typed(adder_proxy::AdderProxy)
            .upgrade(new_value)
            .code(&self.config.general.contract_path)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        match response {
            Ok(_) => {
                println!("Contract successfully upgraded.");
            }
            Err(tx_err) => {
                println!("Contract failed upgrade with error: {}", tx_err.message);
                assert_eq!(tx_err.message, err.unwrap_or_default());
            }
        }
    }

    pub async fn add(&mut self, value: u32) {
        self.interactor
            .tx()
            .id("interactor add")
            .from(self.config.wallet.address())
            .to(self.state.current_adder_address())
            .gas(6_000_000u64)
            .typed(adder_proxy::AdderProxy)
            .add(value)
            .run()
            .await;

        println!("Successfully performed add");
    }

    pub async fn get_sum(&mut self) -> RustBigUint {
        self.interactor
            .query()
            .to(self.state.current_adder_address())
            .typed(adder_proxy::AdderProxy)
            .sum()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }
}
