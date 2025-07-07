multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ManagedMapFeatures {
    #[storage_get("mm-num-entries")]
    fn get_mm_num_entries(&self) -> usize;

    #[storage_get("mm-key")]
    fn get_mm_key(&self, index: usize) -> ManagedBuffer;

    #[storage_get("mm-value")]
    fn get_mm_value(&self, index: usize) -> ManagedBuffer;

    fn create_map(&self) -> ManagedMap {
        let mut map = ManagedMap::new();
        let num_entries = self.get_mm_num_entries();
        for index in 0..num_entries {
            map.put(&self.get_mm_key(index), &self.get_mm_value(index));
        }
        map
    }

    #[view]
    fn mm_get(&self, key: &ManagedBuffer) -> ManagedBuffer {
        self.create_map().get(key)
    }

    #[view]
    fn mm_contains(&self, key: &ManagedBuffer) -> bool {
        self.create_map().contains(key)
    }

    #[view]
    fn mm_remove_get(
        &self,
        remove_key: &ManagedBuffer,
        get_key: &ManagedBuffer,
    ) -> MultiValue2<ManagedBuffer, ManagedBuffer> {
        let mut map = self.create_map();
        let removed_value = map.remove(remove_key);
        let get_value = map.get(get_key);
        (removed_value, get_value).into()
    }

    #[view]
    fn mm_mutable_input_test(
        &self,
        mut key: ManagedBuffer,
        mut value: ManagedBuffer,
    ) -> MultiValue2<ManagedBuffer, ManagedBuffer> {
        let mut map = self.create_map();
        map.put(&key, &value);
        let original_key = key.clone();
        key.append_bytes(b"...changed");
        value.append_bytes(b"...changed");
        let value1 = map.get(&original_key);
        let value2 = map.get(&key);
        (value1, value2).into()
    }
}
