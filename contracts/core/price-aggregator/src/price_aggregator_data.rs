multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone)]
pub struct TokenPair<M: ManagedTypeApi> {
    pub from: ManagedBuffer<M>,
    pub to: ManagedBuffer<M>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct PriceFeed<M: ManagedTypeApi> {
    pub round_id: u32,
    pub from: ManagedBuffer<M>,
    pub to: ManagedBuffer<M>,
    pub timestamp: u64,
    pub price: BigUint<M>,
    pub decimals: u8,
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
