mod multisig_interact_cli;
mod multisig_interact_config;
mod multisig_interact_nfts;
mod multisig_interact_state;
mod multisig_interact_wegld;

use clap::Parser;
use multisig::{
    multisig_perform::ProxyTrait as _, multisig_propose::ProxyTrait as _,
    multisig_state::ProxyTrait as _, ProxyTrait as _,
};
use multisig_interact_config::Config;
use multisig_interact_state::State;
use multiversx_sc_modules::dns::ProxyTrait as _;
use multiversx_sc_scenario::{
    mandos_system::ScenarioRunner, multiversx_sc::codec::multi_types::IgnoreValue,
    scenario_format::interpret_trait::InterpretableFrom,
    standalone::retrieve_account_as_scenario_set_state, test_wallets,
};
use multiversx_sc_snippets::{
    dns_address_for_name, env_logger,
    multiversx_sc::{
        codec::multi_types::MultiValueVec, storage::mappers::SingleValue, types::Address,
    },
    multiversx_sc_scenario::{
        api::StaticApi, bech32, scenario_format::interpret_trait::InterpreterContext,
        scenario_model::*, ContractInfo,
    },
    tokio, Interactor, StepBuffer,
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
            "file:../output/multisig.wasm",
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
        let (new_address, _) = self
            .interactor
            .sc_deploy_get_result::<_, IgnoreValue>(
                ScDeployStep::new()
                    .call(
                        self.state
                            .default_multisig()
                            .init(Config::load_config().quorum(), board),
                    )
                    .from(&self.wallet_address)
                    .code(&self.multisig_code)
                    .gas_limit("100,000,000")
                    .expect(TxExpect::ok().additional_error_message("deploy failed: ")),
            )
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
        let mut steps = Vec::new();
        for _ in 0..*count {
            let typed_sc_deploy = ScDeployStep::new()
                .call(
                    self.state
                        .default_multisig()
                        .init(Config::load_config().quorum(), board.clone()),
                )
                .from(&self.wallet_address)
                .code(&self.multisig_code)
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

                let new_address_expr = format!("bech32:{new_address_bech32}");
                self.state.set_multisig_address(&new_address_expr);
            } else {
                println!("deploy failed");
                return;
            }
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
        let _ = self
            .interactor
            .transfer(
                TransferStep::new()
                    .from(&self.wallet_address)
                    .to(self.state.multisig())
                    .egld_value("0,050000000000000000"),
            )
            .await;
    }

    async fn perform_action(&mut self, action_id: usize, gas_expr: &str) {
        if !self.quorum_reached(action_id).await && !self.sign(action_id).await {
            return;
        }
        println!("quorum reached for action `{action_id}`");

        self.interactor
            .sc_call(
                ScCallStep::new()
                    .call(self.state.multisig().perform_action_endpoint(action_id))
                    .from(&self.wallet_address)
                    .gas_limit(gas_expr)
                    .expect(TxExpect::ok().additional_error_message(format!(
                        "perform action `{action_id}` failed with: "
                    ))),
            )
            .await;

        println!("successfully performed action `{action_id}`");
    }

    async fn perform_actions(&mut self, actions: Vec<usize>, gas_expr: &str) {
        let mut steps = Vec::new();
        for action_id in actions.iter() {
            if !self.quorum_reached(*action_id).await && !self.sign(*action_id).await {
                continue;
            }
            println!("quorum reached for action `{action_id}`");

            let typed_sc_call = ScCallStep::new()
                .call(self.state.multisig().perform_action_endpoint(action_id))
                .from(&self.wallet_address)
                .gas_limit(gas_expr);

            steps.push(typed_sc_call);
        }

        self.interactor
            .multi_sc_exec(StepBuffer::from_sc_call_vec(&mut steps))
            .await;

        for (i, action_id) in actions.iter().enumerate() {
            if !steps[i].response().is_success() {
                println!(
                    "perform action `{action_id}` failed with: {}",
                    steps[i].response().tx_error
                );
                continue;
            }

            println!("successfully performed action `{action_id}`");
        }
    }

    async fn quorum_reached(&mut self, action_id: usize) -> bool {
        self.interactor
            .quick_query(self.state.multisig().quorum_reached(action_id))
            .await
    }

    async fn signed(&mut self, signer: &Address, action_id: usize) -> bool {
        self.interactor
            .quick_query(self.state.multisig().signed(signer, action_id))
            .await
    }

    async fn sign(&mut self, action_id: usize) -> bool {
        println!("signing action `{action_id}`...");
        let mut steps = Vec::new();
        for signer in self.board().iter() {
            if self.signed(signer, action_id).await {
                println!(
                    "{} - already signed action `{action_id}`",
                    bech32::encode(signer)
                );
                continue;
            }

            let typed_sc_call = ScCallStep::new()
                .call(self.state.multisig().sign(action_id))
                .from(signer)
                .gas_limit("15,000,000");

            steps.push(typed_sc_call);
        }

        self.interactor
            .multi_sc_exec(StepBuffer::from_sc_call_vec(&mut steps))
            .await;

        for step in steps.iter() {
            if !step.response().is_success() {
                println!(
                    "perform sign `{action_id}` failed with: {}",
                    step.response().tx_error
                );
                return false;
            }
        }

        println!("successfully performed sign action `{action_id}`");
        true
    }

    async fn dns_register(&mut self, name: &str) {
        let dns_address = dns_address_for_name(name);
        self.interactor
            .sc_call(
                ScCallStep::new()
                    .call(self.state.multisig().dns_register(dns_address, name))
                    .from(&self.wallet_address)
                    .gas_limit("30,000,000")
                    .expect(TxExpect::ok().additional_error_message("dns register failed with: ")),
            )
            .await;

        println!("successfully registered dns");
    }

    async fn print_quorum(&mut self) {
        let quorum: SingleValue<usize> = self
            .interactor
            .quick_query(self.state.multisig().quorum())
            .await;

        println!("quorum: {}", quorum.into());
    }

    async fn print_board(&mut self) {
        let board: SingleValue<usize> = self
            .interactor
            .quick_query(self.state.multisig().num_board_members())
            .await;

        println!("board: {}", board.into());
    }
}
