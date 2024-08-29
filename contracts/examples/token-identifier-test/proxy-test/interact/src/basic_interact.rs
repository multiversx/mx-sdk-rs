mod basic_interact_cli;
mod basic_interact_config;
mod basic_interact_state;

use proxy_test::proxy_test_proxy;
use basic_interact_config::Config;
use basic_interact_state::State;
use clap::Parser;

use multiversx_sc_snippets::imports::*;

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";

const ADDER_CODE_PATH: MxscPath = MxscPath::new("../output/proxy-test.mxsc.json");

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut basic_interact = ProxyTestInteract::init().await;

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
        Some(basic_interact_cli::InteractCliCommand::Issue) => {
            basic_interact.issue_fungible_token().await
        },
        Some(basic_interact_cli::InteractCliCommand::ViewLastIssued) => {
            basic_interact.last_issued_token().await;
        },
        None => {},
    }
}

#[allow(unused)]
struct ProxyTestInteract {
    interactor: Interactor,
    adder_owner_address: Bech32Address,
    wallet_address: Bech32Address,
    state: State,
}

impl ProxyTestInteract {
    async fn init() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(config.gateway())
            .await
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;

        let adder_owner_address = interactor.register_wallet(test_wallets::alice());
        let wallet_address = interactor.register_wallet(test_wallets::alice());

        Self {
            interactor,
            adder_owner_address: adder_owner_address.into(),
            wallet_address: wallet_address.into(),
            state: State::load_state(),
        }
    }

    async fn set_state(&mut self) {
        println!("wallet address: {}", self.wallet_address);
        self.interactor
            .retrieve_account(&self.adder_owner_address)
            .await;
        self.interactor.retrieve_account(&self.wallet_address).await;
    }

    async fn deploy(&mut self) {
        // warning: multi deploy not yet fully supported
        // only works with last deployed address

        self.set_state().await;

        let new_address = self
            .interactor
            .tx()
            .from(&self.adder_owner_address)
            .gas(30_000_000)
            .typed(proxy_test_proxy::ProxyTestProxy)
            .init(0u32)
            .code(ADDER_CODE_PATH)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewBech32Address)
            .prepare_async()
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.set_adder_address(new_address);
    }

    async fn multi_deploy(&mut self, count: usize) {
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
                    .typed(proxy_test_proxy::ProxyTestProxy)
                    .init(0u32)
                    .code(ADDER_CODE_PATH)
                    .gas(30_000_000)
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
        self.set_state().await;
        println!("calling contract {count} times...");

        let mut buffer = self.interactor.homogenous_call_buffer();
        for _ in 0..count {
            buffer.push_tx(|tx| {
                tx.from(&self.wallet_address)
                    .to(self.state.current_adder_address())
                    .typed(proxy_test_proxy::ProxyTestProxy)
                    .add(value)
                    .gas(30_000_000)
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
            .gas(30_000_000)
            .typed(proxy_test_proxy::ProxyTestProxy)
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
            .typed(proxy_test_proxy::ProxyTestProxy)
            .sum()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("sum: {sum}");
    }

    async fn last_issued_token(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_adder_address())
            .typed(proxy_test_proxy::ProxyTestProxy)
            .last_issued_token()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn last_error_message(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_adder_address())
            .typed(proxy_test_proxy::ProxyTestProxy)
            .last_error_message()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn upgrade(&mut self, new_value: u32) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_adder_address())
            .gas(30_000_000)
            .typed(proxy_test_proxy::ProxyTestProxy)
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
            .typed(proxy_test_proxy::ProxyTestProxy)
            .sum()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;
        assert_eq!(sum, RustBigUint::from(new_value));

        println!("response: {response:?}");
    }

    async fn issue_fungible_token(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(50000000000000000u128);

        let token_display_name = ManagedBuffer::from("PROXYTOKEN");
        let token_ticker = ManagedBuffer::from("PRXY");
        let initial_supply = BigUint::<StaticApi>::from(100000000000000000000u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_adder_address())
            .gas(60_000_000u64)
            .typed(proxy_test_proxy::ProxyTestProxy)
            .issue_fungible_token(token_display_name, token_ticker, initial_supply)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Papacioc: {response:?}"); // asta e gol
    }

    async fn send_egld(&mut self) {
        let to = bech32::decode("");
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_adder_address())
            .gas(60_000_000u64)
            .typed(proxy_test_proxy::ProxyTestProxy)
            .send_egld(to, amount)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }
}

#[tokio::test]
#[ignore = "run on demand"]
async fn test() {
    let mut basic_interact = ProxyTestInteract::init().await;

    basic_interact.deploy().await;
    basic_interact.add(1u32).await;

    basic_interact.upgrade(7u32).await;
}
