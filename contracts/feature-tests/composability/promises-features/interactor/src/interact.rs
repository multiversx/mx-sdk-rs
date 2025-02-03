pub mod config;
mod interactor_cli;
mod interactor_state;

use clap::Parser;
use config::Config;
use interactor_state::State;
use multiversx_sc_snippets::imports::*;
use promises_features::promises_feature_proxy;

const CODE_PATH: MxscPath = MxscPath::new("output/promises-features.mxsc.json");

pub async fn promises_features_cli() {
    env_logger::init();

    let config = Config::new();
    let mut interact = ContractInteract::new(config).await;

    let cli = interactor_cli::InteractCli::parse();
    match &cli.command {
        Some(interactor_cli::InteractCliCommand::Deploy) => interact.deploy().await,
        Some(interactor_cli::InteractCliCommand::CallbackData) => interact.callback_data().await,
        Some(interactor_cli::InteractCliCommand::CallbackDataAtIndex(args)) => {
            interact.callback_data_at_index(args.index).await
        },
        Some(interactor_cli::InteractCliCommand::ClearCallbackData) => {
            interact.clear_callback_data().await
        },
        Some(interactor_cli::InteractCliCommand::ForwardPromiseAcceptFunds(args)) => {
            interact
                .forward_promise_accept_funds(
                    &args.token_id,
                    args.token_nonce,
                    args.token_amount,
                    &Bech32Address::from_bech32_string(args.to.clone()),
                )
                .await
        },
        Some(interactor_cli::InteractCliCommand::ForwardPromiseRetrieveFunds(args)) => {
            interact
                .forward_promise_retrieve_funds(
                    &args.token_id,
                    args.token_nonce,
                    args.token_amount,
                    &Bech32Address::from_bech32_string(args.to.clone()),
                )
                .await
        },
        Some(interactor_cli::InteractCliCommand::ForwardPaymentCallback(args)) => {
            interact
                .forward_payment_callback(
                    &args.token_id,
                    args.token_nonce,
                    args.token_amount,
                    &Bech32Address::from_bech32_string(args.to.clone()),
                )
                .await
        },
        Some(interactor_cli::InteractCliCommand::PromiseRawSingleToken(args)) => {
            let promise_args_vec: Vec<&str> = args.args.split(",").collect();

            interact
                .promise_raw_single_token(
                    &args.token_id,
                    args.token_nonce,
                    args.token_amount,
                    &Bech32Address::from_bech32_string(args.to.clone()),
                    &args.endpoint_name,
                    args.gas_limit,
                    args.extra_gas_for_callback,
                    &promise_args_vec,
                )
                .await
        },
        Some(interactor_cli::InteractCliCommand::PromiseRawMultiTransfer(args)) => {
            let payment: EsdtTokenPaymentMultiValue<StaticApi> =
                EsdtTokenPaymentMultiValue::from(EsdtTokenPayment::new(
                    TokenIdentifier::from_esdt_bytes(&b""[..]),
                    0u64,
                    BigUint::<StaticApi>::from(0u128),
                ));

            let mut token_payment_args = MultiValueEncoded::new();
            token_payment_args.push(payment);

            interact
                .promise_raw_multi_transfer(
                    &Bech32Address::from_bech32_string(args.to.clone()),
                    &args.endpoint_name,
                    args.extra_gas_for_callback,
                    token_payment_args,
                )
                .await
        },
        Some(interactor_cli::InteractCliCommand::ForwardSyncRetrieveFundsBt(args)) => {
            interact
                .forward_sync_retrieve_funds_bt(
                    &Bech32Address::from_bech32_string(args.to.clone()),
                    &args.token_id,
                    args.token_nonce,
                    args.token_amount,
                )
                .await
        },
        Some(interactor_cli::InteractCliCommand::ForwardSyncRetrieveFundsBtTwice(args)) => {
            interact
                .forward_sync_retrieve_funds_bt_twice(
                    &Bech32Address::from_bech32_string(args.to.clone()),
                    &args.token_id,
                    args.token_nonce,
                    args.token_amount,
                )
                .await
        },
        Some(interactor_cli::InteractCliCommand::ForwardPromiseRetrieveFundsBackTransfers(
            args,
        )) => {
            interact
                .forward_promise_retrieve_funds_back_transfers(
                    &Bech32Address::from_bech32_string(args.to.clone()),
                    &args.token_id,
                    args.token_nonce,
                    args.token_amount,
                )
                .await
        },
        _ => {},
    }
}

pub struct ContractInteract {
    pub interactor: Interactor,
    pub wallet_address: Address,
    pub state: State,
}

