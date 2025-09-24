use serde::{Deserialize, Serialize};

multiversx_sc::imports!();

multiversx_sc::derive_imports!();

#[type_abi]
#[derive(
    NestedEncode,
    NestedDecode,
    TopEncode,
    TopDecode,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct StructManaged<M: ManagedTypeApi> {
    pub m_buffer: ManagedBuffer<M>,
    pub m_vec_of_m_buffers: ManagedVec<M, ManagedBuffer<M>>,
}

#[multiversx_sc::module]
pub trait SerdeFeatures {
    #[endpoint]
    fn managed_serialize(&self, json: ManagedBuffer) -> StructManaged<Self::Api> {
        todo!();
    }

    #[endpoint]
    fn managed_deserialize(&self, m_struct: StructManaged<Self::Api>) -> ManagedBuffer {
        todo!();
    }
}
