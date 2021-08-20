use crate::api::{ErrorApi, ManagedTypeApi, StorageWriteApi};
use crate::types::ManagedBuffer;
use crate::*;
use elrond_codec::*;

struct StorageSetOutput<'k, SWA>
where
    SWA: StorageWriteApi + ManagedTypeApi + ErrorApi + 'static,
{
    api: SWA,
    key: &'k [u8],
}

impl<'k, SWA> StorageSetOutput<'k, SWA>
where
    SWA: StorageWriteApi + ManagedTypeApi + ErrorApi + 'static,
{
    #[inline]
    fn new(api: SWA, key: &'k [u8]) -> Self {
        StorageSetOutput { api, key }
    }

    fn set_managed_buffer(&self, managed_buffer: &ManagedBuffer<SWA>) {
        let key_handle = self.api.mb_new_from_bytes(self.key);
        self.api
            .storage_store_managed_buffer_raw(key_handle, managed_buffer.handle);
    }
}

impl<'k, SWA> TopEncodeOutput for StorageSetOutput<'k, SWA>
where
    SWA: StorageWriteApi + ManagedTypeApi + ErrorApi + 'static,
{
    type NestedBuffer = ManagedBuffer<SWA>;

    fn set_slice_u8(self, bytes: &[u8]) {
        self.api.storage_store_slice_u8(self.key, bytes)
    }

    fn set_u64(self, value: u64) {
        self.api.storage_store_u64(self.key, value);
    }

    fn set_i64(self, value: i64) {
        self.api.storage_store_i64(self.key, value);
    }

    fn set_specialized<T: TryStaticCast>(&self, value: &T) -> bool {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<SWA>>() {
            self.set_managed_buffer(managed_buffer);
            true
        } else {
            false
        }
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer {
        ManagedBuffer::new_empty(self.api.clone())
    }

    fn finalize_nested_encode(self, nb: Self::NestedBuffer) {
        self.set_managed_buffer(&nb);
    }

    #[inline]
    fn set_big_uint_handle_or_bytes<F: FnOnce() -> Vec<u8>>(self, handle: i32, _else_bytes: F) {
        self.api.storage_store_big_uint_raw(self.key, handle);
    }

    // TODO: there is currently no API hook for storage of signed big ints
}

// #[inline]
pub fn storage_set<SWA, T>(api: SWA, key: &[u8], value: &T)
where
    T: TopEncode,
    SWA: StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
{
    value.top_encode_or_exit(
        StorageSetOutput::new(api.clone(), key),
        api,
        storage_set_exit,
    );
}

#[inline(always)]
fn storage_set_exit<SWA>(api: SWA, encode_err: EncodeError) -> !
where
    SWA: StorageWriteApi + ManagedTypeApi + ErrorApi + 'static,
{
    api.signal_error(encode_err.message_bytes())
}
