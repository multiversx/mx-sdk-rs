use postcard::from_bytes;
use serde::de::DeserializeOwned;

use crate::{api::ManagedTypeApi, types::ManagedBuffer};

pub fn from_managed_buffer<M: ManagedTypeApi, T: DeserializeOwned>(
    mb: &ManagedBuffer<M>,
) -> Result<T, postcard::Error> {
    mb.with_buffer_contents(|slice| from_bytes(slice))
}
