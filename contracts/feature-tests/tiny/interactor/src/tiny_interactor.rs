mod tiny_interactor_cli;
mod tiny_interactor_config;
mod tiny_interactor_state;

use clap::Parser;
pub use tiny_interactor_config::Config;
use tiny_interactor_state::State;

use multiversx_sc_snippets::imports::*;

const CODE_PATH: FilePath = FilePath("../tiny.wasm");

pub async fn tiny_interactor_cli() {
    env_logger::init();

    let config = Config::load_config();

    let mut basic_interact = BasicInteractor::new(config).await;

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

pub struct BasicInteractor {
    pub interactor: Interactor,
    pub adder_owner_address: Bech32Address,
    pub wallet_address: Bech32Address,
    pub state: State,
}

impl BasicInteractor {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        let adder_owner_address = interactor.register_wallet(test_wallets::mike()).await;
        let wallet_address = interactor.register_wallet(test_wallets::ivan()).await;

        interactor.generate_blocks(30u64).await.unwrap();

        BasicInteractor {
            interactor,
            adder_owner_address: adder_owner_address.into(),
            wallet_address: wallet_address.into(),
            state: State::load_state(),
        }
    }

    pub async fn generate_blocks(&self, num_blocks: i32) {
        self.interactor
            .generate_blocks(num_blocks as u64)
            .await
            .unwrap();
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.adder_owner_address.clone())
            .gas(600_000)
            .raw_deploy()
            .code(CODE_PATH)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.set_adder_address(new_address);
    }

    pub async fn call_x(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_contract_address())
            .gas(1_100_000)
            .raw_call("x")
            .run()
            .await;

        println!("Successfully called x");
    }
}
