multiversx_sc::imports!();

use crate::types::*;

/// Storage tests: direct load from storage to the heap.
#[multiversx_sc::module]
pub trait StorageLoadFeatures {
    #[endpoint]
    #[storage_get("vec_u8")]
    fn load_vec_u8(&self) -> Vec<u8>;

    #[endpoint]
    #[storage_get("addr")]
    fn load_addr(&self) -> Address;

    #[storage_get("opt_addr")]
    fn get_opt_addr(&self) -> Option<Address>;

    #[endpoint]
    fn load_opt_addr(&self) -> OptionalValue<Address> {
        self.get_opt_addr().into()
    }

    #[view]
    #[storage_is_empty("opt_addr")]
    fn is_empty_opt_addr(&self) -> bool;

    #[endpoint]
    #[storage_get("ser_1")]
    fn load_ser_1(&self) -> StructExampleAlloc;

    #[storage_set("slice1")]
    fn store_slice1(&self, slice: &[BigUint]);
}
