#![allow(non_snake_case)]

mod config;
mod proxy;

pub use config::Config;
use forwarder::vault_proxy;
use multiversx_sc_snippets::imports::*;
pub use proxy::Color;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const STATE_FILE: &str = "state.toml";
pub const FORWARDER_DEPLOY_INTERACTOR_TRACE_PATH: &str =
    "scenarios/forwarder_deploy_scenario.scen.json";
pub const FORWARDER_BUILTIN_INTERACTOR_TRACE_PATH: &str =
    "scenarios/forwarder_builtin_scenario.scen.json";
pub const FORWARDER_CHANGE_TO_DYNAMIC_INTERACTOR_TRACE_PATH: &str =
    "scenarios/forwarder_change_to_dynamic_scenario.scen.json";
pub const FORWARDER_UPDATE_TOKEN_INTERACTOR_TRACE_PATH: &str =
    "scenarios/forwarder_update_token_scenario.scen.json";
pub const FORWARDER_MODIFY_CREATOR_INTERACTOR_TRACE_PATH: &str =
    "scenarios/forwarder_modify_creator_scenario.scen.json";
const VAULT_CODE: MxscPath =
    MxscPath::new("../contracts/feature-tests/composability/vault/output/vault.mxsc.json");

pub async fn forwarder_cli() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut interact = ContractInteract::new(Config::new(), None).await;
    match cmd.as_str() {
        // "deploy" => interact.deploy().await,
        "send_egld" => interact.send_egld().await,
        "echo_arguments_sync" => interact.echo_arguments_sync().await,
        "echo_arguments_sync_twice" => interact.echo_arguments_sync_twice().await,
        "forward_sync_accept_funds" => interact.forward_sync_accept_funds().await,
        "forward_sync_accept_funds_rh_egld" => interact.forward_sync_accept_funds_rh_egld().await,
        "forward_sync_accept_funds_rh_single_esdt" => {
            interact.forward_sync_accept_funds_rh_single_esdt().await
        },
        "forward_sync_accept_funds_rh_multi_esdt" => {
            interact.forward_sync_accept_funds_rh_multi_esdt().await
        },
        "forward_sync_accept_funds_with_fees" => {
            interact.forward_sync_accept_funds_with_fees().await
        },
        "forward_sync_accept_funds_then_read" => {
            interact.forward_sync_accept_funds_then_read().await
        },
        "forward_sync_retrieve_funds" => interact.forward_sync_retrieve_funds().await,
        "forward_sync_retrieve_funds_with_accept_func" => {
            interact
                .forward_sync_retrieve_funds_with_accept_func()
                .await
        },
        "accept_funds_func" => interact.accept_funds_func().await,
        "forward_sync_accept_funds_multi_transfer" => {
            interact.forward_sync_accept_funds_multi_transfer().await
        },
        "echo_args_async" => interact.echo_args_async().await,
        "forward_async_accept_funds" => interact.forward_async_accept_funds().await,
        "forward_async_accept_funds_half_payment" => {
            interact.forward_async_accept_funds_half_payment().await
        },
        "forward_async_accept_funds_with_fees" => {
            interact.forward_async_accept_funds_with_fees().await
        },
        "forward_async_retrieve_funds" => interact.forward_async_retrieve_funds().await,
        "send_funds_twice" => interact.send_funds_twice().await,
        "send_async_accept_multi_transfer" => interact.send_async_accept_multi_transfer().await,
        "callback_data" => interact.callback_data().await,
        "callback_data_at_index" => interact.callback_data_at_index().await,
        "clear_callback_data" => interact.clear_callback_data().await,
        "forward_transf_exec_accept_funds" => interact.forward_transf_exec_accept_funds().await,
        "forward_transf_execu_accept_funds_with_fees" => {
            interact.forward_transf_execu_accept_funds_with_fees().await
        },
        "forward_transf_exec_accept_funds_twice" => {
            interact.forward_transf_exec_accept_funds_twice().await
        },
        "forward_transf_exec_accept_funds_return_values" => {
            interact
                .forward_transf_exec_accept_funds_return_values()
                .await
        },
        "transf_exec_multi_accept_funds" => interact.transf_exec_multi_accept_funds().await,
        // "forward_transf_exec_reject_funds_multi_transfer" => {
        //     interact
        //         .forward_transf_exec_reject_funds_multi_transfer()
        //         .await
        // },
        // "transf_exec_multi_reject_funds" => interact.transf_exec_multi_reject_funds().await,
        "changeOwnerAddress" => interact.change_owner().await,
        "deploy_contract" => interact.deploy_contract().await,
        "deploy_two_contracts" => interact.deploy_two_contracts().await,
        "deploy_vault_from_source" => interact.deploy_vault_from_source().await,
        "upgradeVault" => interact.upgrade_vault().await,
        "upgrade_vault_from_source" => interact.upgrade_vault_from_source().await,
        "getFungibleEsdtBalance" => interact.get_fungible_esdt_balance().await,
        "getCurrentNftNonce" => interact.get_current_nft_nonce().await,
        "send_esdt" => interact.send_esdt().await,
        "send_esdt_with_fees" => interact.send_esdt_with_fees().await,
        "send_esdt_twice" => interact.send_esdt_twice().await,
        "send_esdt_direct_multi_transfer" => interact.send_esdt_direct_multi_transfer().await,
        "issue_fungible_token" => interact.issue_fungible_token().await,
        "local_mint" => interact.local_mint().await,
        "local_burn" => interact.local_burn().await,
        "get_esdt_local_roles" => interact.get_esdt_local_roles().await,
        "get_esdt_token_data" => interact.get_esdt_token_data().await,
        "is_esdt_frozen" => interact.is_esdt_frozen().await,
        "is_esdt_paused" => interact.is_esdt_paused().await,
        "is_esdt_limited_transfer" => interact.is_esdt_limited_transfer().await,
        "validate_token_identifier" => interact.validate_token_identifier().await,
        "sft_issue" => interact.sft_issue().await,
        "get_nft_balance" => interact.get_nft_balance().await,
        "buy_nft" => interact.buy_nft().await,
        "nft_issue" => interact.nft_issue().await,
        "nft_create_compact" => interact.nft_create_compact().await,
        "nft_add_uris" => interact.nft_add_uris().await,
        "nft_update_attributes" => interact.nft_update_attributes().await,
        "nft_decode_complex_attributes" => interact.nft_decode_complex_attributes().await,
        "nft_add_quantity" => interact.nft_add_quantity().await,
        "nft_burn" => interact.nft_burn().await,
        "transfer_nft_via_async_call" => interact.transfer_nft_via_async_call().await,
        "transfer_nft_and_execute" => interact.transfer_nft_and_execute().await,
        "create_and_send" => interact.create_and_send().await,
        "setLocalRoles" => interact.set_local_roles().await,
        "unsetLocalRoles" => interact.unset_local_roles().await,
        "lastErrorMessage" => interact.last_error_message().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    contract_address: Option<Bech32Address>,
}

