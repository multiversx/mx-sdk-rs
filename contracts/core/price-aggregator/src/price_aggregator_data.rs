multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone)]
pub struct TokenPair<M: ManagedTypeApi> {
    pub from: ManagedBuffer<M>,
    pub to: ManagedBuffer<M>,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct PriceFeed<M: ManagedTypeApi> {
    pub round_id: u32,
    pub from: ManagedBuffer<M>,
    pub to: ManagedBuffer<M>,
    pub timestamp: u64,
    pub price: BigUint<M>,
    pub decimals: u8,
}

#[type_abi]
#[derive(TopEncode, TopDecode, Debug, PartialEq, Eq)]
pub struct TimestampedPrice<M: ManagedTypeApi> {
    pub price: BigUint<M>,
    pub timestamp: u64,
    pub decimals: u8,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Debug, PartialEq, Eq)]
pub struct OracleStatus {
    pub accepted_submissions: u64,
    pub total_submissions: u64,
}
