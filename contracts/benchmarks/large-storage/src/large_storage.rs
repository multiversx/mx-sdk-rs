#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub enum SampleEnum {
    Value1,
    Value2,
}
#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct Structure<M: ManagedTypeApi> {
    pub field1: ManagedBuffer<M>,
    pub field2: SampleEnum,
    pub field3: ManagedBuffer<M>,
}

#[multiversx_sc::contract]
pub trait LargeStorageBenchmark {
    #[init]
    fn init(&self) {}

    #[endpoint(saveStructure)]
    fn save_structure(&self, field1: ManagedBuffer, field2: SampleEnum, field3: ManagedBuffer) {
        let s = Structure {
            field1,
            field2,
            field3,
        };
        self.structure().set(s);
    }

    #[view(savedStructure)]
    #[storage_mapper("savedStructure")]
    fn structure(&self) -> SingleValueMapper<Structure<Self::Api>>;
}
