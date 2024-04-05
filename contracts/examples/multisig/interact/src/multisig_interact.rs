mod multisig_interact_cli;
mod multisig_interact_config;
mod multisig_interact_nfts;
mod multisig_interact_state;
mod multisig_interact_wegld;

use clap::Parser;
use multisig::{multisig_perform::ProxyTrait as _, multisig_proxy, ProxyTrait as _};
use multisig_interact_config::Config;
use multisig_interact_state::State;
use multiversx_sc_scenario::{
    mandos_system::ScenarioRunner,
    multiversx_sc::{
        imports::OptionalValue,
        types::{BigUint, ReturnsNewAddress, ReturnsResult},
    },
    scenario_format::interpret_trait::InterpretableFrom,
    standalone::retrieve_account_as_scenario_set_state,
    test_wallets, NumExpr,
};
use multiversx_sc_snippets::{
    dns_address_for_name, env_logger,
    multiversx_sc::{codec::multi_types::MultiValueVec, types::Address},
    multiversx_sc_scenario::{
        api::StaticApi, bech32, scenario_format::interpret_trait::InterpreterContext,
        scenario_model::*, ContractInfo,
    },
    tokio, Interactor, InteractorPrepareAsync, StepBuffer,
};

const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";
const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut multisig_interact = MultisigInteract::init().await;
    multisig_interact.register_wallets();

    let cli = multisig_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(multisig_interact_cli::InteractCliCommand::Board) => {
            multisig_interact.print_board().await;
        },
        Some(multisig_interact_cli::InteractCliCommand::Deploy) => {
            multisig_interact.deploy().await;
        },
        Some(multisig_interact_cli::InteractCliCommand::DnsRegister(args)) => {
            multisig_interact.dns_register(&args.name).await;
        },
        Some(multisig_interact_cli::InteractCliCommand::Feed) => {
            multisig_interact.feed_contract_egld().await;
        },
        Some(multisig_interact_cli::InteractCliCommand::MultiDeploy(args)) => {
            multisig_interact.multi_deploy(&args.count).await;
        },
        Some(multisig_interact_cli::InteractCliCommand::NftFullAllRoles) => {
            multisig_interact
                .issue_multisig_and_collection_with_all_roles_full()
                .await;
        },
        Some(multisig_interact_cli::InteractCliCommand::NftFull) => {
            multisig_interact.issue_multisig_and_collection_full().await;
        },
        Some(multisig_interact_cli::InteractCliCommand::NftIssueAllRoles) => {
            multisig_interact.issue_collection_with_all_roles().await;
        },
        Some(multisig_interact_cli::InteractCliCommand::NftIssue) => {
            multisig_interact.issue_collection().await;
        },
        Some(multisig_interact_cli::InteractCliCommand::NftItems) => {
            multisig_interact.create_items().await;
        },
        Some(multisig_interact_cli::InteractCliCommand::NftSpecial) => {
            multisig_interact.set_special_role().await;
        },
        Some(multisig_interact_cli::InteractCliCommand::Quorum) => {
            multisig_interact.print_quorum().await;
        },
        Some(multisig_interact_cli::InteractCliCommand::UnwrapEgld) => {
            multisig_interact.unwrap_egld().await;
        },
        Some(multisig_interact_cli::InteractCliCommand::WEgldSwapFull) => {
            multisig_interact.wegld_swap_full().await;
        },
        Some(multisig_interact_cli::InteractCliCommand::WrapEgld) => {
            multisig_interact.wrap_egld().await;
        },
        None => {},
    }
}

struct MultisigInteract {
    interactor: Interactor,
    wallet_address: Address,
    system_sc_address: Address,
    collection_token_identifier: String,
    multisig_code: BytesValue,
    state: State,
}

impl MultisigInteract {
    async fn init() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(config.gateway())
            .await
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;
        let wallet_address = interactor.register_wallet(test_wallets::mike());
        let multisig_code = BytesValue::interpret_from(
            "mxsc:../output/multisig.mxsc.json",
            &InterpreterContext::default(),
        );

