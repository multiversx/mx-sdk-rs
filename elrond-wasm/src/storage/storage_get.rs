use crate::api::{ErrorApi, ManagedTypeApi, StorageReadApi};
use crate::err_msg;
use crate::types::{BigInt, BigUint, BoxedBytes, ManagedBuffer, ManagedBufferNestedDecodeInput};
use alloc::boxed::Box;
use elrond_codec::*;

struct StorageGetInput<'k, SRA>
where
    SRA: StorageReadApi + ManagedTypeApi + ErrorApi + 'static,
{
    api: SRA,
    key: &'k [u8],
}

impl<'k, SRA> StorageGetInput<'k, SRA>
where
    SRA: StorageReadApi + ManagedTypeApi + ErrorApi + 'static,
{
    #[inline]
    fn new(api: SRA, key: &'k [u8]) -> Self {
        StorageGetInput { api, key }
    }

    fn to_managed_buffer(&self) -> ManagedBuffer<SRA> {
        let key_handle = self.api.mb_new_from_bytes(self.key);
        let mbuf_handle = self.api.storage_load_managed_buffer_raw(key_handle);
        ManagedBuffer::new_from_raw_handle(self.api.clone(), mbuf_handle)
    }

    fn to_big_uint(&self) -> BigUint<SRA> {
        let bu_handle = self.api.storage_load_big_uint_raw(self.key);
        BigUint::from_raw_handle(self.api.clone(), bu_handle)
    }

    fn to_big_int(&self) -> BigInt<SRA> {
        BigInt::from_signed_bytes_be_buffer(&self.to_managed_buffer())
    }
}

impl<'k, SRA> TopDecodeInput for StorageGetInput<'k, SRA>
where
    SRA: StorageReadApi + ManagedTypeApi + ErrorApi + 'static,
{
    type NestedBuffer = ManagedBufferNestedDecodeInput<SRA>;

    fn byte_len(&self) -> usize {
        self.api.storage_load_len(self.key)
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        self.api.storage_load_boxed_bytes(self.key).into_box()
    }

    fn into_u64(self) -> u64 {
        self.api.storage_load_u64(self.key)
    }

    fn into_i64(self) -> i64 {
        self.api.storage_load_i64(self.key)
    }

    fn into_specialized<T, F>(self, else_deser: F) -> Result<T, DecodeError>
    where
        T: TryStaticCast,
        F: FnOnce(Self) -> Result<T, DecodeError>,
    {
        if let Some(result) = try_execute_then_cast(|| self.to_managed_buffer()) {
            Ok(result)
        } else if let Some(result) = try_execute_then_cast(|| self.to_big_uint()) {
            Ok(result)
        } else if let Some(result) = try_execute_then_cast(|| self.to_big_int()) {
            Ok(result)
        } else {
            else_deser(self)
        }
    }

    fn into_nested_buffer(self) -> Self::NestedBuffer {
        ManagedBufferNestedDecodeInput::new(self.to_managed_buffer())
    }
}

pub fn storage_get<SRA, T>(api: SRA, key: &[u8]) -> T
where
    T: TopDecode,
    SRA: StorageReadApi + ManagedTypeApi + ErrorApi + Clone + 'static,
{
    T::top_decode_or_exit(
        StorageGetInput::new(api.clone(), key),
        api,
        storage_get_exit,
    )
}

#[inline(always)]
fn storage_get_exit<SRA>(api: SRA, de_err: DecodeError) -> !
where
    SRA: StorageReadApi + ManagedTypeApi + ErrorApi + 'static,
{
    let decode_err_message =
        BoxedBytes::from_concat(&[err_msg::STORAGE_DECODE_ERROR, de_err.message_bytes()][..]);
    api.signal_error(decode_err_message.as_slice())
}
