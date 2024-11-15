use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Eq, Clone, Copy)]
pub enum UserStatus {
    New,
    Registered,
    Withdrawn,
}

#[type_abi]
#[derive(TopEncode, TopDecode, Default)]
pub struct ContractState<M: ManagedTypeApi> {
    pub ping_amount: BigUint<M>,
    pub deadline: u64,
    pub activation_timestamp: u64,
    pub max_funds: Option<BigUint<M>>,
    pub pong_all_last_user: usize,
}
