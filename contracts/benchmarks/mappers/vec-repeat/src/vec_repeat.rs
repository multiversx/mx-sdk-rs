#![no_std]

use benchmark_common::ExampleStruct;

multiversx_sc::imports!();

#[multiversx_sc::contract]
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

    #[view]
    #[storage_mapper("benchmark")]
    fn bench(&self) -> VecMapper<ManagedBuffer>;

    #[endpoint]
    fn add_struct(&self, num_repeats: usize, value: ExampleStruct<Self::Api>) {
        let mut bench = self.bench_struct();
        for i in 0..num_repeats {
            bench.push(&self.use_index_struct(&value, i));
        }
    }

    #[endpoint]
    fn count_struct(&self, value: ExampleStruct<Self::Api>) -> usize {
        let bench = self.bench_struct();
        (1..=bench.len()).filter(|&i| bench.get(i) == value).count()
    }

    #[endpoint]
    fn remove_struct(&self, num_repeats: usize) {
        let bench = self.bench_struct();
        for i in 1..=num_repeats {
            bench.clear_entry(i);
        }
    }

    #[view]
    #[storage_mapper("bench_struct")]
    fn bench_struct(&self) -> VecMapper<ExampleStruct<Self::Api>>;
}
