use crate::api::{ErrorApi, ManagedTypeApi, StorageReadApi};
use crate::err_msg;
use crate::managed_codec::{ManagedTopDecode, ManagedTopDecodeInput};
use crate::types::{BoxedBytes, ManagedBuffer};
use alloc::boxed::Box;
use elrond_codec::*;

struct StorageGetManagedInput<A>
where
    A: ManagedTypeApi + StorageReadApi + ErrorApi + 'static,
{
    api: A,
    key: ManagedBuffer<A>,
}

impl<A> StorageGetManagedInput<A>
where
    A: ManagedTypeApi + StorageReadApi + ErrorApi + 'static,
{
    #[inline]
    fn new(api: A, base_key: &[u8]) -> Self {
        StorageGetManagedInput {
            api: api.clone(),
            key: ManagedBuffer::new_from_bytes(api, base_key),
        }
    }
}

impl<A> ManagedTopDecodeInput<A> for StorageGetManagedInput<A>
where
    A: ManagedTypeApi + StorageReadApi + ErrorApi + 'static,
{
    fn get_managed_buffer(&self) -> ManagedBuffer<A> {
        ManagedBuffer::new_from_raw_handle(
            self.api.clone(),
            self.api.storage_load_managed_buffer_raw(self.key.handle),
        )
    }
}

pub fn storage_get<A, T>(api: A, key: &[u8]) -> T
where
    T: ManagedTopDecode<A>,
    A: ManagedTypeApi + StorageReadApi + ErrorApi + 'static,
{
    T::top_decode_or_exit(
        StorageGetManagedInput::new(api.clone(), key),
        api,
        storage_get_exit,
    )
}

struct StorageGetInputLegacy<'k, SRA>
where
    SRA: StorageReadApi + ErrorApi + 'static,
{
    api: SRA,
    key: &'k [u8],
}

impl<'k, SRA> StorageGetInputLegacy<'k, SRA>
where
    SRA: StorageReadApi + ErrorApi + 'static,
{
    #[inline]
    fn new(api: SRA, key: &'k [u8]) -> Self {
        StorageGetInputLegacy { api, key }
    }
}

impl<'k, SRA> TopDecodeInput for StorageGetInputLegacy<'k, SRA>
where
    SRA: StorageReadApi + ErrorApi + 'static,
{
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

    fn try_get_big_uint_handle(&self) -> (bool, i32) {
        (true, self.api.storage_load_big_uint_raw(self.key))
    }

    // TODO: there is currently no API hook for storage of signed big ints
}

pub fn storage_get_old<SRA, T>(api: SRA, key: &[u8]) -> T
where
    T: TopDecode,
    SRA: StorageReadApi + ErrorApi + Clone + 'static,
{
    T::top_decode_or_exit(
        StorageGetInputLegacy::new(api.clone(), key),
        api,
        storage_get_exit,
    )
}

#[inline(always)]
fn storage_get_exit<SRA>(api: SRA, de_err: DecodeError) -> !
where
    SRA: StorageReadApi + ErrorApi + 'static,
{
    let decode_err_message =
        BoxedBytes::from_concat(&[err_msg::STORAGE_DECODE_ERROR, de_err.message_bytes()][..]);
    api.signal_error(decode_err_message.as_slice())
}
