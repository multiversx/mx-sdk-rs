mod basic_interact_cli;
mod basic_interact_config;
mod basic_interact_state;

use adder::adder_proxy;
use basic_interact_config::Config;
use basic_interact_state::State;
use clap::Parser;

use multiversx_sc_snippets::{imports::*, sdk::data::keystore::InsertPassword};

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";
const ERROR_NOT_OWNER: ExpectError = ExpectError(4u64, "upgrade is allowed only for owner");

const ADDER_CODE_PATH: MxscPath = MxscPath::new("../output/adder.mxsc.json");

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut basic_interact = AdderInteract::init().await;
    let accounts = Accounts::init(&mut basic_interact.interactor).await;
    basic_interact.set_state(&accounts).await;

    let cli = basic_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(basic_interact_cli::InteractCliCommand::Add(args)) => {
            if args.count == 1 {
                basic_interact
                    .add(args.value, &accounts.adder_owner_address)
                    .await;
            } else {
                basic_interact
                    .multi_add(args.value, args.count, &accounts.adder_owner_address)
                    .await;
            }
        },
        Some(basic_interact_cli::InteractCliCommand::Deploy) => {
            basic_interact.deploy(&accounts.adder_owner_address).await;
        },
        Some(basic_interact_cli::InteractCliCommand::Feed) => {
            basic_interact
                .feed_contract_egld(&accounts.adder_owner_address)
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::MultiDeploy(args)) => {
            basic_interact
                .multi_deploy(args.count, &accounts.adder_owner_address)
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::Sum) => {
            basic_interact.print_sum().await;
        },
        Some(basic_interact_cli::InteractCliCommand::Upgrade(args)) => {
            basic_interact
                .upgrade(args.value, &accounts.adder_owner_address, None)
                .await
        },
        None => {},
    }
}

#[allow(unused)]
struct AdderInteract {
    interactor: Interactor,
    state: State,
}

struct Accounts {
    adder_owner_address: Bech32Address,
    wallet_address: Bech32Address,
}

impl Accounts {
    async fn init(interactor: &mut Interactor) -> Self {
        let adder_owner_address =
            interactor.register_wallet(Wallet::from_pem_file("adder-owner.pem").unwrap());
        // PASSWORD: "alice"
        // InsertPassword::Plaintext("alice".to_string()) || InsertPassword::StandardInput
        let wallet_address = interactor.register_wallet(
            Wallet::from_keystore_secret(
                "alice.json",
                InsertPassword::Plaintext("alice".to_string()),
            )
            .unwrap(),
        );

        Self {
            adder_owner_address: adder_owner_address.into(),
            wallet_address: wallet_address.into(),
        }
    }
}

impl AdderInteract {
    async fn init() -> Self {
        let config = Config::load_config();
        let interactor = Interactor::new(config.gateway())
            .await
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;

        Self {
            interactor,
            state: State::load_state(),
        }
    }

    async fn set_state(&mut self, accounts: &Accounts) {
        println!("wallet address: {}", accounts.wallet_address);
        self.interactor
            .retrieve_account(&accounts.adder_owner_address)
            .await;
        self.interactor
            .retrieve_account(&accounts.wallet_address)
            .await;
    }

    async fn deploy(&mut self, from: &Bech32Address) {
        // warning: multi deploy not yet fully supported
        // only works with last deployed address

        let new_address = self
            .interactor
            .tx()
            .from(from)
            .gas(6_000_000)
            .typed(adder_proxy::AdderProxy)
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

    async fn multi_deploy(&mut self, count: usize, from: &Bech32Address) {
        if count == 0 {
            println!("count must be greater than 0");
            return;
        }

        println!("deploying {count} contracts...");

        let mut buffer = self.interactor.homogenous_call_buffer();
        for _ in 0..count {
            buffer.push_tx(|tx| {
                tx.from(from)
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

    async fn multi_add(&mut self, value: u32, count: usize, from: &Bech32Address) {
        println!("calling contract {count} times...");

        let mut buffer = self.interactor.homogenous_call_buffer();
        for _ in 0..count {
            buffer.push_tx(|tx| {
                tx.from(from)
                    .to(self.state.current_adder_address())
                    .typed(adder_proxy::AdderProxy)
                    .add(value)
                    .gas(6_000_000)
            });
        }

        let _ = buffer.run().await;

        println!("successfully performed add {count} times");
    }

    async fn feed_contract_egld(&mut self, from: &Bech32Address) {
        self.interactor
            .tx()
            .from(from)
            .to(self.state.current_adder_address())
            .egld(NumExpr("0,050000000000000000"))
            .prepare_async()
            .run()
            .await;
    }

    async fn add(&mut self, value: u32, from: &Bech32Address) {
        self.interactor
            .tx()
            .from(from)
            .to(self.state.current_adder_address())
            .gas(6_000_000)
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

    async fn upgrade(
        &mut self,
        new_value: u32,
        sender: &Bech32Address,
        expected_result: Option<ExpectError<'_>>,
    ) {
        let transaction = self
            .interactor
            .tx()
            .from(sender)
            .to(self.state.current_adder_address())
            .gas(6_000_000)
            .typed(adder_proxy::AdderProxy)
            .upgrade(BigUint::from(new_value))
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .code(ADDER_CODE_PATH);

        match expected_result {
            Some(error) => {
                transaction.returns(error).prepare_async().run().await;
            },
            None => {
                transaction.prepare_async().run().await;

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
            },
        }
    }
}

#[tokio::test]
#[ignore = "run on demand"]
async fn upgrade_test() {
    let mut basic_interact = AdderInteract::init().await;
    let accounts = Accounts::init(&mut basic_interact.interactor).await;
    basic_interact.set_state(&accounts).await;

    basic_interact.deploy(&accounts.adder_owner_address).await;
    basic_interact
        .add(1u32, &accounts.adder_owner_address)
        .await;

    // Sum will be 1
    basic_interact.print_sum().await;

    basic_interact
        .upgrade(7u32, &accounts.adder_owner_address, None)
        .await;

    // Sum will be the updated value of 7
    basic_interact.print_sum().await;

    basic_interact
        .upgrade(10u32, &accounts.wallet_address, Some(ERROR_NOT_OWNER))
        .await;

    // Sum will remain 7
    basic_interact.print_sum().await;
}
