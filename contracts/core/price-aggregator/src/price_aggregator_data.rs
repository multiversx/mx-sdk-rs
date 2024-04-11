multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone)]
pub struct TokenPair<'a, M: ManagedTypeApi<'a>> {
    pub from: ManagedBuffer<'a, M>,
    pub to: ManagedBuffer<'a, M>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct PriceFeed<'a, M: ManagedTypeApi<'a>> {
    pub round_id: u32,
    pub from: ManagedBuffer<'a, M>,
    pub to: ManagedBuffer<'a, M>,
    pub timestamp: u64,
    pub price: BigUint<'a, M>,
    pub decimals: u8,
}

#[derive(TopEncode, TopDecode, TypeAbi, Debug, PartialEq, Eq)]
pub struct TimestampedPrice<'a, M: ManagedTypeApi<'a>> {
    pub price: BigUint<'a, M>,
    pub timestamp: u64,
    pub decimals: u8,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Debug, PartialEq, Eq)]
pub struct OracleStatus {
    pub accepted_submissions: u64,
    pub total_submissions: u64,
}
