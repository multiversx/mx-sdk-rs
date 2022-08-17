elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone)]
pub struct TokenPair<M: ManagedTypeApi> {
    pub from: ManagedBuffer<M>,
    pub to: ManagedBuffer<M>,
}

pub type PriceFeedMultiValue<M> =
    MultiValue6<u32, ManagedBuffer<M>, ManagedBuffer<M>, u64, BigUint<M>, u8>;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct PriceFeed<M: ManagedTypeApi> {
    pub round_id: u32,
    pub from: ManagedBuffer<M>,
    pub to: ManagedBuffer<M>,
    pub timestamp: u64,
    pub price: BigUint<M>,
    pub decimals: u8,
}

impl<M: ManagedTypeApi> PriceFeed<M> {
    pub fn into_multi_value(self) -> PriceFeedMultiValue<M> {
        (
            self.round_id,
            self.from,
            self.to,
            self.timestamp,
            self.price,
            self.decimals,
        )
            .into()
    }
}

#[derive(TopEncode, TopDecode, TypeAbi, Debug, PartialEq, Eq)]
pub struct TimestampedPrice<M: ManagedTypeApi> {
    pub price: BigUint<M>,
    pub timestamp: u64,
    pub decimals: u8,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Debug, PartialEq, Eq)]
pub struct OracleStatus {
    pub accepted_submissions: u64,
    pub total_submissions: u64,
}
