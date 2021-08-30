use crate::api::{ErrorApi, ManagedTypeApi};
use crate::types::{BoxedBytes, ManagedBuffer};
use crate::*;
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
    pub fn new(api: A, base_key: &[u8]) -> Self {
        StorageKey {
            buffer: ManagedBuffer::new_from_bytes(api, base_key),
        }
    }

    #[inline]
    pub fn append_bytes(&mut self, bytes: &[u8]) {
        self.buffer.append_bytes(bytes)
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
    api.signal_error(encode_err.message_bytes())
}
