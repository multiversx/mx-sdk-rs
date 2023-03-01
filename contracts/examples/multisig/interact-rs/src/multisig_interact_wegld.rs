use multiversx_sc_snippets::multiversx_sc::types::{ContractCall, ContractCallNoPayment};
#[allow(unused_imports)]
use multiversx_sc_snippets::multiversx_sc::types::{
    EsdtTokenPayment, MultiValueEncoded, TokenIdentifier,
};

use super::*;

const WEGLD_SWAP_SC_BECH32: &str = "erd1qqqqqqqqqqqqqpgqcy2wua5cq59y6sxqj2ka3scayh5e5ms7cthqht8xtp";
const WEGLD_TOKEN_IDENTIFIER: &str = "WEGLD-6cf38e";
const AMOUNT: u64 = 50000000000000000; // 0.05 EGLD | 0.05 WEGLD

impl MultisigInteract {
    pub async fn wrap_egld(&mut self) {
        println!("proposing wrap egld...");
        let action_id = self.propose_wrap_egld().await;
        if action_id.is_none() {
            return;
        }

        let action_id = action_id.unwrap();
        println!("perfoming wrap egld action `{action_id}`...");
        self.perform_action(action_id, "15,000,000").await;
    }

    pub async fn unwrap_egld(&mut self) {
        println!("proposing unwrap egld...");
        let action_id = self.propose_unwrap_egld().await;
        if action_id.is_none() {
            return;
        }

        let action_id = action_id.unwrap();
        println!("perfoming unwrap egld action `{action_id}`...");
        self.perform_action(action_id, "15,000,000").await;
    }

    async fn propose_wrap_egld(&mut self) -> Option<usize> {
        let result = self
            .interactor
            .sc_call_get_result(
                self.state
                    .multisig()
                    .propose_async_call(
                        bech32::decode(WEGLD_SWAP_SC_BECH32),
                        AMOUNT,
                        "wrapEgld".to_string(),
                        MultiValueEncoded::new(),
                    )
                    .into_blockchain_call()
                    .from(&self.wallet_address)
                    .gas_limit("10,000,000"),
            )
            .await;

        let result = result.value();
        if result.is_err() {
            println!("propose wrap egld failed with: {}", result.err().unwrap());
            return None;
        }

        let action_id = result.unwrap();
        println!("successfully proposed wrap egld action `{action_id}`");
        Some(action_id)
    }

    async fn propose_unwrap_egld(&mut self) -> Option<usize> {
        let contract_call = ContractCallNoPayment::<DebugApi, ()>::new(
            bech32::decode(WEGLD_SWAP_SC_BECH32).into(),
            "unwrapEgld",
        )
        .with_esdt_transfer(EsdtTokenPayment::new(
            TokenIdentifier::from(WEGLD_TOKEN_IDENTIFIER),
            0u64,
            AMOUNT.into(),
        ))
        .into_normalized();

        let result = self
            .interactor
            .sc_call_get_result(
                self.state
                    .multisig()
                    .propose_async_call(
                        contract_call.basic.to,
                        0u64,
                        contract_call.basic.endpoint_name,
                        contract_call.basic.arg_buffer.into_multi_value_encoded(),
                    )
                    .into_blockchain_call()
                    .from(&self.wallet_address)
                    .gas_limit("10,000,000"),
            )
            .await;

        let result = result.value();
        if result.is_err() {
            println!("propose unwrap egld failed with: {}", result.err().unwrap());
            return None;
        }

        let action_id = result.unwrap();
        println!("successfully proposed unwrap egld action `{action_id}`");
        Some(action_id)
    }
}
