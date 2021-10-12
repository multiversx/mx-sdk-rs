#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait SingleValueRepeat: benchmark_common::BenchmarkCommon {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn add(&self, num_repeats: usize, key: ManagedBuffer, value: ManagedBuffer) {
        for i in 0..num_repeats {
            self.bench(self.append_index(&key, i)).set(&value);
        }
    }

    #[endpoint]
    fn get(&self, num_repeats: usize, key: ManagedBuffer) -> usize {
        (0..num_repeats)
            .map(|i| self.bench(self.append_index(&key, i)).get().len())
            .sum()
    }

    #[endpoint]
    fn remove(&self, num_repeats: usize, key: ManagedBuffer) {
        for i in 1..=num_repeats {
            self.bench(self.append_index(&key, i)).clear();
        }
    }

    #[storage_mapper("benchmark")]
    fn bench(&self, key: ManagedBuffer) -> SingleValueMapper<ManagedBuffer>;
}
