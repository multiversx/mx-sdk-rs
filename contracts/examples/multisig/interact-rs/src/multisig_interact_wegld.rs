use multiversx_sc_snippets::multiversx_sc::types::MultiValueEncoded;

use super::*;

const WEGLD_SWAP_SC_BECH32: &str = "erd1qqqqqqqqqqqqqpgqcy2wua5cq59y6sxqj2ka3scayh5e5ms7cthqht8xtp";

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

    async fn propose_wrap_egld(&mut self) -> Option<usize> {
        let result = self
            .interactor
            .sc_call_get_result(
                self.state
                    .multisig()
                    .propose_async_call(
                        bech32::decode(WEGLD_SWAP_SC_BECH32),
                        BigUintValue::from("0,050000000000000000").value,
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
}
