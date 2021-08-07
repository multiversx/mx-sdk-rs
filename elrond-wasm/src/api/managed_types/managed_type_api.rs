use crate::api::ErrorApi;

use super::{BigIntApi, ManagedBufferApi};

pub type Handle = i32;

pub trait ManagedTypeApi: BigIntApi + ManagedBufferApi + ErrorApi + Clone {
    fn managed_buffer_to_big_int_signed(&self, buffer_handle: Handle) -> Handle;
}