        Self {
            interactor,
            wallet_address,
            system_sc_address: bech32::decode(SYSTEM_SC_BECH32),
            collection_token_identifier: String::new(),
            multisig_code,
            state: State::load_state(),
        }
    }

    fn register_wallets(&mut self) {
        let carol = test_wallets::carol();
        let dan = test_wallets::dan();
        let eve = test_wallets::eve();

        for wallet in &[carol, dan, eve] {
            self.interactor.register_wallet(*wallet);
        }
    }

    async fn set_state(&mut self) {
        for board_member_address in self.board().iter() {
            println!(
                "board member address: {}",
                bech32::encode(board_member_address)
            );
            let scenario_raw = retrieve_account_as_scenario_set_state(
                Config::load_config().gateway().to_string(),
                bech32::encode(board_member_address),
                true,
            )
            .await;

            let scenario = Scenario::interpret_from(scenario_raw, &InterpreterContext::default());

            self.interactor.pre_runners.run_scenario(&scenario);
            self.interactor.post_runners.run_scenario(&scenario);
        }

        self.wegld_swap_set_state().await;
    }

    async fn deploy(&mut self) {
        self.set_state().await;

        let board = self.board();

        let quorum = Config::load_config().quorum();
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .typed(multisig_proxy::MultisigProxy)
            .init(quorum, board)
            .code(&self.multisig_code)
            .gas(NumExpr("100,000,000"))
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;

        let new_address_bech32 = bech32::encode(&new_address);
        println!("new address: {new_address_bech32}");

        let new_address_expr = format!("bech32:{new_address_bech32}");
        self.state.set_multisig_address(&new_address_expr);
    }

    async fn multi_deploy(&mut self, count: &u8) {
        if *count == 0 {
            println!("count must be greater than 0");
            return;
        }
        self.set_state().await;
        println!("deploying {count} contracts...");

        let board = self.board();
        let quorum = Config::load_config().quorum();
        let mut buffer = self.interactor.homogenous_call_buffer();
        for _ in 0..*count {
            buffer.push_tx(|tx| {
                tx.from(&self.wallet_address)
                    .typed(multisig_proxy::MultisigProxy)
                    .init(quorum, board.clone())
                    .code(&self.multisig_code)
                    .gas(NumExpr("70,000,000"))
                    .returns(ReturnsNewAddress)
            });
        }

        let results = buffer.run().await;
        for result in results {
            let new_address_bech32 = bech32::encode(&result);
            println!("new address: {new_address_bech32}");

            let new_address_expr = format!("bech32:{new_address_bech32}");
            self.state.set_multisig_address(&new_address_expr);
        }
    }

    fn board(&mut self) -> MultiValueVec<Address> {
        let carol = test_wallets::carol();
        let dan = test_wallets::dan();
        let eve = test_wallets::eve();

        MultiValueVec::from([
            self.wallet_address.clone(),
            carol.address().to_bytes().into(),
            dan.address().to_bytes().into(),
            eve.address().to_bytes().into(),
        ])
    }

    async fn feed_contract_egld(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.multisig().to_address())
            .egld(BigUint::from(50_000_000_000_000_000u64)) // 0,05 or 5 * 10^16
            .prepare_async()
            .run()
            .await;
    }

    async fn perform_action(&mut self, action_id: usize, gas_expr: u64) {
        if !self.quorum_reached(action_id).await && !self.sign(action_id).await {
            return;
        }
        println!("quorum reached for action `{action_id}`");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.multisig().to_address())
            .gas(gas_expr)
            .typed(multisig_proxy::MultisigProxy)
            .perform_action_endpoint(action_id)
            .prepare_async()
            .run()
            .await;

        println!("successfully performed action `{action_id}`");
    }

    async fn perform_actions(&mut self, actions: Vec<usize>, gas_expr: u64) {
        let mut buffer = self.interactor.homogenous_call_buffer();
        let multisig_address = self.state.multisig().to_address();
        let from = &self.wallet_address;

        for &action_id in actions.iter() {
            if self.quorum_reached(action_id).await && self.sign(action_id).await {
                buffer.push_tx(|tx| {
                    tx.from(from)
                        .to(&multisig_address)
                        .gas(gas_expr)
                        .typed(multisig_proxy::MultisigProxy)
                        .perform_action_endpoint(action_id)
                        .returns(ReturnsResult)
                });
            }
        }

        let deployed_addresses = buffer.run().await;

        for (action_id, address) in deployed_addresses.iter().enumerate() {
            println!("successfully performed action `{action_id}`");
            if address.is_some() {
                println!(
                    "new deployed address for action `{action_id}: {:#?}`",
                    address.clone().into_option().unwrap()
                )
            }
        }
    }

    async fn quorum_reached(&mut self, action_id: usize) -> bool {
        self.interactor
            .query()
            .to(self.state.multisig().to_address())
            .typed(multisig_proxy::MultisigProxy)
            .quorum_reached(action_id)
            .returns(ReturnsResult)
            .prepare_async()
            .run()
            .await
    }

    async fn signed(&mut self, signer: &Address, action_id: usize) -> bool {
        self.interactor
            .query()
            .to(self.state.multisig().to_address())
            .typed(multisig_proxy::MultisigProxy)
            .signed(signer, action_id)
            .returns(ReturnsResult)
            .prepare_async()
            .run()
            .await
    }

    async fn sign(&mut self, action_id: usize) -> bool {
        println!("signing action `{action_id}`...");
        let mut buffer = self.interactor.homogenous_call_buffer();
        let multisig_address = self.state.multisig().to_address();

        for signer in self.board().iter() {
            if self.signed(signer, action_id).await {
                println!(
                    "{} - already signed action `{action_id}`",
                    bech32::encode(signer)
                );
                continue;
            }

            buffer.push_tx(|tx| {
                tx.from(signer)
                    .to(&multisig_address)
                    .gas(15_000_000u64)
                    .typed(multisig_proxy::MultisigProxy)
                    .sign(action_id)
            });
        }

        buffer.run().await;

        println!("successfully performed sign action `{action_id}`");
        true
    }

    async fn dns_register(&mut self, name: &str) {
        let dns_address = dns_address_for_name(name);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.multisig().to_address())
            .gas(NumExpr("30,000,000"))
            .typed(multisig_proxy::MultisigProxy)
            .dns_register(dns_address, name)
            .prepare_async()
            .run()
            .await;

        println!("successfully registered dns");
    }

    async fn print_quorum(&mut self) {
        let quorum = self
            .interactor
            .query()
            .to(self.state.multisig().to_address())
            .typed(multisig_proxy::MultisigProxy)
            .quorum()
            .returns(ReturnsResult)
            .prepare_async()
            .run()
            .await;

        println!("quorum: {}", quorum);
    }

    async fn print_board(&mut self) {
        let board = self
            .interactor
            .query()
            .to(self.state.multisig().to_address())
            .typed(multisig_proxy::MultisigProxy)
            .num_board_members()
            .returns(ReturnsResult)
            .prepare_async()
            .run()
            .await;

        println!("board: {}", board);
    }
}
