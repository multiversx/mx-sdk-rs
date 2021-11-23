use crate::{
    api::{ErrorApi, ManagedTypeApi},
    types::{BoxedBytes, ManagedBuffer, ManagedByteArray, ManagedType},
    *,
};
use elrond_codec::*;

pub struct StorageKey<A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    pub(crate) buffer: ManagedBuffer<A>,
}

impl<A> StorageKey<A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    #[inline]
    pub fn new(_api: A, base_key: &[u8]) -> Self {
        StorageKey {
            buffer: ManagedBuffer::new_from_bytes(base_key),
        }
    }

    #[inline]
    pub fn append_bytes(&mut self, bytes: &[u8]) {
        self.buffer.append_bytes(bytes);
    }

    #[inline]
    pub fn append_managed_buffer(&mut self, buffer: &ManagedBuffer<A>) {
        self.buffer.append(buffer);
    }

    pub fn append_item<T>(&mut self, item: &T)
    where
        T: NestedEncode,
    {
        let exit_ctx = self.buffer.type_manager();
        item.dep_encode_or_exit(&mut self.buffer, exit_ctx, storage_key_append_exit);
    }

    #[inline]
    pub fn to_boxed_bytes(&self) -> BoxedBytes {
        self.buffer.to_boxed_bytes()
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for StorageKey<M> {
    #[inline]
    fn from(buffer: ManagedBuffer<M>) -> Self {
        StorageKey { buffer }
    }
}

impl<M, const N: usize> From<ManagedByteArray<M, N>> for StorageKey<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from(mba: ManagedByteArray<M, N>) -> Self {
        StorageKey { buffer: mba.buffer }
    }
}

impl<M: ManagedTypeApi> Clone for StorageKey<M> {
    fn clone(&self) -> Self {
        StorageKey {
            buffer: self.buffer.clone(),
        }
    }
}

#[inline(always)]
fn storage_key_append_exit<A>(api: A, encode_err: EncodeError) -> !
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    let mut message_buffer = ManagedBuffer::<A>::new_from_bytes(err_msg::STORAGE_KEY_ENCODE_ERROR);
    message_buffer.append_bytes(encode_err.message_bytes());
    api.signal_error_from_buffer(message_buffer.get_raw_handle())
}
