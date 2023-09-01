mod adder_interact_cli;
mod adder_interact_config;
mod adder_interact_state;

use adder::ProxyTrait;
use adder_interact_config::Config;
use adder_interact_state::State;
use clap::Parser;
use multiversx_sc_snippets::{
    env_logger,
    multiversx_sc::{storage::mappers::SingleValue, types::Address},
    multiversx_sc_scenario::{
        api::StaticApi,
        bech32,
        mandos_system::ScenarioRunner,
        num_bigint::BigUint,
        scenario_format::interpret_trait::{InterpretableFrom, InterpreterContext},
        scenario_model::{
            BytesValue, ScCallStep, ScDeployStep, ScQueryStep, Scenario, TransferStep, TxExpect,
        },
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
        let adder_code =
            BytesValue::interpret_from("file:../output/adder.wasm", &InterpreterContext::default());

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

        self.interactor
            .sc_deploy_use_result(
                ScDeployStep::new()
                    .call(self.state.default_adder().init(BigUint::from(0u64)))
                    .from(&self.wallet_address)
                    .code(&self.adder_code),
                |new_address, tr| {
                    tr.result.unwrap_or_else(|err| {
                        panic!(
                            "deploy failed: status: {}, message: {}",
                            err.status, err.message
                        )
                    });

                    let new_address_bech32 = bech32::encode(&new_address);
                    println!("new address: {new_address_bech32}");

                    let new_address_expr = format!("bech32:{new_address_bech32}");
                    self.state.set_adder_address(&new_address_expr);
                },
            )
            .await;
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
        self.interactor
            .sc_call(
                ScCallStep::new()
                    .call(self.state.adder().add(value))
                    .from(&self.wallet_address)
                    .expect(
                        TxExpect::ok().additional_error_message("performing add failed with: "),
                    ),
            )
            .await;

        println!("successfully performed add");
    }

    async fn print_sum(&mut self) {
        self.interactor
            .sc_query_use_result(ScQueryStep::new().call(self.state.adder().sum()), |tr| {
                let sum: SingleValue<BigUint> = tr.result.unwrap();
                println!("sum: {}", sum.into());
            })
            .await;
    }
}
