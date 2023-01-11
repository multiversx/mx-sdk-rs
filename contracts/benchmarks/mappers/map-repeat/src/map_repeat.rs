#![no_std]

use benchmark_common::ExampleStruct;

multiversx_sc::imports!();

#[multiversx_sc::contract]
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

    #[endpoint]
    fn add_struct(
        &self,
        num_repeats: usize,
        key: ExampleStruct<Self::Api>,
        value: ExampleStruct<Self::Api>,
    ) {
        let mut bench = self.bench_struct();
        for i in 0..num_repeats {
            bench.insert(self.use_index_struct(&key, i), value.clone());
        }
    }

    #[endpoint]
    fn count_struct(&self, value: ExampleStruct<Self::Api>) -> usize {
        let bench = self.bench_struct();
        bench.iter().filter(|(_, v)| *v == value).count()
    }

    #[endpoint]
    fn remove_struct(&self, num_repeats: usize, key: ExampleStruct<Self::Api>) {
        let mut bench = self.bench_struct();
        for i in 1..=num_repeats {
            bench.remove(&self.use_index_struct(&key, i));
        }
    }

    #[storage_mapper("bench_struct")]
    fn bench_struct(&self) -> MapMapper<ExampleStruct<Self::Api>, ExampleStruct<Self::Api>>;
}
