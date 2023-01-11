#![no_std]

multiversx_sc::imports!();

use benchmark_common::ExampleStruct;

#[multiversx_sc::contract]
pub trait LinkedListRepeat: benchmark_common::BenchmarkCommon {
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
        self.bench()
            .iter()
            .filter(|item| *item.get_value_as_ref() == value)
            .count()
    }

    #[endpoint]
    fn remove(&self, num_repeats: usize) {
        let mut bench = self.bench();
        for _ in 0..num_repeats {
            bench.pop_front();
        }
    }

    #[view]
    #[storage_mapper("benchmark")]
    fn bench(&self) -> LinkedListMapper<ManagedBuffer>;

    #[endpoint]
    fn add_struct(&self, num_repeats: usize, value: ExampleStruct<Self::Api>) {
        let mut bench = self.bench_struct();
        for i in 0..num_repeats {
            bench.push_back(self.use_index_struct(&value, i));
        }
    }

    #[endpoint]
    fn count_struct(&self, value: ExampleStruct<Self::Api>) -> usize {
        self.bench_struct()
            .iter()
            .filter(|item| *item.get_value_as_ref() == value)
            .count()
    }

    #[endpoint]
    fn remove_struct(&self, num_repeats: usize) {
        let mut bench = self.bench_struct();
        for _ in 0..num_repeats {
            bench.pop_front();
        }
    }

    #[view]
    #[storage_mapper("bench_struct")]
    fn bench_struct(&self) -> LinkedListMapper<ExampleStruct<Self::Api>>;
}