impl State {
    // Deserializes state from file
    pub fn load_state() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    /// Sets the contract address
    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = Some(address);
    }

    /// Returns the contract address
    pub fn current_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known contract, deploy first")
    }
}

impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}

pub struct ContractInteract {
    pub interactor: Interactor,
    pub wallet_address: Address,
    contract_code: BytesValue,
    pub state: State,
}

impl ContractInteract {
    pub async fn new(config: Config, trace_path: Option<&str>) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        if let Some(path) = trace_path {
            interactor = interactor.with_tracer(path).await;
        }

        interactor.set_current_dir_from_workspace("forwarder-interactor");
        let wallet_address = interactor.register_wallet(test_wallets::alice()).await;

        // Useful in the chain simulator setting
        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();

        let contract_code = BytesValue::interpret_from(
            "mxsc:../forwarder/output/forwarder.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) -> (Bech32Address, u64) {
        let (new_address, gas_used) = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(300_000_000u64)
            .typed(proxy::ForwarderProxy)
            .init()
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .returns(ReturnsGasUsed)
            .run()
            .await;

        let new_address_bech32 = bech32::encode(&new_address);
        self.state.set_address(Bech32Address::from_bech32_string(
            new_address_bech32.clone(),
        ));

        println!("new address: {new_address_bech32}");

        (new_address.into(), gas_used)
    }

