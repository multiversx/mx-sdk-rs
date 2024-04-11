use multiversx_sc::{api::ManagedTypeApi, types::BigUint};

use multiversx_sc::derive_imports::*;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, TypeAbi, Clone)]
pub enum UnlockType<'a, M: ManagedTypeApi<'a>> {
    FixedAmount {
        period_unlock_amount: BigUint<'a, M>,
        release_period: u64,
        release_ticks: u64,
    },
    Percentage {
        period_unlock_percentage: u8,
        release_period: u64,
        release_ticks: u64,
    },
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, TypeAbi, Clone)]
pub struct Schedule<'a, M: ManagedTypeApi<'a>> {
    pub group_total_amount: BigUint<'a, M>,
    pub unlock_type: UnlockType<'a, M>,
}
