multiversx_sc::imports!();

/// Storage mapper test.
#[multiversx_sc::module]
pub trait UnorderedSetMapperFeatures {
    #[view]
    #[storage_mapper("unordered_set_mapper")]
    fn unordered_set_mapper(&self) -> UnorderedSetMapper<u32>;

    #[endpoint]
    fn unordered_set_mapper_insert(&self, item: u32) -> bool {
        let mut set_mapper = self.unordered_set_mapper();
        set_mapper.insert(item)
    }

    #[endpoint]
    fn unordered_set_mapper_contains(&self, item: u32) -> bool {
        let set_mapper = self.unordered_set_mapper();
        set_mapper.contains(&item)
    }

    #[endpoint]
    fn unordered_set_mapper_remove(&self, item: u32) -> bool {
        let mut set_mapper = self.unordered_set_mapper();
        set_mapper.swap_remove(&item)
    }
}
