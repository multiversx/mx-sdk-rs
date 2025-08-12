use crate::{api::ManagedTypeApi, types::ManagedVec};
use postcard::from_bytes;
use serde::de::DeserializeOwned;

pub fn from_managed_vec<M: ManagedTypeApi, T: DeserializeOwned>(
    mv: &ManagedVec<M, u8>,
) -> Result<T, postcard::Error> {
    mv.with_self_as_slice(|mv_slice| from_bytes(mv_slice))
}
