mod multisig_interact_cli;
mod multisig_interact_config;
mod multisig_interact_nfts;
mod multisig_interact_state;

use clap::Parser;
use multisig::{
    multisig_perform::ProxyTrait as _, multisig_propose::ProxyTrait as _,
    multisig_state::ProxyTrait as _, ProxyTrait as _,
};
use multisig_interact_config::Config;
use multisig_interact_state::State;
use multiversx_sc_modules::dns::ProxyTrait as _;
use multiversx_sc_snippets::{
    dns_address_for_name, env_logger,
    erdrs::wallet::Wallet,
    multiversx_sc::{
        codec::multi_types::MultiValueVec,
        storage::mappers::SingleValue,
        types::{Address, CodeMetadata},
    },
    multiversx_sc_scenario::{
        bech32, scenario_format::interpret_trait::InterpreterContext, scenario_model::*,
        ContractInfo, DebugApi,
    },
    tokio, Interactor,
};

const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";

#[tokio::main]
async fn main() {
    DebugApi::dummy();
    env_logger::init();

    let mut multisig_interact = MultisigInteract::init().await;

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
        let mut interactor = Interactor::new(config.gateway()).await;
        let wallet_address =
            interactor.register_wallet(Wallet::from_pem_file(config.pem()).unwrap());
        MultisigInteract {
            interactor,
            wallet_address,
            system_sc_address: bech32::decode(SYSTEM_SC_BECH32),
            collection_token_identifier: multisig_interact_nfts::COLLECTION_TOKEN_IDENTIFIER
                .to_string(),
            state: State::load_state(),
        }
    }

    async fn deploy(&mut self) {
        let deploy_result: multiversx_sc_snippets::InteractorResult<()> = self
            .interactor
            .sc_deploy(
                self.state
                    .default_multisig()
                    .init(0usize, MultiValueVec::from([self.wallet_address.clone()]))
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

        let new_address_bech32 = bech32::encode(&result.unwrap());
        println!("new address: {new_address_bech32}");

        let new_address_expr = format!("bech32:{new_address_bech32}");
        self.state.set_multisig_address(&new_address_expr);
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

    fn perform_action_step(&mut self, action_id: usize, gas_expr: &str) -> ScCallStep {
        self.state
            .multisig()
            .perform_action_endpoint(action_id)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(gas_expr)
            .into()
    }

    async fn perform_action(&mut self, action_id: usize, gas_expr: &str) {
        let sc_call_step = self.perform_action_step(action_id, gas_expr);
        let _ = self.interactor.sc_call_get_raw_result(sc_call_step).await;
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
        let board_members: MultiValueVec<Address> = self
            .interactor
            .vm_query(self.state.multisig().get_all_board_members())
            .await;

        println!("board members:");
        for board_member in board_members.iter() {
            println!("    {}", bech32::encode(board_member));
        }
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
        self.interactor.sc_call(dns_register_call).await;
    }
}
