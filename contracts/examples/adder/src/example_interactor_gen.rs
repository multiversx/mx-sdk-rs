use adder::ProxyTrait as _;
use elrond_interact_snippets::{
    elrond_wasm::{
        elrond_codec::multi_types::MultiValueVec,
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

// should probably be saved into a user-friendly config file
// also, have a default config file structure and path, which users can extend with custom variables
const GATEWAY: &str = elrond_interact_snippets::erdrs::blockchain::rpc::TESTNET_GATEWAY;
const PEM: &str = "alice.pem";
const SC_ADDRESS: &str = "";

// can remain const
const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";
const DEFAULT_MULTISIG_ADDRESS_EXPR: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";

type AdderContract = ContractInfo<adder::Proxy<DebugApi>>;

#[tokio::main]
async fn main() {
    env_logger::init();
    let _ = DebugApi::dummy();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut state = State::new().await;
    match cmd.as_str() {
        "deploy" => state.deploy(args).await,
        "upgrade" => state.upgrade(args).await,
        "add" => state.add(args).await,
        "getSum" => state.getSum(args).await,
        // Maybe also add some common functions, like issue/set roles ESDT etc.
        _ => panic!("unknown command: {}", &cmd),
    }
}

struct State {
    interactor: Interactor,
    wallet_address: Address,
    contract: AdderContract,
}

impl State {
    async fn new() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(Wallet::from_pem_file(PEM).unwrap());
        let sc_addr_expr = if SC_ADDRESS == "" {
            DEFAULT_ADDRESS_EXPR.to_string()
        } else {
            "bec32:".to_string() + SC_ADDRESS
        };
        let contract = AdderContract::new(sc_addr_expr);

        State {
            interactor,
            wallet_address,
            contract,
        }
    }

    async fn deploy(&mut self) {
        let deploy_result: elrond_interact_snippets::InteractorResult<()> = self
            .interactor
            .sc_deploy(
                self.contract
                    .init(0usize, MultiValueVec::from([self.wallet_address.clone()]))
                    .into_blockchain_call()
                    .from(&self.wallet_address)
                    .code_metadata(CodeMetadata::all())
                    .contract_code(
                        // contract code path can most likely be deduced
                        "file:../output/multisig.wasm",
                        &InterpreterContext::default(),
                    )
                    // gas limit should have a default value, like 100M, and alternatively, read from config
                    .gas_limit("70,000,000")
                    .expect(TxExpect::ok()),
            )
            .await;

        // decode each result and pretty-print.
        let new_address = deploy_result.new_deployed_address();
        let new_address_bech32 = bech32::encode(&new_address);
        println!("new address: {}", new_address_bech32);
        let new_address_expr = format!("bech32:{}", new_address_bech32);
        save_address_expr(new_address_expr.as_str());
    }

    fn add(&mut self) {
        // extract arg
        let add_val: num_bigint::BigUint = 0u32.into();

        let results = self
            .contract
            .add(add_val)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(gas_limit)
            .into()
            .expect(TxExpect::ok())
            .await;

        // print results
    }

    async fn getSum(&mut self, mut args: Args) {
        let results = self.interactor.vm_query(self.adder.getSum()).await;

        // process results, and print each separately
    }
}
