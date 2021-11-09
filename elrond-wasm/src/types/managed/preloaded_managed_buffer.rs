use crate::{
    api::{InvalidSliceError, ManagedTypeApi},
    types::BoxedBytes,
};

use super::{ManagedBuffer, ManagedType};

pub(crate) struct PreloadedManagedBuffer<M>
where
    M: ManagedTypeApi,
{
    pub managed_buffer: ManagedBuffer<M>,
    pub buffer_len: usize,
    local_buffer: BoxedBytes,
}

impl<M> PreloadedManagedBuffer<M>
where
    M: ManagedTypeApi,
{
    pub fn new(managed_buffer: ManagedBuffer<M>) -> Self {
        let buffer_len = managed_buffer.len();
        let mut local_buffer = BoxedBytes::zeros(buffer_len);
        if managed_buffer
            .load_slice(0, local_buffer.as_mut_slice())
            .is_err()
        {
            managed_buffer.api.signal_error(b"preload buffer error");
        }
        Self {
            managed_buffer,
            buffer_len,
            local_buffer,
        }
    }

    fn get_slice<'a>(
        &self,
        buffer: &'a [u8],
        starting_position: usize,
        slice_len: usize,
    ) -> Result<&'a [u8], InvalidSliceError> {
        if starting_position + slice_len <= self.buffer_len {
            Ok(&buffer[starting_position..starting_position + slice_len])
        } else {
            Err(InvalidSliceError)
        }
    }

    #[inline]
    pub fn load_slice(
        &self,
        starting_position: usize,
        dest_slice: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        let slice = self.get_slice(
            self.local_buffer.as_slice(),
            starting_position,
            dest_slice.len(),
        )?;
        dest_slice.copy_from_slice(slice);
        Ok(())
    }

    pub fn copy_slice(
        &self,
        starting_position: usize,
        slice_len: usize,
    ) -> Option<ManagedBuffer<M>> {
        let type_manager = self.managed_buffer.type_manager();
        let slice = self
            .get_slice(self.local_buffer.as_slice(), starting_position, slice_len)
            .ok()?;
        Some(ManagedBuffer::new_from_bytes(type_manager, slice))
    }

    pub fn type_manager(&self) -> M {
        self.managed_buffer.type_manager()
    }
}
