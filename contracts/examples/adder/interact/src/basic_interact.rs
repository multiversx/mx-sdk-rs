mod basic_interact_cli;
mod basic_interact_config;
mod basic_interact_state;

use core::str;

use adder::adder_proxy;
use basic_interact_config::Config;
use basic_interact_state::State;
use clap::Parser;

use multiversx_sc_snippets::imports::*;

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";

const ADDER_CODE_PATH: MxscPath = MxscPath::new("../output/adder.mxsc.json");

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut basic_interact = AdderInteract::init().await;

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
            basic_interact.print_sum().await;
        },
        Some(basic_interact_cli::InteractCliCommand::Upgrade(args)) => {
            basic_interact.upgrade(args.value).await
        },
        None => {},
    }
}

#[allow(unused)]
struct AdderInteract {
    interactor: Interactor,
    adder_owner_address: Bech32Address,
    wallet_address: Bech32Address,
    state: State,
}

impl AdderInteract {
    async fn init() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(config.gateway())
            .await
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;

        let adder_owner_address = interactor.register_wallet(test_wallets::alice());

        let wallet_address = interactor.register_wallet(test_wallets::mike());

        interactor
            .proxy
            .send_user_funds(&Bech32Address::from(&adder_owner_address).to_bech32_string())
            .await;

        interactor
            .proxy
            .send_user_funds(&Bech32Address::from(&wallet_address).to_bech32_string())
            .await;

        interactor.proxy.generate_blocks(20).await;

        Self {
            interactor,
            adder_owner_address: adder_owner_address.into(),
            wallet_address: wallet_address.into(),
            state: State::load_state(),
        }
    }

    async fn deploy(&mut self) {
        // warning: multi deploy not yet fully supported
        // only works with last deployed address

        let new_address = self
            .interactor
            .tx()
            .from(&self.adder_owner_address)
            .gas(3_000_000)
            .typed(adder_proxy::AdderProxy)
            .init(0u32)
            .code(ADDER_CODE_PATH)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewBech32Address)
            .prepare_async()
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.set_adder_address(new_address.clone());
    }

    async fn multi_deploy(&mut self, count: usize) {
        if count == 0 {
            println!("count must be greater than 0");
            return;
        }

        println!("deploying {count} contracts...");

        let mut buffer = self.interactor.homogenous_call_buffer();
        for _ in 0..count {
            buffer.push_tx(|tx| {
                tx.from(&self.wallet_address)
                    .typed(adder_proxy::AdderProxy)
                    .init(0u32)
                    .code(ADDER_CODE_PATH)
                    .gas(3_000_000)
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

    async fn multi_add(&mut self, value: u32, count: usize) {
        println!("calling contract {count} times...");

        let mut buffer = self.interactor.homogenous_call_buffer();
        for _ in 0..count {
            buffer.push_tx(|tx| {
                tx.from(&self.wallet_address)
                    .to(self.state.current_adder_address())
                    .typed(adder_proxy::AdderProxy)
                    .add(value)
                    .gas(3_000_000)
            });
        }

        let _ = buffer.run().await;

        println!("successfully performed add {count} times");
    }

    async fn feed_contract_egld(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_adder_address())
            .egld(NumExpr("0,050000000000000000"))
            .prepare_async()
            .run()
            .await;
    }

    async fn add(&mut self, value: u32) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_adder_address())
            .gas(3_000_000)
            .typed(adder_proxy::AdderProxy)
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
            .to(self.state.current_adder_address())
            .typed(adder_proxy::AdderProxy)
            .sum()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("sum: {sum}");
    }

    async fn upgrade(&mut self, new_value: u32) {
        let response = self
            .interactor
            .tx()
            .from(&self.adder_owner_address)
            .to(self.state.current_adder_address())
            .gas(3_000_000)
            .typed(adder_proxy::AdderProxy)
            .upgrade(BigUint::from(new_value))
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .code(ADDER_CODE_PATH)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        let sum = self
            .interactor
            .query()
            .to(self.state.current_adder_address())
            .typed(adder_proxy::AdderProxy)
            .sum()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;
        assert_eq!(sum, RustBigUint::from(new_value));

        println!("response: {response:?}");
    }
}

#[tokio::test]
#[ignore = "run on demand"]
async fn test() {
    let mut basic_interact = AdderInteract::init().await;

    basic_interact.deploy().await;
    basic_interact.add(1u32).await;

    basic_interact.upgrade(5u32).await;
}
