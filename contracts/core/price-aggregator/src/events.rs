multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::price_aggregator_data::{TimestampedPrice, TokenPair};

#[derive(TypeAbi, TopEncode)]
pub struct NewRoundEvent<M: ManagedTypeApi> {
    price: BaseBigUint<M>,
    timestamp: u64,
    decimals: u8,
    block: u64,
    epoch: u64,
}

#[multiversx_sc::module]
pub trait EventsModule {
    fn emit_new_round_event(
        &self,
        token_pair: &TokenPair<CurrentApi>,
        price_feed: &TimestampedPrice<CurrentApi>,
    ) {
        let epoch = self.blockchain().get_block_epoch();
        self.new_round_event(
            &token_pair.from.clone(),
            &token_pair.to.clone(),
            epoch,
            &NewRoundEvent {
                price: price_feed.price.clone(),
                timestamp: price_feed.timestamp,
                decimals: price_feed.decimals,
                block: self.blockchain().get_block_nonce(),
                epoch,
            },
        )
    }

    #[event("new_round")]
    fn new_round_event(
        &self,
        #[indexed] from: &ManagedBuffer,
        #[indexed] to: &ManagedBuffer,
        #[indexed] epoch: u64,
        new_round_event: &NewRoundEvent<CurrentApi>,
    );
}
