elrond_wasm::imports!();

/// Storage mapper test.
#[elrond_wasm::module]
pub trait MapMapperFeatures {
    #[storage_mapper("map_mapper")]
    fn map_mapper(&self) -> MapMapper<u32, u32>;

    #[view]
    fn map_mapper_keys(&self) -> MultiResultVec<u32> {
        self.map_mapper().keys().collect()
    }

    #[view]
    fn map_mapper_values(&self) -> MultiResultVec<u32> {
        self.map_mapper().values().collect()
    }

    #[endpoint]
    fn map_mapper_insert(&self, item: u32, value: u32) -> Option<u32> {
        let mut map_mapper = self.map_mapper();
        map_mapper.insert(item, value)
    }

    #[endpoint]
    fn map_mapper_contains_key(&self, item: u32) -> bool {
        let map_mapper = self.map_mapper();
        map_mapper.contains_key(&item)
    }

    #[endpoint]
    fn map_mapper_get(&self, item: u32) -> Option<u32> {
        let map_mapper = self.map_mapper();
        map_mapper.get(&item)
    }

    #[endpoint]
    fn map_mapper_remove(&self, item: u32) -> Option<u32> {
        let mut map_mapper = self.map_mapper();
        map_mapper.remove(&item)
    }

    #[endpoint]
    fn map_mapper_entry_or_default_update_increment(&self, item: u32, increment: u32) -> u32 {
        self.map_mapper().entry(item).or_default().update(|value| {
            *value += increment;
            *value
        })
    }

    #[endpoint]
    fn map_mapper_entry_or_insert_default(&self, item: u32, default: u32) -> u32 {
        let mut mapper = self.map_mapper();
        let entry = mapper.entry(item);
        entry.or_insert_with(|| default).get()
    }

    #[endpoint]
    fn map_mapper_entry_and_modify(&self, item: u32, increment: u32, otherwise: u32) -> u32 {
        self.map_mapper()
            .entry(item)
            .and_modify(|value| *value += increment)
            .or_insert(otherwise)
            .get()
    }

    #[endpoint]
    fn map_mapper_entry_or_insert_with_key(&self, item: u32, key_increment: u32) -> u32 {
        self.map_mapper()
            .entry(item)
            .or_insert_with_key(|key| key + key_increment)
            .get()
    }
}
