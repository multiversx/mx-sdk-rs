use multiversx_sc::storage::StorageKey;

multiversx_sc::imports!();

/// Storage mapper test.
#[multiversx_sc::module]
pub trait VecMapperFeatures {
    #[view]
    #[storage_mapper("vec_mapper")]
    fn vec_mapper(&self) -> VecMapper<u32>;

    #[endpoint]
    fn vec_mapper_push(&self, item: u32) {
        let mut vec_mapper = self.vec_mapper();
        let _ = vec_mapper.push(&item);
    }

    #[view]
    fn vec_mapper_get(&self, index: usize) -> u32 {
        self.vec_mapper().get(index)
    }

    #[view]
    fn vec_mapper_get_at_address(&self, address: ManagedAddress, index: usize) -> u32 {
        let mapper = VecMapper::new_from_address(address, StorageKey::from("vec_mapper"));
        mapper.get(index)
    }

    #[view]
    fn vec_mapper_len(&self) -> usize {
        self.vec_mapper().len()
    }

    #[view]
    fn vec_mapper_len_at_address(&self, address: ManagedAddress) -> usize {
        let mapper: VecMapper<Self::Api, u32, _> =
            VecMapper::new_from_address(address, StorageKey::from("vec_mapper"));
        mapper.len()
    }
}
