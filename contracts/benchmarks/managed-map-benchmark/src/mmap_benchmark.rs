#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait ManagedMapBenchmark {
    #[init]
    fn init(&self) {}

    fn create_map(&self) -> ManagedMap {
        let mut map = ManagedMap::new();
        map.put(&"key0".into(), &"value0".into());
        map.put(&"key1".into(), &"value1".into());
        map
    }

    #[view]
    fn mm_get(&self, key: &ManagedBuffer, repeats: usize) -> ManagedBuffer {
        let map = self.create_map();
        for _ in 0..repeats {
            map.get(key);
        }
        map.get(key)
    }

    #[view]
    fn mm_contains(&self, key: &ManagedBuffer, repeats: usize) -> bool {
        let map = self.create_map();
        for _ in 0..repeats {
            map.contains(key);
        }
        map.contains(key)
    }

    #[endpoint]
    fn mm_remove(&self, remove_key: &ManagedBuffer, repeats: usize) {
        let mut map = self.create_map();
        for _ in 0..repeats {
            map.remove(remove_key);
        }
    }
}
