mod bf_interact_cli;
mod bf_interact_config;
mod bf_interact_state;

use basic_features::{
    storage_direct_load::ProxyTrait as _, storage_direct_store::ProxyTrait as _, ProxyTrait,
};
use bf_interact_config::Config;
use bf_interact_state::State;
use clap::Parser;
use multiversx_sc_snippets::{
    env_logger,
    multiversx_sc::types::{Address, CodeMetadata},
    multiversx_sc_scenario::{
        api::StaticApi,
        bech32,
        mandos_system::ScenarioRunner,
        scenario_format::interpret_trait::{InterpretableFrom, InterpreterContext},
        scenario_model::{ScCallStep, ScDeployStep, Scenario, TxExpect},
        standalone::retrieve_account_as_scenario_set_state,
        test_wallets, ContractInfo,
    },
    tokio, Interactor,
};

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
    wallet_address: Address,
    state: State,
    large_storage_payload: Vec<u8>,
}

impl BasicFeaturesInteract {
    async fn init() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(config.gateway())
            .await
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;
        let wallet_address = interactor.register_wallet(test_wallets::mike());

        Self {
            interactor,
            wallet_address,
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
        println!("wallet address: {}", bech32::encode(&self.wallet_address));
        let scenario_raw = retrieve_account_as_scenario_set_state(
            Config::load_config().gateway().to_string(),
            bech32::encode(&self.wallet_address),
            true,
        )
        .await;

        let scenario = Scenario::interpret_from(scenario_raw, &InterpreterContext::default());

        self.interactor.pre_runners.run_scenario(&scenario);
        self.interactor.post_runners.run_scenario(&scenario);
    }

    async fn deploy(&mut self) {
        self.set_state().await;

        self.interactor
            .sc_deploy_use_result(
                ScDeployStep::new()
                    .call(self.state.default_contract().init())
                    .from(&self.wallet_address)
                    .code_metadata(CodeMetadata::all())
                    .contract_code(
                        "file:../output/basic-features-storage-bytes.wasm",
                        &InterpreterContext::default(),
                    )
                    .gas_limit("4,000,000")
                    .expect(TxExpect::ok()),
                |new_address, tr| {
                    tr.result
                        .unwrap_or_else(|err| panic!("deploy failed: {}", err.message));

                    let new_address_bech32 = bech32::encode(&new_address);
                    println!("new address: {new_address_bech32}");

                    let new_address_expr = format!("bech32:{new_address_bech32}");
                    self.state.set_bf_address(&new_address_expr);
                },
            )
            .await;
    }

    async fn set_large_storage(&mut self, value: &[u8]) {
        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.state.bf_contract().store_bytes(value))
                    .from(&self.wallet_address)
                    .gas_limit("600,000,000"),
                |tr| {
                    tr.result.unwrap_or_else(|err| {
                        panic!("performing store_bytes failed with: {}", err.message)
                    });
                },
            )
            .await;

        println!("successfully performed add");
    }

    async fn print_length(&mut self) {
        let data: Vec<u8> = self
            .interactor
            .quick_query(self.state.bf_contract().load_bytes())
            .await;
        println!("retrieved data length: {}", data.len());
        if data != self.large_storage_payload {
            println!("WARNING! Payload mismatch!");
        }
    }
}
