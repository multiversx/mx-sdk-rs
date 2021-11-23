use elrond_codec::TryStaticCast;

use crate::api::ErrorApi;

use super::{BigIntApi, EllipticCurveApi, ManagedBufferApi, StaticBufferApi};

pub type Handle = i32;

pub trait ManagedTypeApi:
    TryStaticCast
    + BigIntApi
    + EllipticCurveApi
    + ManagedBufferApi
    + StaticBufferApi
    + ErrorApi
    + Clone
    + 'static
{
    fn instance() -> Self;

    fn mb_to_big_int_unsigned(&self, buffer_handle: Handle) -> Handle;

    fn mb_to_big_int_signed(&self, buffer_handle: Handle) -> Handle;

    fn mb_from_big_int_unsigned(&self, big_int_handle: Handle) -> Handle;

    fn mb_from_big_int_signed(&self, big_int_handle: Handle) -> Handle;

    fn mb_overwrite_static_buffer(&self, buffer_handle: Handle) -> bool {
        self.with_lockable_static_buffer(|lsb| {
            let len = self.mb_len(buffer_handle);
            lsb.try_lock_with_copy_bytes(len, |dest| {
                let _ = self.mb_load_slice(buffer_handle, 0, dest);
            })
        })
    }

    fn append_mb_to_static_buffer(&self, buffer_handle: Handle) -> bool {
        self.with_lockable_static_buffer(|lsb| {
            let len = self.mb_len(buffer_handle);
            lsb.try_extend_from_copy_bytes(len, |dest| {
                let _ = self.mb_load_slice(buffer_handle, 0, dest);
            })
        })
    }

    fn append_static_buffer_to_mb(&self, buffer_handle: Handle) {
        self.with_lockable_static_buffer(|lsb| {
            self.mb_append_bytes(buffer_handle, lsb.as_slice());
        });
    }
}
