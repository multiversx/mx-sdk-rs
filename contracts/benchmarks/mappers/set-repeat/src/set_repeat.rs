#![no_std]

use benchmark_common::ExampleStruct;

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait SetRepeat: benchmark_common::BenchmarkCommon {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn add(&self, num_repeats: usize, value: ManagedBuffer) {
        let mut bench = self.bench();
        for i in 0..num_repeats {
            bench.insert(self.append_index(&value, i));
        }
    }

    #[endpoint]
    fn count(&self, value: ManagedBuffer) -> usize {
        self.bench().iter().filter(|v| *v == value).count()
    }

    #[endpoint]
    fn remove(&self, num_repeats: usize, value: ManagedBuffer) {
        let mut bench = self.bench();
        for i in 1..=num_repeats {
            bench.remove(&self.append_index(&value, i));
        }
    }

    #[view]
    #[storage_mapper("benchmark")]
    fn bench(&self) -> SetMapper<ManagedBuffer>;

    #[endpoint]
    fn add_struct(&self, num_repeats: usize, value: ExampleStruct<Self::Api>) {
        let mut bench = self.bench_struct();
        for i in 0..num_repeats {
            bench.insert(self.use_index_struct(&value, i));
        }
    }

    #[endpoint]
    fn count_struct(&self, value: ExampleStruct<Self::Api>) -> usize {
        self.bench_struct().iter().filter(|v| *v == value).count()
    }

    #[endpoint]
    fn remove_struct(&self, num_repeats: usize, value: ExampleStruct<Self::Api>) {
        let mut bench = self.bench_struct();
        for i in 1..=num_repeats {
            bench.remove(&self.use_index_struct(&value, i));
        }
    }

    #[view]
    #[storage_mapper("bench_struct")]
    fn bench_struct(&self) -> SetMapper<ExampleStruct<Self::Api>>;
}
