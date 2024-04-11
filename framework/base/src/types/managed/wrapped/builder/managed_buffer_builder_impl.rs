use crate::{api::ManagedTypeApi, types::ManagedBuffer};

pub trait ManagedBufferBuilderImpl<M>
where
    M: ManagedTypeApi,
{
    fn new_from_slice(slice: &[u8]) -> Self;

    fn into_managed_buffer(self) -> ManagedBuffer<M>;

    fn append_bytes(&mut self, bytes: &[u8]);

    fn append_managed_buffer(&mut self, item: &ManagedBuffer<M>);
}
