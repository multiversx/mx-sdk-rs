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
use multiversx_sc_scenario::test_wallets;
use multiversx_sc_snippets::{
    dns_address_for_name, env_logger,
    multiversx_sc::{
        codec::multi_types::MultiValueVec,
        storage::mappers::SingleValue,
        types::{Address, CodeMetadata},
    },
    multiversx_sc_scenario::{
        bech32, scenario_format::interpret_trait::InterpreterContext, scenario_model::*,
        ContractInfo, DebugApi,
    },
    tokio, Interactor, StepBuffer,
};

const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";
const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";

#[tokio::main]
async fn main() {
    DebugApi::dummy();
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
        Some(multisig_interact_cli::InteractCliCommand::NftFull) => {
            multisig_interact.issue_multisig_and_collection_full().await;
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

        Self {
            interactor,
            wallet_address,
            system_sc_address: bech32::decode(SYSTEM_SC_BECH32),
            collection_token_identifier: multisig_interact_nfts::COLLECTION_TOKEN_IDENTIFIER
                .to_string(),
            state: State::load_state(),
        }
    }

    fn register_wallets(&mut self) {
        let bob = test_wallets::bob();
        let carol = test_wallets::carol();
        let dan = test_wallets::dan();
        let eve = test_wallets::eve();

        for wallet in vec![bob, carol, dan, eve] {
            self.interactor.register_wallet(wallet);
        }
    }

    async fn deploy(&mut self) {
        let board = self.board();
        let deploy_result: multiversx_sc_snippets::InteractorResult<()> = self
            .interactor
            .sc_deploy(
                self.state
                    .default_multisig()
                    .init(Config::load_config().quorum(), board)
                    .into_blockchain_call()
                    .from(&self.wallet_address)
                    .code_metadata(CodeMetadata::all())
                    .contract_code(
                        "file:../output/multisig.wasm",
                        &InterpreterContext::default(),
                    )
                    .gas_limit("70,000,000")
                    .expect(TxExpect::ok()),
            )
            .await;

        let result = deploy_result.new_deployed_address();
        if result.is_err() {
            println!("deploy failed: {}", result.err().unwrap());
            return;
        }

        self.wegld_swap_set_state().await;

        let new_address_bech32 = bech32::encode(&result.unwrap());
        println!("new address: {new_address_bech32}");

        let new_address_expr = format!("bech32:{new_address_bech32}");
        self.state.set_multisig_address(&new_address_expr);
    }

    async fn multi_deploy(&mut self, count: &u8) {
        if *count == 0 {
            println!("count must be greater than 0");
            return;
        }
        println!("deploying {count} contracts...");

        let board = self.board();
        let mut steps = Vec::new();
        for _ in 0..*count {
            let sc_deploy_step: ScDeployStep = self
                .state
                .default_multisig()
                .init(Config::load_config().quorum(), board.clone())
                .into_blockchain_call()
                .from(&self.wallet_address)
                .code_metadata(CodeMetadata::all())
                .contract_code(
                    "file:../output/multisig.wasm",
                    &InterpreterContext::default(),
                )
                .gas_limit("70,000,000")
                .expect(TxExpect::ok())
                .into();

            steps.push(sc_deploy_step);
        }

        let results = self.interactor.multiple_sc_deploy_results(&steps).await;
        for result in results {
            let result = result.new_deployed_address();
            if result.is_err() {
                println!("deploy failed: {}", result.err().unwrap());
                return;
            }

            let new_address_bech32 = bech32::encode(&result.unwrap());
            println!("new address: {new_address_bech32}");
        }
    }

    fn board(&mut self) -> MultiValueVec<Address> {
        let bob = test_wallets::bob();
        let carol = test_wallets::carol();
        let dan = test_wallets::dan();
        let eve = test_wallets::eve();

        MultiValueVec::from([
            self.wallet_address.clone(),
            bob.address().to_bytes().into(),
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

        let mut typed_sc_call = self
            .state
            .multisig()
            .perform_action_endpoint(action_id)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(gas_expr);

        self.interactor.sc_call_get_result(&mut typed_sc_call).await;

        let result = typed_sc_call
            .sc_call_step
            .response
            .unwrap()
            .handle_signal_error_event();
        if result.is_err() {
            println!(
                "perform action `{action_id}` failed with: {}",
                result.err().unwrap()
            );
            return;
        }
        println!("successfully performed action `{action_id}`");
    }

    async fn quorum_reached(&mut self, action_id: usize) -> bool {
        self.interactor
            .vm_query(self.state.multisig().quorum_reached(action_id))
            .await
    }

    async fn signed(&mut self, signer: &Address, action_id: usize) -> bool {
        self.interactor
            .vm_query(self.state.multisig().signed(signer, action_id))
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

            let sc_call_step: ScCallStep = self
                .state
                .multisig()
                .sign(action_id)
                .into_blockchain_call()
                .from(signer)
                .gas_limit("15,000,000")
                .into();

            steps.push(sc_call_step);
        }

        self.interactor
            .multiple_exec(StepBuffer::from_vec(&mut steps))
            .await;

        for step in steps.iter() {
            let response = step.response.clone().unwrap();
            let result = response.handle_signal_error_event();
            if result.is_err() {
                println!(
                    "perform sign `{action_id}` failed with: {}",
                    result.err().unwrap()
                );
                return false;
            }
        }

        println!("successfully performed sign action `{action_id}`");
        true
    }

    async fn print_quorum(&mut self) {
        let quorum: SingleValue<usize> = self
            .interactor
            .vm_query(self.state.multisig().quorum())
            .await;

        println!("quorum: {}", quorum.into());
    }

    async fn get_action_last_index(&mut self) -> usize {
        self.interactor
            .vm_query(self.state.multisig().get_action_last_index())
            .await
    }

    async fn print_board(&mut self) {
        let board: SingleValue<usize> = self
            .interactor
            .vm_query(self.state.multisig().num_board_members())
            .await;

        println!("board: {}", board.into());
    }

    async fn dns_register(&mut self, name: &str) {
        let dns_address = dns_address_for_name(name);
        let dns_register_call: ScCallStep = self
            .state
            .multisig()
            .dns_register(dns_address, name)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit("30,000,000")
            .into();
        self.interactor.sc_call_and_forget(dns_register_call).await;
    }
}
