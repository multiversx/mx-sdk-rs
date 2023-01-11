use crate::ExampleStruct;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait BenchmarkCommon {
    fn append_index(&self, base: &ManagedBuffer, index: usize) -> ManagedBuffer {
        let mut concatenated = base.clone();
        concatenated.append_u32_be(index as u32);
        concatenated
    }

    fn use_index_struct(
        &self,
        base: &ExampleStruct<Self::Api>,
        index: usize,
    ) -> ExampleStruct<Self::Api> {
        let mut example_struct = base.clone();
        example_struct.first_token_nonce = index as u64;
        example_struct.second_token_nonce = index as u64;
        example_struct
    }
}
