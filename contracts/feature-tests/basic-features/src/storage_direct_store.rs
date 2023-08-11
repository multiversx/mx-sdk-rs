multiversx_sc::imports!();

use crate::types::*;

/// Storage tests: direct store.
#[multiversx_sc::module]
pub trait StorageStoreFeatures {
    #[endpoint]
    #[storage_set("storage_bytes")]
    fn store_bytes(&self, bi: ManagedBuffer);

    #[endpoint]
    #[storage_set("big_uint")]
    fn store_big_uint(&self, bi: BigUint);

    #[endpoint]
    #[storage_set("big_int")]
    fn store_big_int(&self, bi: BigInt);

    #[endpoint]
    #[storage_set("usize")]
    fn store_usize(&self, i: usize);

    #[endpoint]
    #[storage_set("i32")]
    fn store_i32(&self, i: i32);

    #[endpoint]
    #[storage_set("u64")]
    fn store_u64(&self, i: u64);

    #[endpoint]
    #[storage_set("i64")]
    fn store_i64(&self, i: i64);

    #[endpoint]
    #[storage_set("bool")]
    fn store_bool(&self, i: bool);

    #[endpoint]
    #[storage_set("addr")]
    fn store_addr(&self, arg: ManagedAddress);

    #[storage_set("opt_addr")]
    fn set_opt_addr(&self, opt_addr: Option<ManagedAddress>);

    #[endpoint]
    fn store_opt_addr(&self, opt_addr: OptionalValue<ManagedAddress>) {
        self.set_opt_addr(opt_addr.into_option());
    }

    #[endpoint]
    #[storage_set("ser_2")]
    fn store_ser_2(&self, arg: ExampleEnumWithFields);

    #[endpoint]
    #[storage_set("map1")]
    fn store_map1(&self, addr: ManagedAddress, bi: BigUint);

    #[endpoint]
    #[storage_set("map2")]
    fn store_map2(&self, addr1: &ManagedAddress, addr2: &ManagedAddress, bi: &BigUint);

    #[endpoint]
    #[storage_set("map3")]
    fn store_map3(&self, x: usize, b: bool);

    #[endpoint]
    #[storage_set("ELRONDi64")]
    fn store_reserved_i64(&self, i: i64);

    #[endpoint]
    #[storage_set("ELRONDBigUint")]
    fn store_reserved_big_uint(&self, i: BigUint);

    #[endpoint]
    #[storage_set("ELRONDreserved")]
    fn store_reserved_vec_u8(&self, i: ManagedBuffer);
}