impl ContractInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace(
            "contracts/feature-tests/composability/promises-features",
        );
        let wallet_address = interactor.register_wallet(test_wallets::alice()).await;

        // Useful in the chain simulator setting
        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();

        ContractInteract {
            interactor,
            wallet_address,
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(51_000_000u64)
            .typed(promises_feature_proxy::PromisesFeaturesProxy)
            .init()
            .code(CODE_PATH)
            .returns(ReturnsNewAddress)
            .run()
            .await;

        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_promises_features_address(Bech32Address::from_bech32_string(
                new_address_bech32.clone(),
            ));

        println!("new address: {new_address_bech32}");
    }

    pub async fn callback_data(&mut self) {
        let response = self
            .interactor
            .query()
            .to(self.state.current_promises_features_address())
            .typed(promises_feature_proxy::PromisesFeaturesProxy)
            .callback_data()
            .returns(ReturnsHandledOrError::new().returns(ReturnsResultUnmanaged))
            .run()
            .await;

        match response {
            Ok(result) => {
                println!("Callbacks stored");
                for r in result.into_vec() {
                    let args_str: Vec<String> = r
                        .args
                        .into_vec()
                        .iter()
                        .map(|arg| arg.to_string())
                        .collect();

                    println!(
                        "{} | {} | {} | {} | {}",
                        r.callback_name,
                        r.token_identifier.into_name(),
                        r.token_nonce,
                        r.token_amount
                            .to_u64()
                            .unwrap_or_else(|| panic!("unable to parse token amount")),
                        args_str.join(", ")
                    );
                }
            },
            Err(_) => panic!("Cannot retrieve CallbackData storage!"),
        }
    }

    pub async fn callback_data_at_index(&mut self, index: u32) {
        let response = self
            .interactor
            .query()
            .to(self.state.current_promises_features_address())
            .typed(promises_feature_proxy::PromisesFeaturesProxy)
            .callback_data_at_index(index)
            .returns(ReturnsHandledOrError::new().returns(ReturnsResultUnmanaged))
            .run()
            .await;

        match response {
            Ok(result) => {
                let (callback_name, token_identifier, token_nonce, token_amount, args) =
                    result.into_tuple();
                let args_str: Vec<String> = args
                    .0
                    .iter()
                    .map(|arg| String::from_utf8(arg.to_vec()).unwrap())
                    .collect();
                println!(
                    "{} | {} | {} | {} | {}",
                    String::from_utf8(callback_name).unwrap(),
                    token_identifier.into_name(),
                    token_nonce,
                    token_amount,
                    args_str.join(", ")
                )
            },
            Err(_) => panic!("Cannot retrieve CallbackData at index {index} storage!"),
        }
    }

    pub async fn clear_callback_data(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_promises_features_address())
            .gas(30_000_000u64)
            .typed(promises_feature_proxy::PromisesFeaturesProxy)
            .clear_callback_data()
            .run()
            .await;

        println!("DONE cleared callback data");
    }

    pub async fn forward_promise_accept_funds(
        &mut self,
        token_id: &str,
        token_nonce: u64,
        token_amount: u64,
        to: &Bech32Address,
    ) {
        let payment: EsdtTokenPayment<StaticApi> = EsdtTokenPayment::new(
            TokenIdentifier::from(token_id),
            token_nonce,
            token_amount.into(),
        );

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_promises_features_address())
            .gas(30_000_000u64)
            .typed(promises_feature_proxy::PromisesFeaturesProxy)
            .forward_promise_accept_funds(to)
            .payment(payment)
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        match response {
            Ok(_) => {
                println!("forward_promise_accept_funds done successfully. Params used: {token_id} | {token_nonce} | {token_amount} | {}", to.to_bech32_expr())
            },
            Err(err) => panic!("FAILED: forward_promise_accept_funds. Reason: {}. Params used: {token_id} | {token_nonce} | {token_amount} | {}", err.message, to.to_bech32_expr()),
        }
    }

    pub async fn forward_promise_retrieve_funds(
        &mut self,
        token: &str,
        token_nonce: u64,
        amount: u64,
        to: &Bech32Address,
    ) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_promises_features_address())
            .gas(30_000_000u64)
            .typed(promises_feature_proxy::PromisesFeaturesProxy)
            .forward_promise_retrieve_funds(to, token, token_nonce, amount)
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        match response {
            Ok(_) => {
                println!("forward_promise_retrieve_funds done successfully. Params used: {token} | {token_nonce} | {amount} | {}", to.to_bech32_expr())
            },
            Err(err) => panic!("FAILED: forward_promise_retrieve_funds. Reason: {}. Params used: {token} | {token_nonce} | {amount} | {}", err.message, to.to_bech32_expr()),
        }
    }

    pub async fn forward_payment_callback(
        &mut self,
        token_id: &str,
        token_nonce: u64,
        token_amount: u64,
        to: &Bech32Address,
    ) {
        let payment: EsdtTokenPayment<StaticApi> = EsdtTokenPayment::new(
            TokenIdentifier::from(token_id),
            token_nonce,
            token_amount.into(),
        );

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_promises_features_address())
            .gas(30_000_000u64)
            .typed(promises_feature_proxy::PromisesFeaturesProxy)
            .forward_payment_callback(to)
            .payment(payment)
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        match response {
            Ok(_) => println!("forward_payment_callback done successfully. Params used: {token_id} | {token_nonce} | {token_amount} | {}", to.to_bech32_expr()),
            Err(err) => panic!("FAILED: forward_payment_callback. Reason: {}. Params used: {token_id} | {token_nonce} | {token_amount} | {}", err.message, to.to_bech32_expr()),
        }
    }

    pub async fn promise_raw_single_token(
        &mut self,
        token_id: &str,
        token_nonce: u64,
        token_amount: u64,
        to: &Bech32Address,
        endpoint_name: &str,
        gas_limit: u64,
        extra_gas_for_callback: u64,
        args: &Vec<&str>,
    ) {
        let payment: EsdtTokenPayment<StaticApi> = EsdtTokenPayment::new(
            TokenIdentifier::from(token_id),
            token_nonce,
            token_amount.into(),
        );

        let promise_args: MultiValueEncoded<StaticApi, ManagedBuffer<StaticApi>> = args
            .into_iter()
            .map(|arg| ManagedBuffer::from(arg.as_bytes()))
            .collect();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_promises_features_address())
            .gas(30_000_000u64)
            .typed(promises_feature_proxy::PromisesFeaturesProxy)
            .promise_raw_single_token(
                to,
                endpoint_name,
                gas_limit,
                extra_gas_for_callback,
                promise_args,
            )
            .payment(payment)
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        match response {
            Ok(_) => println!("promise_raw_single_token done successfully. Params used: {token_id} | {token_nonce} | {token_amount} | {} | {endpoint_name} | {gas_limit} | {extra_gas_for_callback} | {}", to.to_bech32_expr(), args.join(",")),
            Err(err) => panic!("FAILED: promise_raw_single_token. Reason: {}. Params used: {token_id} | {token_nonce} | {token_amount} | {} | {endpoint_name} | {gas_limit} | {extra_gas_for_callback} | {}", err.message, to.to_bech32_expr(), args.join(",")),
        }
    }

    pub async fn promise_raw_multi_transfer(
        &mut self,
        to: &Bech32Address,
        endpoint_name: &str,
        extra_gas_for_callback: u64,
        payment: MultiValueEncoded<StaticApi, EsdtTokenPaymentMultiValue<StaticApi>>,
    ) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_promises_features_address())
            .gas(30_000_000u64)
            .typed(promises_feature_proxy::PromisesFeaturesProxy)
            .promise_raw_multi_transfer(to, endpoint_name, extra_gas_for_callback, payment)
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        match response {
            Ok(_) => println!("promise_raw_multi_transfer done successfully. Params used: {} | {endpoint_name} | {extra_gas_for_callback}", to.to_bech32_expr()),
            Err(err) => panic!("FAILED: promise_raw_multi_transfer. Reason: {}. Params used: {} | {endpoint_name} | {extra_gas_for_callback}", err.message, to.to_bech32_expr()),
        }
    }

    pub async fn forward_sync_retrieve_funds_bt(
        &mut self,
        to: &Bech32Address,
        token: &str,
        token_nonce: u64,
        amount: u64,
    ) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_promises_features_address())
            .gas(30_000_000u64)
            .typed(promises_feature_proxy::PromisesFeaturesProxy)
            .forward_sync_retrieve_funds_bt(to, token, token_nonce, amount)
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        match response {
            Ok(_) => println!("forward_sync_retrieve_funds_bt done successfully. Params used: {} | {token} | {token_nonce} | {amount}", to.to_bech32_expr()),
            Err(err) => panic!("FAILED: forward_sync_retrieve_funds_bt. Reason: {}. Params used: {} | {token} | {token_nonce} | {amount}", err.message, to.to_bech32_expr()),
        }
    }

    pub async fn forward_sync_retrieve_funds_bt_twice(
        &mut self,
        to: &Bech32Address,
        token: &str,
        token_nonce: u64,
        amount: u64,
    ) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_promises_features_address())
            .gas(30_000_000u64)
            .typed(promises_feature_proxy::PromisesFeaturesProxy)
            .forward_sync_retrieve_funds_bt_twice(to, token, token_nonce, amount)
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        match response {
            Ok(_) => println!("forward_sync_retrieve_funds_bt_twice done successfully. Params used: {} | {token} | {token_nonce} | {amount}", to.to_bech32_expr()),
            Err(err) => panic!("FAILED: forward_sync_retrieve_funds_bt_twice. Reason: {}. Params used: {} | {token} | {token_nonce} | {amount}", err.message, to.to_bech32_expr()),
        }
    }

    pub async fn forward_promise_retrieve_funds_back_transfers(
        &mut self,
        to: &Bech32Address,
        token: &str,
        token_nonce: u64,
        amount: u64,
    ) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_promises_features_address())
            .gas(30_000_000u64)
            .typed(promises_feature_proxy::PromisesFeaturesProxy)
            .forward_promise_retrieve_funds_back_transfers(to, token, token_nonce, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }
}
