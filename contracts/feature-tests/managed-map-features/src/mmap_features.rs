#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait ManagedMapFeatures {
    #[init]
    fn init(&self) {}

    #[storage_get("num_entries")]
    fn get_num_entries(&self) -> usize;

    #[storage_get("key")]
    fn get_key(&self, index: usize) -> ManagedBuffer;

    #[storage_get("value")]
    fn get_value(&self, index: usize) -> ManagedBuffer;

    fn create_map(&self) -> ManagedMap {
        let mut map = ManagedMap::new();
        let num_entries = self.get_num_entries();
        for index in 0..num_entries {
            map.put(&self.get_key(index), &self.get_value(index));
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
}
