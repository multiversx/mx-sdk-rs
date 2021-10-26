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
    fn count(&self, num_repeats: usize, key: ManagedBuffer, value: ManagedBuffer) -> usize {
        (0..num_repeats)
            .filter(|&i| self.item_at(&key, i).get() == value)
            .count()
    }

    #[endpoint]
    fn remove(&self, num_repeats: usize, key: ManagedBuffer) {
        for i in 1..=num_repeats {
            self.bench(self.append_index(&key, i)).clear();
        }
    }

    fn item_at(&self, key: &ManagedBuffer, index: usize) -> SingleValueMapper<ManagedBuffer> {
        self.bench(self.append_index(key, index))
    }

    #[storage_mapper("benchmark")]
    fn bench(&self, key: ManagedBuffer) -> SingleValueMapper<ManagedBuffer>;
}
