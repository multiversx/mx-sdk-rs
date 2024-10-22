mod basic_interact_cli;
mod basic_interact_config;
mod basic_interact_state;

use crate::basic_interact_state::State;
use adder::adder_proxy;
pub use basic_interact_config::Config;
use clap::Parser;

use multiversx_sc_snippets::imports::*;

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";

const ADDER_CODE_PATH: MxscPath = MxscPath::new("../output/adder.mxsc.json");

pub async fn adder_cli() {
    env_logger::init();

    let config = Config::load_config();

    let mut basic_interact = AdderInteract::init(config).await;

    let cli = basic_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(basic_interact_cli::InteractCliCommand::Add(args)) => {
            if args.count == 1 {
                basic_interact.add(args.value).await;
            } else {
                basic_interact.multi_add(args.value, args.count).await;
            }
        },
        Some(basic_interact_cli::InteractCliCommand::Deploy) => {
            basic_interact.deploy().await;
        },
        Some(basic_interact_cli::InteractCliCommand::Feed) => {
            basic_interact.feed_contract_egld().await;
        },
        Some(basic_interact_cli::InteractCliCommand::MultiDeploy(args)) => {
            basic_interact.multi_deploy(args.count).await;
        },
        Some(basic_interact_cli::InteractCliCommand::Sum) => {
            let sum = basic_interact.get_sum().await;
            println!("sum: {sum}");
        },
        Some(basic_interact_cli::InteractCliCommand::Upgrade(args)) => {
            let owner_address = basic_interact.adder_owner_address.clone();
            basic_interact
                .upgrade(args.value, &owner_address, None)
                .await
        },
        None => {},
    }
}

#[allow(unused)]
pub struct AdderInteract {
    pub interactor: Interactor,
    pub adder_owner_address: Bech32Address,
    pub wallet_address: Bech32Address,
    pub state: State,
}

impl AdderInteract {
    pub async fn init(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri(), config.use_chain_simulator())
            .await
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;

        let adder_owner_address = interactor
            .register_wallet(Wallet::from_pem_file("adder-owner.pem").unwrap())
            .await;
        // PASSWORD: "alice"
        // InsertPassword::Plaintext("alice".to_string()) || InsertPassword::StandardInput
        let wallet_address = interactor
            .register_wallet(
                Wallet::from_keystore_secret(
                    "alice.json",
                    InsertPassword::Plaintext("alice".to_string()),
                )
                .unwrap(),
            )
            .await;

        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();

        Self {
            interactor,
            adder_owner_address: adder_owner_address.into(),
            wallet_address: wallet_address.into(),
            state: State::load_state(),
        }
    }

    pub async fn set_state(&mut self) {
        println!("wallet address: {}", self.wallet_address);
        self.interactor
            .retrieve_account(&self.adder_owner_address)
            .await;
        self.interactor.retrieve_account(&self.wallet_address).await;
    }

    pub async fn deploy(&mut self) {
        // warning: multi deploy not yet fully supported
        // only works with last deployed address

        self.set_state().await;

        let new_address = self
            .interactor
            .tx()
            .from(&self.adder_owner_address)
            .gas(6_000_000)
            .typed(adder_proxy::AdderProxy)
            .init(0u32)
            .code(ADDER_CODE_PATH)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.set_adder_address(new_address);
    }

    pub async fn multi_deploy(&mut self, count: usize) {
        if count == 0 {
            println!("count must be greater than 0");
            return;
        }

        self.set_state().await;
        println!("deploying {count} contracts...");

        let mut buffer = self.interactor.homogenous_call_buffer();
        for _ in 0..count {
            buffer.push_tx(|tx| {
                tx.from(&self.wallet_address)
                    .typed(adder_proxy::AdderProxy)
                    .init(0u32)
                    .code(ADDER_CODE_PATH)
                    .gas(6_000_000)
                    .returns(ReturnsNewBech32Address)
            });
        }

        let results = buffer.run().await;

        // warning: multi deploy not yet fully supported
        // only works with last deployed address

        for new_address in results {
            println!("new address: {new_address}");

            self.state.set_adder_address(new_address);
        }
    }

    pub async fn multi_add(&mut self, value: u32, count: usize) {
        self.set_state().await;
        println!("calling contract {count} times...");

        let mut buffer = self.interactor.homogenous_call_buffer();
        for _ in 0..count {
            buffer.push_tx(|tx| {
                tx.from(&self.wallet_address)
                    .to(self.state.current_adder_address())
                    .typed(adder_proxy::AdderProxy)
                    .add(value)
                    .returns(ReturnsGasUsed)
                    .gas(6_000_000)
            });
        }

        let gas_used = buffer.run().await;
        let gas_used_sum = gas_used.iter().sum::<u64>();

        println!(
            "successfully performed add {count} times, total gas used: {}, avg gas used: {}",
            gas_used_sum,
            gas_used_sum / count as u64
        );
    }

    pub async fn feed_contract_egld(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_adder_address())
            .egld(NumExpr("0,050000000000000000"))
            .run()
            .await;
    }

    pub async fn add(&mut self, value: u32) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_adder_address())
            .gas(6_000_000)
            .typed(adder_proxy::AdderProxy)
            .add(value)
            .run()
            .await;

        println!("successfully performed add");
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

    pub async fn upgrade(
        &mut self,
        new_value: u32,
        sender: &Bech32Address,
        expected_result: Option<(u64, &str)>,
    ) {
        match expected_result {
            Some((code, msg)) => {
                let response = self
                    .interactor
                    .tx()
                    .from(sender)
                    .to(self.state.current_adder_address())
                    .gas(6_000_000)
                    .typed(adder_proxy::AdderProxy)
                    .upgrade(new_value)
                    .code_metadata(CodeMetadata::UPGRADEABLE)
                    .code(ADDER_CODE_PATH)
                    .returns(ExpectError(code, msg))
                    .run()
                    .await;

                println!("response: {response:?}");
            },
            None => {
                self.interactor
                    .tx()
                    .from(sender)
                    .to(self.state.current_adder_address())
                    .gas(6_000_000)
                    .typed(adder_proxy::AdderProxy)
                    .upgrade(new_value)
                    .code_metadata(CodeMetadata::UPGRADEABLE)
                    .code(ADDER_CODE_PATH)
                    .run()
                    .await;

                let sum = self
                    .interactor
                    .query()
                    .to(self.state.current_adder_address())
                    .typed(adder_proxy::AdderProxy)
                    .sum()
                    .returns(ReturnsResultUnmanaged)
                    .run()
                    .await;

                assert_eq!(sum, RustBigUint::from(new_value));
            },
        }
    }
}
