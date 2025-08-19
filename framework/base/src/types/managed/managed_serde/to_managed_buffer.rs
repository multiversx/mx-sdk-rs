use serde::Serialize;

use crate::{
    api::ManagedTypeApi,
    types::{to_managed_vec::to_managed_vec, ManagedBuffer},
};

struct ManagedBufferWriter<'a, M: ManagedTypeApi> {
    buffer: &'a mut ManagedBuffer<M>,
}

pub fn to_managed_buffer<M: ManagedTypeApi, T: Serialize>(obj: &T) -> ManagedBuffer<M> {
    let mv = to_managed_vec(obj);
    mv.buffer
}
