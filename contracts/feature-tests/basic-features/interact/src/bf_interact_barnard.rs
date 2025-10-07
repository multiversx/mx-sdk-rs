use basic_features::basic_features_proxy;
use multiversx_sc_snippets::{hex, imports::*};

use crate::BasicFeaturesInteract;

impl BasicFeaturesInteract {
    pub async fn epoch_info(&mut self) {
        let result = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.bf_contract())
            .gas(10_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .epoch_info()
            .returns(ReturnsResult)
            .run()
            .await;

        let (
            get_block_round_time_ms,
            epoch_start_block_timestamp_ms,
            epoch_start_block_nonce,
            epoch_start_block_round,
        ) = result.into_tuple();

        println!(
            "Result:
    get_block_round_time_ms: {get_block_round_time_ms}
    epoch_start_block_timestamp_ms: {epoch_start_block_timestamp_ms}
    epoch_start_block_nonce: {epoch_start_block_nonce}
    epoch_start_block_round: {epoch_start_block_round}"
        );
    }

    pub async fn code_hash(&mut self, address: Bech32Address) {
        let result_value = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.bf_contract())
            .gas(10_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .code_hash(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Code hash: {}", hex::encode(result_value));
    }

    pub async fn block_timestamps(&mut self) {
        let result = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.bf_contract())
            .gas(10_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_block_timestamps()
            .returns(ReturnsResult)
            .run()
            .await;

        let (prev_block_timestamp_ms, prev_block_timestamp, block_timestamp_ms, block_timestamp) =
            result.into_tuple();

        println!(
            "Result: 
    prev_block_timestamp: {prev_block_timestamp_ms} ({prev_block_timestamp})
    block_timestamp:      {block_timestamp_ms} ({block_timestamp})
        "
        );
    }

    /// TODO: move somewhere else, ideally the composability interactor
    pub async fn get_esdt_token_data(
        &mut self,
        address: Bech32Address,
        token_id: &str,
        nonce: u64,
    ) {
        let result_value = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.bf_contract())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_esdt_token_data(address, TokenIdentifier::from(token_id), nonce)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }
}
