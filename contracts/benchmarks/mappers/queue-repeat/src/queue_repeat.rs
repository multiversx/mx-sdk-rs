#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait QueueRepeat: benchmark_common::BenchmarkCommon {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn add(&self, num_repeats: usize, value: ManagedBuffer) {
        let mut bench = self.bench();
        for i in 0..num_repeats {
            bench.push_back(self.append_index(&value, i));
        }
    }

    #[endpoint]
    fn count(&self, value: ManagedBuffer) -> usize {
        self.bench().iter().filter(|v| *v == value).count()
    }

    #[endpoint]
    fn remove(&self, num_repeats: usize) {
        let mut bench = self.bench();
        for _ in 0..num_repeats {
            bench.pop_front();
        }
    }

    #[view(getBenchmark)]
    #[storage_mapper("benchmark")]
    fn bench(&self) -> QueueMapper<ManagedBuffer>;
}
