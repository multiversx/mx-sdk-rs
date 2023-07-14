use std::time::Duration;

#[allow(unused_imports)]
use multiversx_sc_snippets::multiversx_sc::types::{
    EsdtTokenPayment, MultiValueEncoded, TokenIdentifier,
};
use multiversx_sc_snippets::{
    multiversx_sc::types::{ContractCall, ContractCallNoPayment},
    multiversx_sc_scenario::{
        mandos_system::ScenarioRunner, scenario_format::interpret_trait::InterpretableFrom,
        standalone::retrieve_account_as_scenario_set_state,
    },
};

use super::*;

const WEGLD_SWAP_SC_BECH32: &str = "erd1qqqqqqqqqqqqqpgqcy2wua5cq59y6sxqj2ka3scayh5e5ms7cthqht8xtp";
const WEGLD_TOKEN_IDENTIFIER: &str = "WEGLD-6cf38e";
const WRAP_AMOUNT: u64 = 50000000000000000; // 0.05 EGLD
const UNWRAP_AMOUNT: u64 = 25000000000000000; // 0.025 WEGLD

impl MultisigInteract {
    pub async fn wegld_swap_full(&mut self) {
        self.deploy().await;
        self.feed_contract_egld().await;
        self.wrap_egld().await;
        self.interactor.sleep(Duration::from_secs(15)).await;
        self.unwrap_egld().await;
    }

    pub async fn wrap_egld(&mut self) {
        println!("proposing wrap egld...");
        let action_id = self.propose_wrap_egld().await;

        println!("perfoming wrap egld action `{action_id}`...");
        self.perform_action(action_id, "15,000,000").await;
    }

    pub async fn unwrap_egld(&mut self) {
        println!("proposing unwrap egld...");
        let action_id = self.propose_unwrap_egld().await;

        println!("perfoming unwrap egld action `{action_id}`...");
        self.perform_action(action_id, "15,000,000").await;
    }

    pub async fn wegld_swap_set_state(&mut self) {
        let scenario_raw = retrieve_account_as_scenario_set_state(
            Config::load_config().gateway().to_string(),
            WEGLD_SWAP_SC_BECH32.to_string(),
            true,
        )
        .await;

        let scenario = Scenario::interpret_from(scenario_raw, &InterpreterContext::default());

        self.interactor.pre_runners.run_scenario(&scenario);
        self.interactor.post_runners.run_scenario(&scenario);
    }

    async fn propose_wrap_egld(&mut self) -> usize {
        let action_id = self
            .interactor
            .sc_call_get_result(
                ScCallStep::new()
                    .call(self.state.multisig().propose_async_call(
                        bech32::decode(WEGLD_SWAP_SC_BECH32),
                        WRAP_AMOUNT,
                        "wrapEgld".to_string(),
                        MultiValueEncoded::new(),
                    ))
                    .from(&self.wallet_address)
                    .gas_limit("10,000,000"),
            )
            .await
            .result
            .unwrap();

        println!("successfully proposed wrap egld action `{action_id}`");
        action_id
    }

    async fn propose_unwrap_egld(&mut self) -> usize {
        let contract_call = ContractCallNoPayment::<StaticApi, ()>::new(
            bech32::decode(WEGLD_SWAP_SC_BECH32).into(),
            "unwrapEgld",
        )
        .with_esdt_transfer(EsdtTokenPayment::new(
            TokenIdentifier::from(WEGLD_TOKEN_IDENTIFIER),
            0u64,
            UNWRAP_AMOUNT.into(),
        ))
        .into_normalized();

        let action_id = self
            .interactor
            .sc_call_get_result(
                ScCallStep::new()
                    .call(self.state.multisig().propose_async_call(
                        contract_call.basic.to,
                        0u64,
                        contract_call.basic.endpoint_name,
                        contract_call.basic.arg_buffer.into_multi_value_encoded(),
                    ))
                    .from(&self.wallet_address)
                    .gas_limit("10,000,000"),
            )
            .await
            .result
            .unwrap();

        println!("successfully proposed unwrap egld action `{action_id}`");
        action_id
    }
}
