mod basic_interact_cli;
mod basic_interact_config;
mod basic_interact_state;

use adder::{temp_proxy, ProxyTrait};
use basic_interact_config::Config;
use basic_interact_state::State;
use clap::Parser;
use multiversx_sc_snippets::{
    env_logger,
    multiversx_sc::types::{Address, ReturnsNewAddress, ReturnsSimilar},
    multiversx_sc_scenario::{
        api::StaticApi,
        bech32,
        mandos_system::ScenarioRunner,
        num_bigint::BigUint,
        scenario_format::interpret_trait::{InterpretableFrom, InterpreterContext},
        scenario_model::{BytesValue, ScDeployStep, Scenario},
        standalone::retrieve_account_as_scenario_set_state,
        test_wallets, ContractInfo, WithRawTxResponse,
    },
    tokio, Interactor, InteractorPrepareAsync, StepBuffer,
};

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut basic_interact = AdderInteract::init().await;

    let cli = basic_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(basic_interact_cli::InteractCliCommand::Add(args)) => {
            basic_interact.add(args.value).await;
        },
        Some(basic_interact_cli::InteractCliCommand::Deploy) => {
            basic_interact.deploy().await;
        },
        Some(basic_interact_cli::InteractCliCommand::Feed) => {
            basic_interact.feed_contract_egld().await;
        },
        Some(basic_interact_cli::InteractCliCommand::MultiDeploy(args)) => {
            basic_interact.multi_deploy(&args.count).await;
        },
        Some(basic_interact_cli::InteractCliCommand::Sum) => {
            basic_interact.print_sum().await;
        },
        None => {},
    }
}

#[allow(unused)]
struct AdderInteract {
    interactor: Interactor,
    wallet_address: Address,
    adder_code: BytesValue,
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
        let adder_code = BytesValue::interpret_from(
            "mxsc:../output/adder.mxsc.json",
            &InterpreterContext::default(),
        );

        Self {
            interactor,
            wallet_address,
            adder_code,
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

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .typed(temp_proxy::AdderProxy)
            .init(0u32)
            .code(&self.adder_code)
            .with_result(WithRawTxResponse(|response| {
                let err = &response.tx_error;
                assert!(
                    err.is_success(),
                    "deploy failed: status: {}, message: {}",
                    err.status,
                    err.message
                );
            }))
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;

        let new_address_bech32 = bech32::encode(&new_address.to_address());
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
            let typed_sc_deploy = ScDeployStep::new()
                .call(self.state.default_adder().init(0u32))
                .from(&self.wallet_address)
                .code(&self.adder_code)
                .gas_limit("70,000,000");

            steps.push(typed_sc_deploy);
        }

        self.interactor
            .multi_sc_exec(StepBuffer::from_sc_deploy_vec(&mut steps))
            .await;

        for step in steps.iter() {
            // warning: multi deploy not yet fully supported
            // only works with last deployed address
            // will be addressed in future versions
            let new_deployed_address = step.response().new_deployed_address.clone();
            if let Some(new_address) = new_deployed_address {
                let new_address_bech32 = bech32::encode(&new_address);
                println!("new address: {new_address_bech32}");
            } else {
                println!("deploy failed");
                return;
            }
        }
    }

    async fn feed_contract_egld(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.adder().to_address())
            .egld(50000000000000000u64.into()) // TODO: annotate "0,050000000000000000"
            .prepare_async()
            .run()
            .await;
    }

    async fn add(&mut self, value: u64) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.adder().to_address())
            .typed(temp_proxy::AdderProxy)
            .add(value)
            .prepare_async()
            .run()
            .await;

        println!("successfully performed add");
    }

    async fn print_sum(&mut self) {
        let sum = self
            .interactor
            .query()
            .to(self.state.adder().to_address())
            .typed(temp_proxy::AdderProxy)
            .sum()
            .returns(ReturnsSimilar::<BigUint>::new())
            .prepare_async()
            .run()
            .await;

        println!("sum: {sum}");
    }
}
