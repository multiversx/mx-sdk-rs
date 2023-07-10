mod adder_interact_cli;
mod adder_interact_config;
mod adder_interact_state;

use adder::ProxyTrait;
use adder_interact_config::Config;
use adder_interact_state::State;
use clap::Parser;
use multiversx_sc_snippets::{
    env_logger,
    multiversx_sc::{
        storage::mappers::SingleValue,
        types::{Address, CodeMetadata},
    },
    multiversx_sc_scenario::{
        api::StaticApi,
        bech32,
        mandos_system::ScenarioRunner,
        num_bigint::BigUint,
        scenario_format::interpret_trait::{InterpretableFrom, InterpreterContext},
        scenario_model::{IntoBlockchainCall, Scenario, TransferStep, TxExpect},
        standalone::retrieve_account_as_scenario_set_state,
        test_wallets, ContractInfo,
    },
    tokio, Interactor, StepBuffer,
};

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut adder_interact = AdderInteract::init().await;

    let cli = adder_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(adder_interact_cli::InteractCliCommand::Add(args)) => {
            adder_interact.add(args.value).await;
        },
        Some(adder_interact_cli::InteractCliCommand::Deploy) => {
            adder_interact.deploy().await;
        },
        Some(adder_interact_cli::InteractCliCommand::Feed) => {
            adder_interact.feed_contract_egld().await;
        },
        Some(adder_interact_cli::InteractCliCommand::MultiDeploy(args)) => {
            adder_interact.multi_deploy(&args.count).await;
        },
        Some(adder_interact_cli::InteractCliCommand::Sum) => {
            adder_interact.print_sum().await;
        },
        None => {},
    }
}

#[allow(unused)]
struct AdderInteract {
    interactor: Interactor,
    wallet_address: Address,
    state: State,
}

impl AdderInteract {
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
        }
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

        let mut typed_sc_deploy = self
            .state
            .default_adder()
            .init(BigUint::from(0u64))
            .into_blockchain_call()
            .from(&self.wallet_address)
            .code_metadata(CodeMetadata::all())
            .contract_code("file:../output/adder.wasm", &InterpreterContext::default())
            .gas_limit("70,000,000")
            .expect(TxExpect::ok());

        self.interactor.sc_deploy(&mut typed_sc_deploy).await;

        let result = typed_sc_deploy.response().new_deployed_address();
        if result.is_err() {
            println!("deploy failed: {}", result.err().unwrap());
            return;
        }

        let new_address_bech32 = bech32::encode(&result.unwrap());
        println!("new address: {new_address_bech32}");

        let new_address_expr = format!("bech32:{new_address_bech32}");
        self.state.set_adder_address(&new_address_expr);
    }

    async fn multi_deploy(&mut self, count: &u8) {
        if *count == 0 {
            println!("count must be greater than 0");
            return;
        }

        self.set_state().await;
        println!("deploying {count} contracts...");

        let mut steps = Vec::new();
        for _ in 0..*count {
            let typed_sc_deploy = self
                .state
                .default_adder()
                .init(BigUint::from(0u64))
                .into_blockchain_call()
                .from(&self.wallet_address)
                .code_metadata(CodeMetadata::all())
                .contract_code("file:../output/adder.wasm", &InterpreterContext::default())
                .gas_limit("70,000,000")
                .expect(TxExpect::ok());

            steps.push(typed_sc_deploy);
        }

        self.interactor
            .multi_sc_exec(StepBuffer::from_sc_deploy_vec(&mut steps))
            .await;

        for step in steps.iter() {
            let result = step.response().new_deployed_address();
            if result.is_err() {
                println!("deploy failed: {}", result.err().unwrap());
                return;
            }

            let new_address_bech32 = bech32::encode(&result.unwrap());
            println!("new address: {new_address_bech32}");
        }
    }

    async fn feed_contract_egld(&mut self) {
        let _ = self
            .interactor
            .transfer(
                TransferStep::new()
                    .from(&self.wallet_address)
                    .to(self.state.adder())
                    .egld_value("0,050000000000000000"),
            )
            .await;
    }

    async fn add(&mut self, value: u64) {
        let mut typed_sc_call = self
            .state
            .adder()
            .add(BigUint::from(value))
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit("70,000,000")
            .expect(TxExpect::ok());

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.response().handle_signal_error_event();
        if result.is_err() {
            println!("performing add failed with: {}", result.err().unwrap());
            return;
        }

        println!("successfully performed add");
    }

    async fn print_sum(&mut self) {
        let sum: SingleValue<BigUint> = self.interactor.vm_query(self.state.adder().sum()).await;
        println!("sum: {}", sum.into());
    }
}
