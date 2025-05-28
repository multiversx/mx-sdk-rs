mod barnard_features_proxy;
mod barnard_interactor_cli;
mod barnard_interactor_config;
mod barnard_interactor_state;

pub use barnard_interactor_config::Config;
use barnard_interactor_state::State;
use clap::Parser;

use multiversx_sc_snippets::imports::*;

const CODE_PATH: MxscPath = MxscPath::new("../output/barnard-features.mxsc.json");

pub async fn adder_cli() {
    env_logger::init();

    let config = Config::load_config();

    let mut basic_interact = PayableInteract::new(config).await;

    let cli = barnard_interactor_cli::InteractCli::parse();
    match cli.command {
        Some(barnard_interactor_cli::InteractCliCommand::Deploy) => {
            basic_interact.deploy().await;
        },
        Some(barnard_interactor_cli::InteractCliCommand::EpochInfo) => {
            basic_interact.epoch_info().await;
        },
        Some(barnard_interactor_cli::InteractCliCommand::CodeHash(args)) => {
            basic_interact
                .code_hash(Bech32Address::from_bech32_string(args.address))
                .await;
        },
        None => {},
    }
}

pub struct PayableInteract {
    pub interactor: Interactor,
    pub sc_owner_address: Bech32Address,
    pub wallet_address: Bech32Address,
    pub state: State,
}

impl PayableInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        let sc_owner_address = interactor.register_wallet(test_wallets::alice()).await;
        let wallet_address = interactor.register_wallet(test_wallets::ivan()).await;

        interactor.generate_blocks(30u64).await.unwrap();

        PayableInteract {
            interactor,
            sc_owner_address: sc_owner_address.into(),
            wallet_address: wallet_address.into(),
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.sc_owner_address.clone())
            .gas(30_000_000)
            .typed(barnard_features_proxy::BarnardFeaturesProxy)
            .init()
            .code(CODE_PATH)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.set_barnard_features_address(new_address);
    }

    pub async fn epoch_info(&mut self) {
        let result = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_barnard_features_address())
            .gas(3_000_000u64)
            .typed(barnard_features_proxy::BarnardFeaturesProxy)
            .epoch_info()
            .returns(ReturnsResult)
            .run()
            .await;

        println!("Result: {result:?}");
    }

    pub async fn code_hash(&mut self, address: Bech32Address) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_barnard_features_address())
            .typed(barnard_features_proxy::BarnardFeaturesProxy)
            .code_hash(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }
}
