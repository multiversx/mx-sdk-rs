use crate::{
    api::{ErrorApi, ManagedTypeApi},
    codec::*,
    contract_base::ExitCodecErrorHandler,
    types::{heap::BoxedBytes, ManagedBuffer, ManagedByteArray, ManagedType},
    *,
};

pub struct StorageKey<'a, A>
where
    A: ManagedTypeApi<'a> + ErrorApi + 'static,
{
    pub(crate) buffer: ManagedBuffer<'a, A>,
}

impl<'a, A> ManagedType<'a, A> for StorageKey<'a, A>
where
    A: ManagedTypeApi<'a> + ErrorApi + 'static,
{
    type OwnHandle = A::ManagedBufferHandle;

    #[inline]
    fn from_handle(handle: A::ManagedBufferHandle) -> Self {
        StorageKey {
            buffer: ManagedBuffer::from_handle(handle),
        }
    }

    unsafe fn get_handle(&self) -> A::ManagedBufferHandle {
        self.buffer.get_handle()
    }

    fn take_handle(self) -> Self::OwnHandle {
        self.buffer.take_handle()
    }

    fn transmute_from_handle_ref(handle_ref: &A::ManagedBufferHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<'a, A> StorageKey<'a, A>
where
    A: ManagedTypeApi<'a> + ErrorApi + 'static,
{
    #[inline]
    pub fn new(base_key: &[u8]) -> Self {
        StorageKey {
            buffer: ManagedBuffer::new_from_bytes(base_key),
        }
    }

    #[inline]
    pub fn append_bytes(&mut self, bytes: &[u8]) {
        self.buffer.append_bytes(bytes);
    }

    #[inline]
    pub fn append_managed_buffer(&mut self, buffer: &ManagedBuffer<'a, A>) {
        self.buffer.append(buffer);
    }

    pub fn append_item<T>(&mut self, item: &T)
    where
        T: NestedEncode,
    {
        let Ok(()) = item.dep_encode_or_handle_err(
            &mut self.buffer,
            ExitCodecErrorHandler::<'a, A>::from(err_msg::STORAGE_KEY_ENCODE_ERROR),
        );
    }

    #[inline]
    pub fn to_boxed_bytes(&self) -> BoxedBytes {
        self.buffer.to_boxed_bytes()
    }
}

impl<'a, M: ManagedTypeApi<'a>> From<ManagedBuffer<'a, M>> for StorageKey<'a, M> {
    #[inline]
    fn from(buffer: ManagedBuffer<'a, M>) -> Self {
        StorageKey { buffer }
    }
}

impl<'a, M: ManagedTypeApi<'a>> From<&str> for StorageKey<'a, M> {
    #[inline]
    fn from(s: &str) -> Self {
        StorageKey {
            buffer: ManagedBuffer::from(s),
        }
    }
}

impl<'a, M, const N: usize> From<ManagedByteArray<'a, M, N>> for StorageKey<'a, M>
where
    M: ManagedTypeApi<'a> + ErrorApi,
{
    #[inline]
    fn from(mba: ManagedByteArray<'a, M, N>) -> Self {
        StorageKey { buffer: mba.buffer }
    }
}

impl<'a, M: ManagedTypeApi<'a>> Clone for StorageKey<'a, M> {
    fn clone(&self) -> Self {
        StorageKey {
            buffer: self.buffer.clone(),
        }
    }
}
