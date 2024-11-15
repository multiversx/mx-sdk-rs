mod basic_interactor_cli;
mod basic_interactor_config;
mod basic_interactor_state;

use adder::adder_proxy;
pub use basic_interactor_config::Config;
use basic_interactor_state::State;
use clap::Parser;

use multiversx_sc_snippets::imports::*;

const ADDER_CODE_PATH: MxscPath = MxscPath::new("../output/adder.mxsc.json");

pub async fn adder_cli() {
    env_logger::init();

    let config = Config::load_config();

    let mut basic_interact = AdderInteract::new(config).await;

    let cli = basic_interactor_cli::InteractCli::parse();
    match &cli.command {
        Some(basic_interactor_cli::InteractCliCommand::Deploy) => {
            basic_interact.deploy().await;
        },
        Some(basic_interactor_cli::InteractCliCommand::Upgrade(args)) => {
            let owner_address = basic_interact.adder_owner_address.clone();
            basic_interact
                .upgrade(args.value, &owner_address, None)
                .await
        },
        Some(basic_interactor_cli::InteractCliCommand::Add(args)) => {
            basic_interact.add(args.value).await;
        },
        Some(basic_interactor_cli::InteractCliCommand::Sum) => {
            let sum = basic_interact.get_sum().await;
            println!("sum: {sum}");
        },
        None => {},
    }
}

pub struct AdderInteract {
    pub interactor: Interactor,
    pub adder_owner_address: Bech32Address,
    pub wallet_address: Bech32Address,
    pub state: State,
}

impl AdderInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());
        interactor.set_current_dir_from_workspace("contracts/examples/adder/interactor");

        let adder_owner_address = interactor.register_wallet(test_wallets::heidi()).await;
        let wallet_address = interactor.register_wallet(test_wallets::ivan()).await;

        interactor.generate_blocks_until_epoch(1).await.unwrap();

        AdderInteract {
            interactor,
            adder_owner_address: adder_owner_address.into(),
            wallet_address: wallet_address.into(),
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.adder_owner_address.clone())
            .gas(6_000_000)
            .typed(adder_proxy::AdderProxy)
            .init(0u64)
            .code(ADDER_CODE_PATH)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.set_adder_address(new_address);
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
            .code(ADDER_CODE_PATH)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        match response {
            Ok(_) => {
                println!("Contract successfully upgraded.");
            },
            Err(tx_err) => {
                println!("Contract failed upgrade with error: {}", tx_err.message);
                assert_eq!(tx_err.message, err.unwrap_or_default());
            },
        }
    }

    pub async fn add(&mut self, value: u32) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
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
