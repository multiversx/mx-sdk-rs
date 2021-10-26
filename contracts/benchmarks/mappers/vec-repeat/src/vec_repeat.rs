#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait VecRepeat: benchmark_common::BenchmarkCommon {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn add(&self, num_repeats: usize, value: ManagedBuffer) {
        let mut bench = self.bench();
        for i in 0..num_repeats {
            bench.push(&self.append_index(&value, i));
        }
    }

    #[endpoint]
    fn count(&self, value: ManagedBuffer) -> usize {
        let bench = self.bench();
        (1..=bench.len()).filter(|&i| bench.get(i) == value).count()
    }

    #[endpoint]
    fn remove(&self, num_repeats: usize) {
        let bench = self.bench();
        for i in 1..=num_repeats {
            bench.clear_entry(i);
        }
    }

    #[view(getBenchmark)]
    #[storage_mapper("benchmark")]
    fn bench(&self) -> VecMapper<ManagedBuffer>;
}
