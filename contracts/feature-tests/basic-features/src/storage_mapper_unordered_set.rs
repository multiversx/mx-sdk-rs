multiversx_sc::imports!();

/// Storage mapper test.
#[multiversx_sc::module]
pub trait UnorderedSetMapperFeatures {
    #[view]
    #[storage_mapper("set_mapper")]
    fn set_mapper(&self) -> UnorderedSetMapper<u32>;

    #[endpoint]
    fn set_mapper_insert(&self, item: u32) -> bool {
        let mut set_mapper = self.set_mapper();
        set_mapper.insert(item)
    }

    #[endpoint]
    fn set_mapper_contains(&self, item: u32) -> bool {
        let set_mapper = self.set_mapper();
        set_mapper.contains(&item)
    }

    #[endpoint]
    fn set_mapper_remove(&self, item: u32) -> bool {
        let mut set_mapper = self.set_mapper();
        set_mapper.remove(&item)
    }
}
