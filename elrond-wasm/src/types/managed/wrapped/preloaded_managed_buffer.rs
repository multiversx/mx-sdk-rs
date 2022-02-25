use crate::{
    api::{InvalidSliceError, ManagedTypeApi},
    types::{ManagedBuffer, StaticBufferRef},
};

pub(crate) struct PreloadedManagedBuffer<M>
where
    M: ManagedTypeApi,
{
    pub managed_buffer: ManagedBuffer<M>,
    pub buffer_len: usize,
    static_cache: Option<StaticBufferRef<M>>,
}

impl<M> PreloadedManagedBuffer<M>
where
    M: ManagedTypeApi,
{
    pub fn new(managed_buffer: ManagedBuffer<M>) -> Self {
        let buffer_len = managed_buffer.len();
        Self {
            managed_buffer,
            buffer_len,
            static_cache: None,
        }
    }

    fn try_load_static_cache_if_necessary(&mut self) {
        if self.static_cache.is_some() {
            return;
        }

        self.static_cache =
            StaticBufferRef::try_new_from_copy_bytes(self.managed_buffer.len(), |dest_slice| {
                let _ = self.managed_buffer.load_slice(0, dest_slice);
            });
    }

    pub fn load_slice(
        &mut self,
        starting_position: usize,
        dest_slice: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        self.try_load_static_cache_if_necessary();
        if let Some(static_cache) = &self.static_cache {
            static_cache.load_slice(starting_position, dest_slice)
        } else {
            self.managed_buffer
                .load_slice(starting_position, dest_slice)
        }
    }

    pub fn copy_slice(
        &self,
        starting_position: usize,
        slice_len: usize,
    ) -> Option<ManagedBuffer<M>> {
        self.managed_buffer.copy_slice(starting_position, slice_len)
    }
}
