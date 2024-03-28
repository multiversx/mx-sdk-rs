use multiversx_sc::{derive_imports::*, imports::*};

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Zombie<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub dna: u64,
    pub level: u16,
    pub ready_time: u64,
    pub win_count: usize,
    pub loss_count: usize,
}
