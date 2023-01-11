#![no_std]

use benchmark_common::ExampleStruct;

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait SingleValueRepeat: benchmark_common::BenchmarkCommon {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn add(&self, num_repeats: usize, key: ManagedBuffer, value: ManagedBuffer) {
        for i in 0..num_repeats {
            self.item_at(&key, i).set(value.as_ref());
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
            self.item_at(&key, i).clear();
        }
    }

    fn item_at(&self, key: &ManagedBuffer, index: usize) -> SingleValueMapper<ManagedBuffer> {
        self.bench(self.append_index(key, index))
    }

    #[storage_mapper("benchmark")]
    fn bench(&self, key: ManagedBuffer) -> SingleValueMapper<ManagedBuffer>;

    #[endpoint]
    fn add_struct(
        &self,
        num_repeats: usize,
        key: ExampleStruct<Self::Api>,
        value: ExampleStruct<Self::Api>,
    ) {
        for i in 0..num_repeats {
            self.struct_at(&key, i).set(&value);
        }
    }

    #[endpoint]
    fn count_struct(
        &self,
        num_repeats: usize,
        key: ExampleStruct<Self::Api>,
        value: ExampleStruct<Self::Api>,
    ) -> usize {
        (0..num_repeats)
            .filter(|&i| self.struct_at(&key, i).get() == value)
            .count()
    }

    #[endpoint]
    fn remove_struct(&self, num_repeats: usize, key: ExampleStruct<Self::Api>) {
        for i in 1..=num_repeats {
            self.struct_at(&key, i).clear();
        }
    }

    fn struct_at(
        &self,
        key: &ExampleStruct<Self::Api>,
        index: usize,
    ) -> SingleValueMapper<ExampleStruct<Self::Api>> {
        self.bench_struct(self.use_index_struct(key, index))
    }

    #[storage_mapper("bench_struct")]
    fn bench_struct(
        &self,
        key: ExampleStruct<Self::Api>,
    ) -> SingleValueMapper<ExampleStruct<Self::Api>>;
}