    pub async fn send_egld(&mut self) {
        let to = Address::zero();
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .send_egld(to, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_arguments_sync(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();
        let args = MultiValueVec::from(vec![ManagedBuffer::new_from_bytes(&b""[..])]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .echo_arguments_sync(to, args)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_arguments_sync_twice(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();
        let args = MultiValueVec::from(vec![ManagedBuffer::new_from_bytes(&b""[..])]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .echo_arguments_sync_twice(to, args)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_sync_accept_funds(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_sync_accept_funds(to)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_sync_accept_funds_rh_egld(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_sync_accept_funds_rh_egld(to)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_sync_accept_funds_rh_single_esdt(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_sync_accept_funds_rh_single_esdt(to)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_sync_accept_funds_rh_multi_esdt(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_sync_accept_funds_rh_multi_esdt(to)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_sync_accept_funds_with_fees(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();
        let percentage_fees = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_sync_accept_funds_with_fees(to, percentage_fees)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_sync_accept_funds_then_read(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_sync_accept_funds_then_read(to)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_sync_retrieve_funds(&mut self) {
        let to = Address::zero();
        let token = EgldOrEsdtTokenIdentifier::esdt(&b""[..]);
        let token_nonce = 0u64;
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_sync_retrieve_funds(to, token, token_nonce, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_sync_retrieve_funds_with_accept_func(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();
        let token = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_sync_retrieve_funds_with_accept_func(to, token, amount)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn accept_funds_func(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .accept_funds_func()
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_sync_accept_funds_multi_transfer(&mut self) {
        let to = Address::zero();
        let token_payments = MultiValueVec::from(vec![MultiValue3::<
            TokenIdentifier<StaticApi>,
            u64,
            BigUint<StaticApi>,
        >::from((
            TokenIdentifier::from_esdt_bytes(&b""[..]),
            0u64,
            BigUint::<StaticApi>::from(0u128),
        ))]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_sync_accept_funds_multi_transfer(to, token_payments)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_args_async(&mut self) {
        let to = Address::zero();
        let args = MultiValueVec::from(vec![ManagedBuffer::new_from_bytes(&b""[..])]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .echo_args_async(to, args)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_async_accept_funds(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_async_accept_funds(to)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_async_accept_funds_half_payment(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_async_accept_funds_half_payment(to)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_async_accept_funds_with_fees(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();
        let percentage_fees = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_async_accept_funds_with_fees(to, percentage_fees)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_async_retrieve_funds(&mut self) {
        let to = Address::zero();
        let token = EgldOrEsdtTokenIdentifier::esdt(&b""[..]);
        let token_nonce = 0u64;
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_async_retrieve_funds(to, token, token_nonce, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn send_funds_twice(&mut self) {
        let to = Address::zero();
        let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b""[..]);
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .send_funds_twice(to, token_identifier, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn send_async_accept_multi_transfer(&mut self) {
        let to = Address::zero();
        let token_payments = MultiValueVec::from(vec![MultiValue3::<
            TokenIdentifier<StaticApi>,
            u64,
            BigUint<StaticApi>,
        >::from((
            TokenIdentifier::from_esdt_bytes(&b""[..]),
            0u64,
            BigUint::<StaticApi>::from(0u128),
        ))]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .send_async_accept_multi_transfer(to, token_payments)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn callback_data(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::ForwarderProxy)
            .callback_data()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {:?}", result_value.0);
    }

    pub async fn callback_data_at_index(&mut self) {
        let index = 0u32;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::ForwarderProxy)
            .callback_data_at_index(index)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn clear_callback_data(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .clear_callback_data()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_transf_exec_accept_funds(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_transf_exec_accept_funds(to)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_transf_execu_accept_funds_with_fees(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();
        let percentage_fees = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_transf_execu_accept_funds_with_fees(to, percentage_fees)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_transf_exec_accept_funds_twice(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_transf_exec_accept_funds_twice(to)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn forward_transf_exec_accept_funds_return_values(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .forward_transf_exec_accept_funds_return_values(to)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn transf_exec_multi_accept_funds(&mut self) {
        let to = Address::zero();
        let token_payments = MultiValueVec::from(vec![MultiValue3::<
            TokenIdentifier<StaticApi>,
            u64,
            BigUint<StaticApi>,
        >::from((
            TokenIdentifier::from_esdt_bytes(&b""[..]),
            0u64,
            BigUint::<StaticApi>::from(0u128),
        ))]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .transf_exec_multi_accept_funds(to, token_payments)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    // pub async fn forward_transf_exec_reject_funds_multi_transfer(&mut self) {
    //     let to = Address::zero();
    //     let token_payments = MultiValueVec::from(vec![MultiValue3::<
    //         TokenIdentifier<StaticApi>,
    //         u64,
    //         BigUint<StaticApi>,
    //     >::from((
    //         TokenIdentifier::from_esdt_bytes(&b""[..]),
    //         0u64,
    //         BigUint::<StaticApi>::from(0u128),
    //     ))]);

    //     let response = self
    //         .interactor
    //         .tx()
    //         .from(&self.wallet_address)
    //         .to(self.state.current_address())
    //         .gas(80_000_000u64)
    //         .typed(proxy::ForwarderProxy)
    //         .forward_transf_exec_reject_funds_multi_transfer(to, token_payments)
    //         .returns(ReturnsResultUnmanaged)
    //         .run()
    //         .await;

    //     println!("Result: {response:?}");
    // }

    pub async fn transf_exec_multi_reject_funds(
        &mut self,
        to: Bech32Address,
        vec_of_payments: Vec<EgldOrEsdtTokenPayment<StaticApi>>,
    ) -> u64 {
        let vec = vec_of_payments
            .iter()
            .map(|e| {
                MultiValue3::from((e.token_identifier.clone(), e.token_nonce, e.amount.clone()))
            })
            .collect::<MultiValueEncoded<
                StaticApi,
                MultiValue3<EgldOrEsdtTokenIdentifier<StaticApi>, u64, BigUint<StaticApi>>,
            >>();

        let vec_param =
            ManagedVec::<StaticApi, EgldOrEsdtTokenPayment<StaticApi>>::from(vec_of_payments);

        let (response, gas_used) = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .transf_exec_multi_reject_funds(to, vec)
            .payment(vec_param)
            .returns(ExpectMessage("reject_funds"))
            .returns(ReturnsGasUsed)
            .run()
            .await;

        println!("Result: {response:?}");

        gas_used
    }

    pub async fn change_owner(&mut self) {
        let child_sc_address = Address::zero();
        let new_owner = Address::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .change_owner(child_sc_address, new_owner)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn deploy_contract(&mut self) {
        let code = ManagedBuffer::new_from_bytes(&b""[..]);
        let opt_arg = OptionalValue::Some(ManagedBuffer::new_from_bytes(&b""[..]));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .deploy_contract(code, opt_arg)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn deploy_two_contracts(&mut self) {
        let code = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .deploy_two_contracts(code)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn deploy_vault_from_source(&mut self) {
        let source_address = Address::zero();
        let opt_arg = OptionalValue::Some(ManagedBuffer::new_from_bytes(&b""[..]));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .deploy_vault_from_source(source_address, opt_arg)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn upgrade_vault(&mut self) {
        let child_sc_address = Address::zero();
        let new_code = ManagedBuffer::new_from_bytes(&b""[..]);
        let opt_arg = OptionalValue::Some(ManagedBuffer::new_from_bytes(&b""[..]));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .upgrade_vault(child_sc_address, new_code, opt_arg)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn upgrade_vault_from_source(&mut self) {
        let child_sc_address = Address::zero();
        let source_address = Address::zero();
        let opt_arg = OptionalValue::Some(ManagedBuffer::new_from_bytes(&b""[..]));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .upgrade_vault_from_source(child_sc_address, source_address, opt_arg)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_fungible_esdt_balance(&mut self) {
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::ForwarderProxy)
            .get_fungible_esdt_balance(token_identifier)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_current_nft_nonce(&mut self) {
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::ForwarderProxy)
            .get_current_nft_nonce(token_identifier)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn send_esdt(&mut self) {
        let to = Address::zero();
        let token_id = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .send_esdt(to, token_id, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn send_esdt_with_fees(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let to = Address::zero();
        let percentage_fees = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .send_esdt_with_fees(to, percentage_fees)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn send_esdt_twice(&mut self) {
        let to = Address::zero();
        let token_id = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let amount_first_time = BigUint::<StaticApi>::from(0u128);
        let amount_second_time = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .send_esdt_twice(to, token_id, amount_first_time, amount_second_time)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn send_esdt_direct_multi_transfer(&mut self) {
        let to = Address::zero();
        let token_payments = MultiValueVec::from(vec![MultiValue3::<
            TokenIdentifier<StaticApi>,
            u64,
            BigUint<StaticApi>,
        >::from((
            TokenIdentifier::from_esdt_bytes(&b""[..]),
            0u64,
            BigUint::<StaticApi>::from(0u128),
        ))]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .send_esdt_direct_multi_transfer(to, token_payments)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn issue_fungible_token(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let token_display_name = ManagedBuffer::new_from_bytes(&b""[..]);
        let token_ticker = ManagedBuffer::new_from_bytes(&b""[..]);
        let initial_supply = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .issue_fungible_token(token_display_name, token_ticker, initial_supply)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn local_mint(&mut self) {
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .local_mint(token_identifier, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn local_burn(&mut self) {
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .local_burn(token_identifier, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_esdt_local_roles(&mut self) {
        let token_id = TokenIdentifier::from_esdt_bytes(&b""[..]);

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::ForwarderProxy)
            .get_esdt_local_roles(token_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_esdt_token_data(&mut self) {
        let address = Address::zero();
        let token_id = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let nonce = 0u64;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::ForwarderProxy)
            .get_esdt_token_data(address, token_id, nonce)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn is_esdt_frozen(&mut self) {
        let address = Address::zero();
        let token_id = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let nonce = 0u64;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::ForwarderProxy)
            .is_esdt_frozen(address, token_id, nonce)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn is_esdt_paused(&mut self) {
        let token_id = TokenIdentifier::from_esdt_bytes(&b""[..]);

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::ForwarderProxy)
            .is_esdt_paused(token_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn is_esdt_limited_transfer(&mut self) {
        let token_id = TokenIdentifier::from_esdt_bytes(&b""[..]);

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::ForwarderProxy)
            .is_esdt_limited_transfer(token_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn validate_token_identifier(&mut self) {
        let token_id = TokenIdentifier::from_esdt_bytes(&b""[..]);

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::ForwarderProxy)
            .validate_token_identifier(token_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn sft_issue(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let token_display_name = ManagedBuffer::new_from_bytes(&b""[..]);
        let token_ticker = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .sft_issue(token_display_name, token_ticker)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_nft_balance(&mut self) {
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let nonce = 0u64;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::ForwarderProxy)
            .get_nft_balance(token_identifier, nonce)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn buy_nft(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let nft_id = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let nft_nonce = 0u64;
        let nft_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .buy_nft(nft_id, nft_nonce, nft_amount)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn nft_issue(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let token_display_name = ManagedBuffer::new_from_bytes(&b""[..]);
        let token_ticker = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .nft_issue(token_display_name, token_ticker)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn nft_create(
        &mut self,
        token_id: &[u8],
        amount: RustBigUint,
        name: &[u8],
        royalties: u64,
        hash: &[u8],
        attributes: &Color,
        uri: &[u8],
    ) {
        println!("Minting NFT...");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .nft_create(
                token_id,
                amount,
                name,
                royalties,
                hash,
                attributes,
                &ManagedBuffer::from(uri),
            )
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn nft_create_compact(&mut self) {
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let amount = BigUint::<StaticApi>::from(0u128);
        let color = Color::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .nft_create_compact(token_identifier, amount, color)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn nft_add_uris(&mut self) {
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let nonce = 0u64;
        let uris = MultiValueVec::from(vec![ManagedBuffer::new_from_bytes(&b""[..])]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .nft_add_uris(token_identifier, nonce, uris)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn nft_update_attributes(&mut self) {
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let nonce = 0u64;
        let new_attributes = Color::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .nft_update_attributes(token_identifier, nonce, new_attributes)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn nft_decode_complex_attributes(&mut self) {
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let amount = BigUint::<StaticApi>::from(0u128);
        let name = ManagedBuffer::new_from_bytes(&b""[..]);
        let royalties = BigUint::<StaticApi>::from(0u128);
        let hash = ManagedBuffer::new_from_bytes(&b""[..]);
        let uri = ManagedBuffer::new_from_bytes(&b""[..]);
        let attrs_arg = MultiValue5::<
            BigUint<StaticApi>,
            ManagedBuffer<StaticApi>,
            TokenIdentifier<StaticApi>,
            bool,
            ManagedBuffer<StaticApi>,
        >::from((
            BigUint::<StaticApi>::from(0u128),
            ManagedBuffer::new_from_bytes(&b""[..]),
            TokenIdentifier::from_esdt_bytes(&b""[..]),
            false,
            ManagedBuffer::new_from_bytes(&b""[..]),
        ));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .nft_decode_complex_attributes(
                token_identifier,
                amount,
                name,
                royalties,
                hash,
                uri,
                attrs_arg,
            )
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn nft_add_quantity(&mut self) {
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let nonce = 0u64;
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .nft_add_quantity(token_identifier, nonce, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn nft_burn(&mut self) {
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let nonce = 0u64;
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .nft_burn(token_identifier, nonce, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn transfer_nft_via_async_call(&mut self) {
        let to = Address::zero();
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let nonce = 0u64;
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .transfer_nft_via_async_call(to, token_identifier, nonce, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn transfer_nft_and_execute(&mut self) {
        let to = Address::zero();
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let nonce = 0u64;
        let amount = BigUint::<StaticApi>::from(0u128);
        let function = ManagedBuffer::new_from_bytes(&b""[..]);
        let arguments = MultiValueVec::from(vec![ManagedBuffer::new_from_bytes(&b""[..])]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .transfer_nft_and_execute(to, token_identifier, nonce, amount, function, arguments)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn create_and_send(&mut self) {
        let to = Address::zero();
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let amount = BigUint::<StaticApi>::from(0u128);
        let name = ManagedBuffer::new_from_bytes(&b""[..]);
        let royalties = BigUint::<StaticApi>::from(0u128);
        let hash = ManagedBuffer::new_from_bytes(&b""[..]);
        let color = Color::default();
        let uri = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .create_and_send(
                to,
                token_identifier,
                amount,
                name,
                royalties,
                hash,
                color,
                uri,
            )
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_local_roles(&mut self) {
        let address = Address::zero();
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let roles = MultiValueVec::<EsdtLocalRole>::new();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .set_local_roles(address, token_identifier, roles)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn unset_local_roles(&mut self) {
        let address = Address::zero();
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let roles = MultiValueVec::<EsdtLocalRole>::new();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .unset_local_roles(address, token_identifier, roles)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn issue_dynamic_token(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &[u8],
        token_ticker: &[u8],
        token_type: EsdtTokenType,
        num_decimals: usize,
    ) {
        println!("Registering dynamic token {token_ticker:?} of type {token_type:?}...");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .issue_dynamic_token(token_display_name, token_ticker, token_type, num_decimals)
            .egld(BigUint::from(issue_cost))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn issue_token_all_roles(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &[u8],
        token_ticker: &[u8],
        num_decimals: usize,
        token_type: EsdtTokenType,
    ) {
        println!("Registering and setting all roles for token {token_ticker:?} of type {token_type:?}...");

        let result = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .issue_token_all_roles(token_display_name, token_ticker, token_type, num_decimals)
            .egld(BigUint::from(issue_cost))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result:?}");
    }

    pub async fn change_to_dynamic(&mut self, token_id: &[u8]) {
        println!("Changing the following token {token_id:?} to dynamic...");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .change_to_dynamic(TokenIdentifier::from(token_id))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn update_token(&mut self, token_id: &[u8]) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .update_token(TokenIdentifier::from(token_id))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn modify_royalties(&mut self, token_id: &[u8], nonce: u64, new_royalty: u64) {
        println!("Modifying royalties for token {token_id:?} into {new_royalty:?}...");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .modify_royalties(TokenIdentifier::from(token_id), nonce, new_royalty)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_new_uris(&mut self, token_id: &[u8], nonce: u64, new_uris: Vec<String>) {
        let uris = new_uris
            .into_iter()
            .map(ManagedBuffer::from)
            .collect::<MultiValueEncoded<StaticApi, ManagedBuffer<StaticApi>>>();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .set_new_uris(token_id, nonce, uris)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn modify_creator(&mut self, token_id: &[u8], nonce: u64) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .modify_creator(TokenIdentifier::from(token_id), nonce)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn metadata_recreate(
        &mut self,
        token_id: &[u8],
        nonce: u64,
        name: &[u8],
        royalties: u64,
        hash: &[u8],
        new_attributes: &Color,
        uris: Vec<String>,
    ) {
        println!("Recreating the token {token_id:?} with nonce {nonce:?} with new attributes...");

        let uris = uris
            .into_iter()
            .map(ManagedBuffer::from)
            .collect::<MultiValueEncoded<StaticApi, ManagedBuffer<StaticApi>>>();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .metadata_recreate(token_id, nonce, name, royalties, hash, new_attributes, uris)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn metadata_update(
        &mut self,
        token_id: &[u8],
        nonce: u64,
        name: &[u8],
        royalties: u64,
        hash: &[u8],
        new_attributes: &Color,
        uris: Vec<String>,
    ) {
        println!("Updating the token {token_id:?} with nonce {nonce:?} with new attributes...");

        let uris = uris
            .into_iter()
            .map(ManagedBuffer::from)
            .collect::<MultiValueEncoded<StaticApi, ManagedBuffer<StaticApi>>>();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .metadata_update(token_id, nonce, name, royalties, hash, new_attributes, uris)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn last_issued_token(&mut self) -> String {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::ForwarderProxy)
            .last_issued_token()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");

        result_value.as_managed_buffer().to_string()
    }

    pub async fn last_error_message(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::ForwarderProxy)
            .last_error_message()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn issue_dynamic_token_from_wallet(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &[u8],
        token_ticker: &[u8],
        token_type: EsdtTokenType,
        num_decimals: usize,
    ) -> String {
        println!("Registering dynamic token {token_ticker:?} of type {token_type:?} from the test wallet...");

        let token_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .issue_dynamic(
                issue_cost.into(),
                token_display_name,
                token_ticker,
                token_type,
                num_decimals,
            )
            .returns(ReturnsNewTokenIdentifier)
            .run()
            .await;

        println!("TOKEN ID: {:?}", token_id);

        token_id
    }

    pub async fn set_roles_from_wallet(
        &mut self,
        for_address: &Address,
        token_id: &[u8],
        roles: Vec<EsdtLocalRole>,
    ) {
        println!("Setting the following roles: {roles:?} for {token_id:?}");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(80_000_000u64)
            .typed(ESDTSystemSCProxy)
            .set_special_roles(
                ManagedAddress::from_address(for_address),
                TokenIdentifier::from(token_id),
                roles.into_iter(),
            )
            .run()
            .await;
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn mint_nft_from_wallet<T: TopEncode>(
        &mut self,
        token_id: &[u8],
        amount: RustBigUint,
        name: &[u8],
        royalties: u64,
        hash: &[u8],
        attributes: &T,
        uris: Vec<String>,
    ) -> u64 {
        println!("Minting NFT...");

        let uris = uris
            .into_iter()
            .map(ManagedBuffer::from)
            .collect::<ManagedVec<StaticApi, ManagedBuffer<StaticApi>>>();

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_nft_create(token_id, amount, name, royalties, hash, attributes, &uris)
            .returns(ReturnsResult)
            .run()
            .await
    }

    pub async fn send_esdt_from_wallet(
        &mut self,
        to: &Address,
        token_id: &[u8],
        nonce: u64,
        amount: RustBigUint,
    ) {
        println!("Sending token {token_id:?} with nonce {nonce:?} to other_wallet_address...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(to)
            .single_esdt(&token_id.into(), nonce, &amount.into()) // .transfer()
            .run()
            .await;
    }

    pub async fn deploy_vault(&mut self) -> (Bech32Address, u64) {
        let (new_address, gas_used) = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(300_000_000u64)
            .typed(vault_proxy::VaultProxy)
            .init(OptionalValue::<ManagedBuffer<StaticApi>>::None)
            .code(VAULT_CODE)
            .returns(ReturnsNewAddress)
            .returns(ReturnsGasUsed)
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);

        println!("new vault address: {new_address_bech32}");

        (new_address.into(), gas_used)
    }

    pub async fn forward_send_async_reject_multi_transfer(
        &mut self,
        to: Bech32Address,
        vec_of_payments: Vec<EgldOrEsdtTokenPayment<StaticApi>>,
    ) {
        let vec = vec_of_payments
            .iter()
            .map(|e| {
                MultiValue3::from((e.token_identifier.clone(), e.token_nonce, e.amount.clone()))
            })
            .collect::<MultiValueEncoded<
                StaticApi,
                MultiValue3<EgldOrEsdtTokenIdentifier<StaticApi>, u64, BigUint<StaticApi>>,
            >>();

        let vec_param =
            ManagedVec::<StaticApi, EgldOrEsdtTokenPayment<StaticApi>>::from(vec_of_payments);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(80_000_000u64)
            .typed(proxy::ForwarderProxy)
            .send_async_reject_multi_transfer(to.into_address(), vec)
            .payment(vec_param)
            .returns(ReturnsHandledOrError::new().returns(ExpectMessage("reject_funds")))
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn issue_fungible_token_from_wallet(
        &mut self,
        token_display_name: &[u8],
        token_ticker: &[u8],
        initial_supply: u64,
    ) -> (String, u64) {
        let egld_amount = BigUint::<StaticApi>::from(5_000_000_000_000_000_0u128);

        let (response, gas_used) = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(80_000_000u64)
            .typed(system_proxy::ESDTSystemSCProxy)
            .issue_fungible(
                egld_amount,
                token_display_name,
                token_ticker,
                BigUint::from(initial_supply),
                FungibleTokenProperties::default(),
            )
            .returns(ReturnsNewTokenIdentifier)
            .returns(ReturnsGasUsed)
            .run()
            .await;

        println!("Result: {response:?}");

        (response, gas_used)
    }
}
