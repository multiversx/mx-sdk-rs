use multiversx_sc::{api::ManagedTypeApi, types::BigUint};

use multiversx_sc::derive_imports::*;

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone)]
pub enum UnlockType<M: ManagedTypeApi> {
    FixedAmount {
        period_unlock_amount: BigUint<M>,
        release_period: u64,
        release_ticks: u64,
    },
    Percentage {
        period_unlock_percentage: u8,
        release_period: u64,
        release_ticks: u64,
    },
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, Clone)]
pub struct Schedule<M: ManagedTypeApi> {
    pub group_total_amount: BigUint<M>,
    pub unlock_type: UnlockType<M>,
}
