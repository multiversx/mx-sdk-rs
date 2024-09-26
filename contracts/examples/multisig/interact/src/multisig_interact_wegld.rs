use std::time::Duration;

use multiversx_sc_snippets::imports::*;

use super::*;

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
        self.perform_action(action_id, 15_000_000u64).await;
    }

    pub async fn unwrap_egld(&mut self) {
        println!("proposing unwrap egld...");
        let action_id = self.propose_unwrap_egld().await;

        println!("perfoming unwrap egld action `{action_id}`...");
        self.perform_action(action_id, 15_000_000u64).await;
    }

    pub async fn wegld_swap_set_state(&mut self) {
        self.interactor
            .retrieve_account(&self.config.wegld_address)
            .await;
    }

    async fn propose_wrap_egld(&mut self) -> usize {
        let function_call = self
            .interactor
            .tx()
            .to(&self.config.wegld_address)
            .typed(wegld_proxy::EgldEsdtSwapProxy)
            .wrap_egld()
            .into_function_call();

        let action_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_multisig_address())
            .gas(NumExpr("10,000,000"))
            .typed(multisig_proxy::MultisigProxy)
            .propose_async_call(&self.config.wegld_address, WRAP_AMOUNT, function_call)
            .returns(ReturnsResult)
            .prepare_async()
            .run()
            .await;

        println!("successfully proposed wrap egld action `{action_id}`");
        action_id
    }

    pub async fn query_wegld_token_identifier(&mut self) -> TokenIdentifier<StaticApi> {
        let wegld_token_id = self
            .interactor
            .query()
            .to(&self.config.wegld_address)
            .typed(wegld_proxy::EgldEsdtSwapProxy)
            .wrapped_egld_token_id()
            .returns(ReturnsResult)
            .prepare_async()
            .run()
            .await;

        println!("WEGLD token identifier: {wegld_token_id}");

        wegld_token_id
    }

    async fn propose_unwrap_egld(&mut self) -> usize {
        let wegld_token_id = self.query_wegld_token_identifier().await;

        let normalized_tx = self
            .interactor
            .tx()
            .to(&self.config.wegld_address)
            .typed(wegld_proxy::EgldEsdtSwapProxy)
            .unwrap_egld()
            .single_esdt(&wegld_token_id, 0u64, &UNWRAP_AMOUNT.into())
            .normalize();
        let normalized_to = normalized_tx.to;
        let normalized_data = normalized_tx.data;

        let action_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_multisig_address())
            .gas(NumExpr("10,000,000"))
            .typed(multisig_proxy::MultisigProxy)
            .propose_async_call(normalized_to, 0u64, normalized_data)
            .returns(ReturnsResult)
            .prepare_async()
            .run()
            .await;

        println!("successfully proposed unwrap egld action `{action_id}`");
        action_id
    }
}
