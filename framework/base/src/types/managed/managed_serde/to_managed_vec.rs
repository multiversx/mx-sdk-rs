use crate::types::ManagedVec;
use postcard::to_slice;
use serde::Serialize;

use crate::api::ManagedTypeApi;

pub fn to_managed_vec<M: ManagedTypeApi, T: Serialize>(obj: &T) -> ManagedVec<M, u8> {
    let mut buffer: ManagedVec<M, u8> = ManagedVec::new();

    const MAX: usize = 1024;
    let mut temp = [0u8; MAX];

    let len = to_slice(obj, &mut temp)
        .expect("serialization failed")
        .len();

    for byte in &temp[..len] {
        buffer.push(*byte);
    }

    buffer
}
