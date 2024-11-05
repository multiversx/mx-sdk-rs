mod bf_interact_cli;
mod bf_interact_config;
mod bf_interact_state;

use basic_features::basic_features_proxy;
use bf_interact_config::Config;
use bf_interact_state::State;
use clap::Parser;

use multiversx_sc_snippets::imports::*;

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut bf_interact = BasicFeaturesInteract::init().await;

    let cli = bf_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(bf_interact_cli::InteractCliCommand::Deploy) => {
            bf_interact.deploy().await;
        },
        Some(bf_interact_cli::InteractCliCommand::LargeStorage(args)) => {
            bf_interact.large_storage(args.size_kb).await;
        },
        None => {},
    }
}

#[allow(unused)]
struct BasicFeaturesInteract {
    interactor: Interactor,
    wallet_address: Bech32Address,
    code_expr: BytesValue,
    state: State,
    large_storage_payload: Vec<u8>,
}

impl BasicFeaturesInteract {
    async fn init() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(config.gateway_uri(), config.use_chain_simulator())
            .await
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;
        interactor
            .set_current_dir_from_workspace("contracts/feature-tests/basic-features/interact");
        let wallet_address = interactor.register_wallet(test_wallets::mike()).await;
        let code_expr = BytesValue::interpret_from(
            "mxsc:../output/basic-features-storage-bytes.mxsc.json",
            &InterpreterContext::default(),
        );

        Self {
            interactor,
            wallet_address: wallet_address.into(),
            code_expr,
            state: State::load_state(),
            large_storage_payload: Vec::new(),
        }
    }

    async fn large_storage(&mut self, size_kb: usize) {
        let large_data = std::fs::read_to_string("pi.txt").unwrap().into_bytes();
        let payload = &large_data[0..size_kb * 1024];
        println!("payload size: {}", payload.len());
        self.set_large_storage(payload).await;
        self.large_storage_payload = payload.to_vec();
        self.print_length().await;
    }

    async fn set_state(&mut self) {
        println!("wallet address: {}", self.wallet_address);
        self.interactor.retrieve_account(&self.wallet_address).await;
    }

    async fn deploy(&mut self) {
        self.set_state().await;

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .init()
            .code(&self.code_expr)
            .gas(NumExpr("4,000,000"))
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address: {new_address}");

        self.state.set_bf_address(new_address);
    }

    async fn set_large_storage(&mut self, value: &[u8]) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.bf_contract())
            .gas(NumExpr("600,000,000"))
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_bytes(value)
            .run()
            .await;

        println!("successfully performed store_bytes");
    }

    async fn print_length(&mut self) {
        let data_raw = self
            .interactor
            .query()
            .to(self.state.bf_contract())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .load_bytes()
            .returns(ReturnsResult)
            .run()
            .await;

        let data = data_raw.to_vec();

        println!("retrieved data length: {}", data.len());
        if data != self.large_storage_payload {
            println!("WARNING! Payload mismatch!");
        }
    }
}
