#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait MapRepeat: benchmark_common::BenchmarkCommon {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn add(&self, num_repeats: usize, key: ManagedBuffer, value: ManagedBuffer) {
        let mut bench = self.bench();
        for i in 0..num_repeats {
            bench.insert(self.append_index(&key, i), value.clone());
        }
    }

    #[endpoint]
    fn count(&self, key: ManagedBuffer) -> usize {
        let bench = self.bench();
        bench.iter().filter(|(k, _)| *k == key).count()
    }

    #[endpoint]
    fn contains(&self, key: ManagedBuffer) -> bool {
        self.bench().contains_key(&key)
    }

    #[endpoint]
    fn get(&self, num_repeats: usize, key: ManagedBuffer) -> usize {
        (0..num_repeats)
            .map(|i| {
                self.bench()
                    .get(&self.append_index(&key, i))
                    .map(|item| item.len())
                    .unwrap_or_default()
            })
            .sum()
    }

    #[endpoint]
    fn remove(&self, num_repeats: usize, key: ManagedBuffer) {
        let mut bench = self.bench();
        for i in 1..=num_repeats {
            bench.remove(&self.append_index(&key, i));
        }
    }

    #[view]
    fn len(&self) -> usize {
        self.bench().len()
    }

    #[storage_mapper("benchmark")]
    fn bench(&self) -> MapMapper<ManagedBuffer, ManagedBuffer>;
}
