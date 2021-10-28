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
    fn count(&self, value: ManagedBuffer) -> usize {
        let bench = self.bench();
        bench.iter().filter(|(_, v)| *v == value).count()
    }

    #[endpoint]
    fn remove(&self, num_repeats: usize, key: ManagedBuffer) {
        let mut bench = self.bench();
        for i in 1..=num_repeats {
            bench.remove(&self.append_index(&key, i));
        }
    }

    #[storage_mapper("benchmark")]
    fn bench(&self) -> MapMapper<ManagedBuffer, ManagedBuffer>;
}
