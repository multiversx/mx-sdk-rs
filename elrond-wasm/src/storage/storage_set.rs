use crate::api::{ErrorApi, ManagedTypeApi, StorageWriteApi};
use crate::managed_codec::{ManagedTopEncode, ManagedTopEncodeOutput};
use crate::types::ManagedBuffer;
use crate::*;
use elrond_codec::*;

struct StorageSetManagedOutput<A>
where
    A: ManagedTypeApi + StorageWriteApi + ErrorApi + 'static,
{
    api: A,
    key: ManagedBuffer<A>,
}

impl<A> StorageSetManagedOutput<A>
where
    A: ManagedTypeApi + StorageWriteApi + ErrorApi + 'static,
{
    #[inline]
    fn new(api: A, base_key: &[u8]) -> Self {
        StorageSetManagedOutput {
            api: api.clone(),
            key: ManagedBuffer::new_from_bytes(api, base_key),
        }
    }
}

impl<A> ManagedTopEncodeOutput<A> for StorageSetManagedOutput<A>
where
    A: ManagedTypeApi + StorageWriteApi + ErrorApi + 'static,
{
    fn get_api(&self) -> A {
        self.api.clone()
    }

    fn set_managed_buffer(&self, value: &ManagedBuffer<A>) {
        self.api
            .storage_store_managed_buffer_raw(self.key.handle, value.handle);
    }
}

pub fn storage_set<A, T>(api: A, key: &[u8], value: &T)
where
    T: ManagedTopEncode<A>,
    A: ManagedTypeApi + StorageWriteApi + ErrorApi + 'static,
{
    value.top_encode_or_exit(
        StorageSetManagedOutput::new(api.clone(), key),
        api,
        storage_set_exit,
    );
}

// -------------------------------------------------------------------------------------------------

struct StorageSetOutputLegacy<'k, SWA>
where
    SWA: StorageWriteApi + ErrorApi + 'static,
{
    api: SWA,
    key: &'k [u8],
}

impl<'k, SWA> StorageSetOutputLegacy<'k, SWA>
where
    SWA: StorageWriteApi + ErrorApi + 'static,
{
    #[inline]
    fn new(api: SWA, key: &'k [u8]) -> Self {
        StorageSetOutputLegacy { api, key }
    }
}

impl<'k, SWA> TopEncodeOutput for StorageSetOutputLegacy<'k, SWA>
where
    SWA: StorageWriteApi + ErrorApi + 'static,
{
    fn set_slice_u8(self, bytes: &[u8]) {
        self.api.storage_store_slice_u8(self.key, bytes)
    }

    fn set_u64(self, value: u64) {
        self.api.storage_store_u64(self.key, value);
    }

    fn set_i64(self, value: i64) {
        self.api.storage_store_i64(self.key, value);
    }

    #[inline]
    fn set_big_uint_handle_or_bytes<F: FnOnce() -> Vec<u8>>(self, handle: i32, _else_bytes: F) {
        self.api.storage_store_big_uint_raw(self.key, handle);
    }

    // TODO: there is currently no API hook for storage of signed big ints
}

// #[inline]
pub fn storage_set_old<SWA, T>(api: SWA, key: &[u8], value: &T)
where
    T: TopEncode,
    SWA: StorageWriteApi + ErrorApi + Clone + 'static,
{
    value.top_encode_or_exit(
        StorageSetOutputLegacy::new(api.clone(), key),
        api,
        storage_set_exit,
    );
}

#[inline(always)]
fn storage_set_exit<SWA>(api: SWA, encode_err: EncodeError) -> !
where
    SWA: StorageWriteApi + ErrorApi + 'static,
{
    api.signal_error(encode_err.message_bytes())
}
