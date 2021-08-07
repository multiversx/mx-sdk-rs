use crate::{
    api::{Handle, ManagedTypeApi},
    types::BoxedBytes,
};

/// A byte buffer managed by an external API.
pub struct ManagedBuffer<M: ManagedTypeApi> {
    pub(crate) handle: Handle,
    pub(crate) api: M,
}

impl<M: ManagedTypeApi> ManagedBuffer<M> {
    pub fn new_empty(api: M) -> Self {
        ManagedBuffer {
            handle: api.new_empty(),
            api: api.clone(),
        }
    }

    pub fn new_from_bytes(api: M, bytes: &[u8]) -> Self {
        ManagedBuffer {
            handle: api.new_from_bytes(bytes),
            api: api.clone(),
        }
    }

    pub(crate) fn new_from_raw_handle(api: M, handle: Handle) -> Self {
        ManagedBuffer {
            handle,
            api,
        }
    }

    pub fn len(&self) -> usize {
        self.api.len(self.handle)
    }

    pub fn overwrite(&self, value: &[u8]) {
        self.api.overwrite(self.handle, value);
    }

    pub fn append_bytes(&mut self, slice: &[u8]) {
        self.api.extend_from_slice(self.handle, slice);
    }

    pub fn append(&mut self, other: &ManagedBuffer<M>) {
        // TODO: Arwen specialized API
        self.api.extend_from_slice(self.handle, other.to_boxed_bytes().as_slice());
    }

    pub fn to_boxed_bytes(&self) -> BoxedBytes {
        self.api.to_boxed_bytes(self.handle)
    }
}
