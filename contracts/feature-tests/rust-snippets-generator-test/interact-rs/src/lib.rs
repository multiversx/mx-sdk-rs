#![allow(non_snake_case)]

use multiversx_chain_vm::{
    bech32, scenario_format::interpret_trait::InterpreterContext, ContractInfo, DebugApi,
};
use multiversx_sc_scenario::scenario_model::*;
use multiversx_sc_snippets::{
    env_logger,
    erdrs::wallet::Wallet,
    multiversx_sc::{codec::multi_types::*, types::*},
    tokio, Interactor,
};
use rust_snippets_generator_test::{ProxyTrait as _, *};

const GATEWAY: &str = multiversx_sdk::blockchain::DEVNET_GATEWAY;
const PEM: &str = "alice.pem";
const SC_ADDRESS: &str = "";

const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";
const DEFAULT_ADDRESS_EXPR: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";
const DEFAULT_GAS_LIMIT: u64 = 100_000_000;
const TOKEN_ISSUE_COST: u64 = 50_000_000_000_000_000;

type ContractType = ContractInfo<rust_snippets_generator_test::Proxy<DebugApi>>;

#[tokio::main]
async fn main() {
    env_logger::init();
    DebugApi::dummy();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut state = State::new().await;
    match cmd.as_str() {
        "deploy" => state.deploy().await,
        "no_arg_no_result_endpoint" => state.no_arg_no_result_endpoint().await,
        "no_arg_one_result_endpoint" => state.no_arg_one_result_endpoint().await,
        "one_arg_no_result_endpoint" => state.one_arg_no_result_endpoint().await,
        "one_arg_one_result_endpoint" => state.one_arg_one_result_endpoint().await,
        "multi_result" => state.multi_result().await,
        "nested_result" => state.nested_result().await,
        "custom_struct" => state.custom_struct().await,
        "optional_type" => state.optional_type().await,
        "option_type" => state.option_type().await,
        "esdt_token_payment" => state.esdt_token_payment().await,
        "egld_or_esdt_payment" => state.egld_or_esdt_payment().await,
        "payable_endpoint" => state.payable_endpoint().await,
        "managed_buffer" => state.managed_buffer().await,
        "multi_value_2" => state.multi_value_2().await,
        "multi_value_4" => state.multi_value_4().await,
        "complex_multi_values" => state.complex_multi_values().await,
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

    async fn deploy(&mut self) {
        let mut typed_sc_deploy = self
            .contract
            .init()
            .into_blockchain_call()
            .from(&self.wallet_address)
            .code_metadata(CodeMetadata::all())
            .contract_code(
                "file:../output/rust-snippets-generator-test.wasm",
                &InterpreterContext::default(),
            )
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_deploy(&mut typed_sc_deploy).await;

        if let Some(new_address) = typed_sc_deploy.response().new_deployed_address.clone() {
            let new_address_bech32 = bech32::encode(&new_address);
            println!("new address: {}", new_address_bech32);

            let result = typed_sc_deploy.result();
            if result.is_err() {
                println!("Result error: {}", result.err().unwrap());
                return;
            }

            println!("Result: {:?}", result.unwrap());
        } else {
            println!("deploy failed");
            return;
        }
    }

    async fn no_arg_no_result_endpoint(&mut self) {
        let mut typed_sc_call = self
            .contract
            .no_arg_no_result_endpoint()
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!("Result error: {}", result.err().unwrap());
            return;
        }

        println!("Result: {:?}", result.unwrap());
    }

    async fn no_arg_one_result_endpoint(&mut self) {
        let mut typed_sc_call = self
            .contract
            .no_arg_one_result_endpoint()
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!("Result error: {}", result.err().unwrap());
            return;
        }

        println!("Result: {:?}", result.unwrap());
    }

    async fn one_arg_no_result_endpoint(&mut self) {
        let _arg = 0u64;

        let mut typed_sc_call = self
            .contract
            .one_arg_no_result_endpoint(_arg)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!("Result error: {}", result.err().unwrap());
            return;
        }

        println!("Result: {:?}", result.unwrap());
    }

    async fn one_arg_one_result_endpoint(&mut self) {
        let _arg = 0u64;

        let mut typed_sc_call = self
            .contract
            .one_arg_one_result_endpoint(_arg)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!("Result error: {}", result.err().unwrap());
            return;
        }

        println!("Result: {:?}", result.unwrap());
    }

    async fn multi_result(&mut self) {
        let _arg = TokenIdentifier::from_esdt_bytes(&b""[..]);

        let mut typed_sc_call = self
            .contract
            .multi_result(_arg)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!("Result error: {}", result.err().unwrap());
            return;
        }

        println!("Result: {:?}", result.unwrap());
    }

    async fn nested_result(&mut self) {
        let _arg = TokenIdentifier::from_esdt_bytes(&b""[..]);

        let mut typed_sc_call = self
            .contract
            .nested_result(_arg)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!("Result error: {}", result.err().unwrap());
            return;
        }

        println!("Result: {:?}", result.unwrap());
    }

    async fn custom_struct(&mut self) {
        let _arg = PlaceholderInput;

        let mut typed_sc_call = self
            .contract
            .custom_struct(_arg)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!("Result error: {}", result.err().unwrap());
            return;
        }

        println!("Result: {:?}", result.unwrap());
    }

    async fn optional_type(&mut self) {
        let _arg = OptionalValue::Some(BigUint::<DebugApi>::from(0u64));

        let mut typed_sc_call = self
            .contract
            .optional_type(_arg)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!("Result error: {}", result.err().unwrap());
            return;
        }

        println!("Result: {:?}", result.unwrap());
    }

    async fn option_type(&mut self) {
        let _arg = Option::Some(ManagedVec::from_single_item(
            TokenIdentifier::from_esdt_bytes(&b""[..]),
        ));

        let mut typed_sc_call = self
            .contract
            .option_type(_arg)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!("Result error: {}", result.err().unwrap());
            return;
        }

        println!("Result: {:?}", result.unwrap());
    }

    async fn esdt_token_payment(&mut self) {
        let _arg = OptionalValue::Some(EsdtTokenPayment::new(
            TokenIdentifier::from_esdt_bytes(&b""[..]),
            0u64,
            BigUint::from(0u64),
        ));

        let mut typed_sc_call = self
            .contract
            .esdt_token_payment(_arg)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!("Result error: {}", result.err().unwrap());
            return;
        }

        println!("Result: {:?}", result.unwrap());
    }

    async fn egld_or_esdt_payment(&mut self) {
        let arg = EgldOrEsdtTokenPayment::new(
            EgldOrEsdtTokenIdentifier::esdt(&b""[..]),
            0u64,
            BigUint::from(0u64),
        );

        let mut typed_sc_call = self
            .contract
            .egld_or_esdt_payment(arg)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!("Result error: {}", result.err().unwrap());
            return;
        }

        println!("Result: {:?}", result.unwrap());
    }

    async fn payable_endpoint(&mut self) {
        let token_id = b"";
        let token_nonce = 0u64;
        let token_amount = BigUint::<DebugApi>::from(0u64);

        let mut typed_sc_call = self
            .contract
            .payable_endpoint()
            .into_blockchain_call()
            .from(&self.wallet_address)
            .esdt_transfer(token_id.to_vec(), token_nonce, token_amount)
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!("Result error: {}", result.err().unwrap());
            return;
        }

        println!("Result: {:?}", result.unwrap());
    }

    async fn managed_buffer(&mut self) {
        let _arg = Option::Some(ManagedBuffer::new_from_bytes(&b""[..]));

        let mut typed_sc_call = self
            .contract
            .managed_buffer(_arg)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!("Result error: {}", result.err().unwrap());
            return;
        }

        println!("Result: {:?}", result.unwrap());
    }

    async fn multi_value_2(&mut self) {
        let arg = MultiValue2::from((0u64, BigUint::<DebugApi>::from(0u64)));

        let mut typed_sc_call = self
            .contract
            .multi_value_2(arg)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!("Result error: {}", result.err().unwrap());
            return;
        }

        println!("Result: {:?}", result.unwrap());
    }

    async fn multi_value_4(&mut self) {
        let arg = PlaceholderInput;

        let mut typed_sc_call = self
            .contract
            .multi_value_4(arg)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!("Result error: {}", result.err().unwrap());
            return;
        }

        println!("Result: {:?}", result.unwrap());
    }

    async fn complex_multi_values(&mut self) {
        let arg = MultiValueVec::from(vec![MultiValue3::from((
            TokenIdentifier::from_esdt_bytes(&b""[..]),
            0u64,
            BigUint::<DebugApi>::from(0u64),
        ))]);

        let mut typed_sc_call = self
            .contract
            .complex_multi_values(arg)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit(DEFAULT_GAS_LIMIT);

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!("Result error: {}", result.err().unwrap());
            return;
        }

        println!("Result: {:?}", result.unwrap());
    }
}
