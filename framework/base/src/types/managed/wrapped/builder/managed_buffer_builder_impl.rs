use crate::{api::ManagedTypeApi, types::ManagedBuffer};

pub trait ManagedBufferBuilderImpl<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    fn new_from_slice(slice: &[u8]) -> Self;

    fn into_managed_buffer(self) -> ManagedBuffer<'a, M>;

    fn append_bytes(&mut self, bytes: &[u8]);

    fn append_managed_buffer(&mut self, item: &ManagedBuffer<'a, M>);
}
