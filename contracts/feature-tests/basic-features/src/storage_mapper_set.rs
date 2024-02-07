multiversx_sc::imports!();

/// Storage mapper test.
#[multiversx_sc::module]
pub trait SetMapperFeatures {
    #[view]
    #[storage_mapper("set_mapper")]
    fn set_mapper(&self) -> SetMapper<u32>;

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

    #[endpoint]
    fn set_mapper_front(&self) -> u32 {
        let set_mapper = self.set_mapper();
        set_mapper.front().unwrap_or_default()
    }

    #[endpoint]
    fn set_mapper_back(&self) -> u32 {
        let set_mapper = self.set_mapper();
        set_mapper.back().unwrap_or_default()
    }

    #[endpoint]
    fn set_mapper_next(&self, item: u32) -> u32 {
        let set_mapper = self.set_mapper();
        set_mapper.next(&item).unwrap_or_default()
    }

    #[endpoint]
    fn set_mapper_previous(&self, item: u32) -> u32 {
        let set_mapper = self.set_mapper();
        set_mapper.previous(&item).unwrap_or_default()
    }

    #[endpoint]
    fn set_mapper_iter_from_and_count(&self, item: u32) -> u32 {
        let set_mapper = self.set_mapper();
        let mut count = 0;
        for _element in set_mapper.iter_from(&item) {
            count += 1;
        }

        count
    }
}
