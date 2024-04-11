use crate::{api::ManagedTypeApi, types::ManagedBuffer};

use super::ManagedBufferBuilderImpl;

/// Basic implementation of a ManagedBuffer builder, no caching.
///
/// It is the ManagedBuffer itself, we just append to it each time.
pub struct ManagedBufferBuilderImplBasic<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    managed_buffer: ManagedBuffer<'a, M>,
}

impl<'a, M> ManagedBufferBuilderImpl<'a, M> for ManagedBufferBuilderImplBasic<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn new_from_slice(slice: &[u8]) -> Self {
        ManagedBufferBuilderImplBasic {
            managed_buffer: slice.into(),
        }
    }

    #[inline]
    fn into_managed_buffer(self) -> ManagedBuffer<'a, M> {
        self.managed_buffer
    }

    #[inline]
    fn append_bytes(&mut self, bytes: &[u8]) {
        self.managed_buffer.append_bytes(bytes);
    }

    #[inline]
    fn append_managed_buffer(&mut self, item: &ManagedBuffer<'a, M>) {
        self.managed_buffer.append(item);
    }
}
