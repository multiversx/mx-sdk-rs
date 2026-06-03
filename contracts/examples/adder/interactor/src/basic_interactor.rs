mod basic_interactor_cli;
mod basic_interactor_config;
mod basic_interactor_state;

use adder::adder_proxy;
pub use basic_interactor_config::Config;
use basic_interactor_state::State;
use clap::Parser;

use multiversx_sc_snippets::imports::*;

pub async fn adder_cli() {
    env_logger::init();

    let mut basic_interact = BasicInteractor::new().await;

    let cli = basic_interactor_cli::InteractCli::parse();
    match &cli.command {
        Some(basic_interactor_cli::InteractCliCommand::Deploy) => {
            basic_interact.deploy().await;
        }
        Some(basic_interactor_cli::InteractCliCommand::Upgrade(args)) => {
            let owner_address = basic_interact.config.owner.address();
            basic_interact
                .upgrade(args.value, &owner_address, None)
                .await
        }
        Some(basic_interactor_cli::InteractCliCommand::Add(args)) => {
            basic_interact.add(args.value).await;
        }
        Some(basic_interactor_cli::InteractCliCommand::Sum) => {
            let sum = basic_interact.get_sum().await;
            println!("sum: {sum}");
        }
        None => {}
    }
}

pub struct BasicInteractor {
    pub interactor: Interactor,
    pub config: Config,
    pub state: AutoSave<State>,
}

impl BasicInteractor {
    pub async fn new() -> Self {
        let (interactor, config) = HttpInteractorBuilder::new()
            .crate_dir(env!("CARGO_MANIFEST_DIR"))
            .build()
            .await;
        let state = interactor.load_state::<State>();
        BasicInteractor {
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
            .code(MxscPath::new(&self.config.contract_path))
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
            .code(MxscPath::new(&self.config.contract_path))
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
        let wallet_address = self.config.wallet.address();
        self.interactor
            .tx()
            .id("interactor add")
            .from(&wallet_address)
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
