multiversx_sc::imports!();

use multiversx_sc::api::{use_raw_handle, HandleTypeInfo};

use crate::types::*;

/// Storage tests: direct load.
#[multiversx_sc::module]
pub trait StorageLoadFeatures {
    #[view]
    #[storage_get("storage_bytes")]
    fn load_bytes(&self) -> ManagedBuffer;

    #[endpoint]
    #[storage_get("big_uint")]
    fn load_big_uint(&self) -> BigUint;

    #[endpoint]
    #[storage_get("big_int")]
    fn load_big_int(&self) -> BigInt;

    #[endpoint]
    #[storage_get("u64")]
    fn load_u64(&self) -> u64;

    #[endpoint]
    #[storage_get("usize")]
    fn load_usize(&self) -> usize;

    #[endpoint]
    #[storage_get("i64")]
    fn load_i64(&self) -> i64;

    #[endpoint]
    #[storage_get("bool")]
    fn load_bool(&self) -> bool;

    #[endpoint]
    #[storage_get("addr")]
    fn load_addr(&self) -> ManagedAddress;

    #[storage_get("opt_addr")]
    fn get_opt_addr(&self) -> Option<ManagedAddress>;

    #[endpoint]
    fn load_opt_addr(&self) -> OptionalValue<ManagedAddress> {
        self.get_opt_addr().into()
    }

    #[view]
    #[storage_is_empty("opt_addr")]
    fn is_empty_opt_addr(&self) -> bool;

    #[endpoint]
    #[storage_get("nr_to_clear")]
    fn get_nr_to_clear(&self) -> u32;

    #[endpoint]
    #[storage_clear("nr_to_clear")]
    fn clear_storage_value(&self);

    #[endpoint]
    #[storage_get("ser_2")]
    fn load_ser_2(&self) -> ExampleEnumWithFields;

    #[endpoint]
    #[storage_get("map1")]
    fn load_map1(&self, addr: ManagedAddress) -> BigUint;

    #[endpoint]
    #[storage_get("map2")]
    fn load_map2(&self, addr1: &ManagedAddress, addr2: &ManagedAddress) -> BigUint;

    #[endpoint]
    #[storage_get("map3")]
    fn load_map3(&self, x: usize) -> bool;

    #[endpoint]
    fn load_from_address_raw(&self, address: ManagedAddress, key: ManagedBuffer) -> ManagedBuffer {
        // TODO: maybe wrap this kind of functionality in a StorageRawWrapper
        use multiversx_sc::api::{
            StaticVarApi, StaticVarApiImpl, StorageReadApi, StorageReadApiImpl,
        };
        let value_handle: <<Self as ContractBase>::Api as HandleTypeInfo>::ManagedBufferHandle =
            use_raw_handle(Self::Api::static_var_api_impl().next_handle());
        Self::Api::storage_read_api_impl().storage_load_from_address(
            address.get_handle(),
            key.get_handle(),
            value_handle.clone(),
        );
        ManagedBuffer::from_handle(value_handle)
    }
}
