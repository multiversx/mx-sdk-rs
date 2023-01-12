mod multisig_interact_nfts;
use multisig::{
    multisig_perform::ProxyTrait as _, multisig_propose::ProxyTrait as _,
    multisig_state::ProxyTrait as _, ProxyTrait as _,
};
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
use std::{
    env::Args,
    io::{Read, Write},
};

const GATEWAY: &str = multiversx_sc_snippets::erdrs::blockchain::TESTNET_GATEWAY;
const PEM: &str = "alice.pem";
const DEFAULT_MULTISIG_ADDRESS_EXPR: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";
const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";

type MultisigContract = ContractInfo<multisig::Proxy<DebugApi>>;

#[tokio::main]
async fn main() {
    env_logger::init();
    let _ = DebugApi::dummy();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut state = State::init(args).await;
    match cmd.as_str() {
        "deploy" => state.deploy().await,
        "feed" => state.feed_contract_egld().await,
        "nft-full" => state.issue_multisig_and_collection_full().await,
        "nft-issue" => state.issue_collection().await,
        "nft-special" => state.set_special_role().await,
        "nft-items" => state.create_items().await,
        "quorum" => state.print_quorum().await,
        "board" => state.print_board().await,
        "dns-register" => state.dns_register().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

struct State {
    interactor: Interactor,
    wallet_address: Address,
    multisig: MultisigContract,
    system_sc_address: Address,
    collection_token_identifier: String,
    #[allow(dead_code)]
    args: Args,
}

impl State {
    async fn init(args: Args) -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(Wallet::from_pem_file(PEM).unwrap());
        let multisig = MultisigContract::new(load_address_expr());
        State {
            interactor,
            wallet_address,
            multisig,
            system_sc_address: bech32::decode(SYSTEM_SC_BECH32),
            collection_token_identifier: multisig_interact_nfts::COLLECTION_TOKEN_IDENTIFIER
                .to_string(),
            args,
        }
    }

    async fn deploy(&mut self) {
        let deploy_result: multiversx_sc_snippets::InteractorResult<()> = self
            .interactor
            .sc_deploy(
                self.multisig
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
        let new_address = deploy_result.new_deployed_address();
        let new_address_bech32 = bech32::encode(&new_address);
        println!("new address: {new_address_bech32}");
        let new_address_expr = format!("bech32:{new_address_bech32}");
        save_address_expr(new_address_expr.as_str());
        self.multisig = MultisigContract::new(new_address_expr);
    }

    async fn feed_contract_egld(&mut self) {
        let _ = self
            .interactor
            .transfer(
                TransferStep::new()
                    .from(&self.wallet_address)
                    .to(&self.multisig)
                    .egld_value("0,050000000000000000"),
            )
            .await;
    }

    fn perform_action_step(&mut self, action_id: usize, gas_expr: &str) -> ScCallStep {
        self.multisig
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
        let quorum: SingleValue<usize> = self.interactor.vm_query(self.multisig.quorum()).await;

        println!("quorum: {}", quorum.into());
    }

    async fn get_action_last_index(&mut self) -> usize {
        self.interactor
            .vm_query(self.multisig.get_action_last_index())
            .await
    }

    async fn print_board(&mut self) {
        let board_members: MultiValueVec<Address> = self
            .interactor
            .vm_query(self.multisig.get_all_board_members())
            .await;

        println!("board members:");
        for board_member in board_members.iter() {
            println!("    {}", bech32::encode(board_member));
        }
    }

    async fn dns_register(&mut self) {
        let name = self.args.next().expect("name argument missing");
        let dns_address = dns_address_for_name(&name);
        let dns_register_call: ScCallStep = self
            .multisig
            .dns_register(dns_address, name)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit("30,000,000")
            .into();
        self.interactor.sc_call(dns_register_call).await;
    }
}

const SAVED_ADDRESS_FILE_NAME: &str = "multisig_address.txt";

fn load_address_expr() -> String {
    match std::fs::File::open(SAVED_ADDRESS_FILE_NAME) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            contents
        },
        Err(_) => DEFAULT_MULTISIG_ADDRESS_EXPR.to_string(),
    }
}

fn save_address_expr(address_expr: &str) {
    let mut file = std::fs::File::create(SAVED_ADDRESS_FILE_NAME).unwrap();
    file.write_all(address_expr.as_bytes()).unwrap();
}
