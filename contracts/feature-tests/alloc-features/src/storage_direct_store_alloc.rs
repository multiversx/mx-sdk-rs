multiversx_sc::imports!();

use crate::types::*;

/// Storage tests: direct store from the heap to storage.
#[multiversx_sc::module]
pub trait StorageStoreFeatures {
    #[endpoint]
    #[storage_set("vec_u8")]
    fn store_vec_u8(&self, arg: Vec<u8>);

    #[endpoint]
    #[storage_set("addr")]
    fn store_addr(&self, arg: Address);

    #[storage_set("opt_addr")]
    fn set_opt_addr(&self, opt_addr: Option<Address>);

    #[endpoint]
    fn store_opt_addr(&self, opt_addr: OptionalValue<Address>) {
        self.set_opt_addr(opt_addr.into_option());
    }

    #[endpoint]
    #[storage_set("ser_1")]
    fn store_ser_1(&self, arg: StructExampleAlloc);
}
