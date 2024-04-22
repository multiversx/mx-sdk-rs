use std::time::Duration;

use multiversx_sc_snippets::imports::*;

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
            .retrieve_account(&Bech32Address::from_bech32_string(
                WEGLD_SWAP_SC_BECH32.to_owned(),
            ))
            .await;
    }

    async fn propose_wrap_egld(&mut self) -> usize {
        let action_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_multisig_address())
            .gas(NumExpr("10,000,000"))
            .typed(multisig_proxy::MultisigProxy)
            .propose_async_call(
                bech32::decode(WEGLD_SWAP_SC_BECH32),
                WRAP_AMOUNT,
                FunctionCall::new("wrapEgld"),
            )
            .returns(ReturnsResult)
            .prepare_async()
            .run()
            .await;

        println!("successfully proposed wrap egld action `{action_id}`");
        action_id
    }

    async fn propose_unwrap_egld(&mut self) -> usize {
        let to = ManagedAddress::<StaticApi>::from(bech32::decode(WEGLD_SWAP_SC_BECH32));
        let payment = EsdtTokenPayment::new(
            TokenIdentifier::from(WEGLD_TOKEN_IDENTIFIER),
            0u64,
            UNWRAP_AMOUNT.into(),
        );
        let function_call = FunctionCall::new("unwrapEgld");

        let action_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_multisig_address())
            .gas(NumExpr("10,000,000"))
            .typed(multisig_proxy::MultisigProxy)
            .propose_async_call(to, 0u64, function_call)
            .esdt(payment)
            .returns(ReturnsResult)
            .prepare_async()
            .run()
            .await;

        println!("successfully proposed unwrap egld action `{action_id}`");
        action_id
    }
}
