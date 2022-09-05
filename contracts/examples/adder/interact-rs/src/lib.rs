#[allow(non_snake_case)]

use adder::ProxyTrait as _;
use elrond_interact_snippets::{
    elrond_wasm::{
        elrond_codec::multi_types::{MultiValueVec, TopDecode},
        storage::mappers::SingleValue,
        types::{Address, CodeMetadata},
    },
    elrond_wasm_debug::{
        bech32, mandos::interpret_trait::InterpreterContext, mandos_system::model::*, ContractInfo,
        DebugApi,
    },
    env_logger,
    erdrs::interactors::wallet::Wallet,
    tokio, Interactor,
};
use std::{
    env::Args,
    io::{Read, Write},
};

const GATEWAY: &str = elrond_interact_snippets::erdrs::blockchain::rpc::DEVNET_GATEWAY;
const PEM: &str = "alice.pem";
const SC_ADDRESS: &str = "";

const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";
const DEFAULT_ADDRESS_EXPR: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";
const DEFAULT_GAS_LIMIT: u64 = 100_000_000;
const TOKEN_ISSUE_COST: u64 = 50_000_000_000_000_000;

type ContractType = ContractInfo<adder::Proxy<DebugApi>>;

#[tokio::main]
async fn main() {
    env_logger::init();
    let _ = DebugApi::dummy();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut state = State::new().await;
    match cmd.as_str() {
        "deploy" => state.deploy().await,
        "upgrade" => state.upgrade().await,
        "getSum" => state.getSum().await,
        "add" => state.add().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

struct State {
    interactor: Interactor,
    wallet_address: Address,
    contract: ContractType,
}

impl State {
    async fn new() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(Wallet::from_pem_file(PEM).unwrap());
        let sc_addr_expr = if SC_ADDRESS == "" {
            DEFAULT_ADDRESS_EXPR.to_string()
        } else {
            "bech32:".to_string() + SC_ADDRESS
        };
        let contract = ContractType::new(sc_addr_expr);

        State {
            interactor,
            wallet_address,
            contract,
        }
    }

    async fn getSum(&mut self) {
        let sc_addr = self.contract.address.clone().into_option().unwrap();
        let mut contract_call =
            ContractCall::<DebugApi, MultiValueVec<Vec<u8>>>::new(sc_addr, "getSum");
        let b_call: ScCallStep = contract_call
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT)
            .into();
        let results: InteractorResult<MultiValueVec<Vec<u8>>> =
        self.interactor.sc_call_get_result(b_call).await;

        let raw_result_values = results.value().0;
        let out0 = BigUint::top_decode(raw_result_values[0]).unwrap();

        println!("out0: {}", out0)

    }

    async fn add(&mut self) {
        let value: BigUint = Default::default();

        let sc_addr = self.contract.address.clone().into_option().unwrap();
        let mut contract_call =
            ContractCall::<DebugApi, MultiValueVec<Vec<u8>>>::new(sc_addr, "add");
        contract_call.push_endpoint_arg(&value);
        let b_call: ScCallStep = contract_call
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT)
            .into();
        self.interactor.sc_call(b_call).await;
    }

}
