multiversx_sc::imports!();

/// Storage mapper test.
#[multiversx_sc::module]
pub trait MapStorageMapperFeatures {
    #[storage_mapper("map_storage_mapper")]
    fn map_storage_mapper(&self) -> MapStorageMapper<u32, MapMapper<u32, u32>>;

    #[view]
    fn map_storage_mapper_view(&self) -> MultiValueEncoded<u32> {
        let mut result = MultiValueEncoded::new();
        for (key1, map) in self.map_storage_mapper().iter() {
            for (key2, value) in map.iter() {
                result.push(key1);
                result.push(key2);
                result.push(value);
            }
        }
        result
    }

    #[endpoint]
    fn map_storage_mapper_insert_default(&self, item: u32) -> bool {
        let mut map_storage_mapper = self.map_storage_mapper();
        map_storage_mapper.insert_default(item)
    }

    #[endpoint]
    fn map_storage_mapper_contains_key(&self, item: u32) -> bool {
        let map_storage_mapper = self.map_storage_mapper();
        map_storage_mapper.contains_key(&item)
    }

    #[endpoint]
    fn map_storage_mapper_get(&self, item: u32) -> MultiValueEncoded<u32> {
        let map_storage_mapper = self.map_storage_mapper();
        if let Some(map) = map_storage_mapper.get(&item) {
            let mut result = MultiValueEncoded::new();
            for (key, value) in map.iter() {
                result.push(key);
                result.push(value);
            }
            return result;
        }
        sc_panic!("No storage!")
    }

    #[endpoint]
    fn map_storage_mapper_insert_value(
        &self,
        item: u32,
        key: u32,
        value: u32,
    ) -> SCResult<Option<u32>> {
        let map_storage_mapper = self.map_storage_mapper();
        if let Some(mut map) = map_storage_mapper.get(&item) {
            return Ok(map.insert(key, value));
        }
        sc_panic!("No storage!")
    }

    #[endpoint]
    fn map_storage_mapper_get_value(&self, item: u32, key: u32) -> SCResult<Option<u32>> {
        let map_storage_mapper = self.map_storage_mapper();
        if let Some(map) = map_storage_mapper.get(&item) {
            return Ok(map.get(&key));
        }
        sc_panic!("No storage!")
    }

    #[endpoint]
    fn map_storage_mapper_remove(&self, item: u32) -> bool {
        let mut map_storage_mapper = self.map_storage_mapper();
        map_storage_mapper.remove(&item)
    }

    #[endpoint]
    fn map_storage_mapper_clear(&self) {
        let mut map_storage_mapper = self.map_storage_mapper();
        map_storage_mapper.clear();
    }

    #[endpoint]
    fn map_storage_mapper_entry_or_default_update_increment(
        &self,
        item: u32,
        key: u32,
        increment: u32,
    ) -> u32 {
        let mut map = self.map_storage_mapper().entry(item).or_default().get();
        map.entry(key).or_default().update(|value| {
            *value += increment;
            *value
        })
    }

    #[endpoint]
    fn map_storage_mapper_entry_and_modify_increment_or_default(
        &self,
        item: u32,
        key: u32,
        value: u32,
        other: u32,
    ) -> u32 {
        let map = self
            .map_storage_mapper()
            .entry(item)
            .and_modify(|map| {
                map.insert(key, value);
            })
            .or_default()
            .get();
        map.get(&key).unwrap_or(other)
    }

    #[endpoint]
    fn map_storage_mapper_entry_or_default_update(
        &self,
        item: u32,
        key: u32,
        value: u32,
    ) -> Option<u32> {
        self.map_storage_mapper()
            .entry(item)
            .or_default()
            .update(|map| map.insert(key, value))
    }
}
