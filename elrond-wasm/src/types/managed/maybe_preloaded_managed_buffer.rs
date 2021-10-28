use crate::api::{InvalidSliceError, ManagedTypeApi};

use super::{ManagedBuffer, ManagedType};

const BUFFER_SIZE: usize = 60000;

pub(crate) struct MaybePreloadedManagedBuffer<M>
where
    M: ManagedTypeApi,
{
    pub managed_buffer: ManagedBuffer<M>,
    pub buffer_len: usize,
    local_buffer: Option<[u8; BUFFER_SIZE]>,
}

impl<M> MaybePreloadedManagedBuffer<M>
where
    M: ManagedTypeApi,
{
    pub fn new(managed_buffer: ManagedBuffer<M>) -> Self {
        let buffer_len = managed_buffer.len();
        let local_buffer = if buffer_len <= BUFFER_SIZE {
            let mut new_buffer = [0; BUFFER_SIZE];
            if managed_buffer.load_slice(0, &mut new_buffer).is_ok() {
                Some(new_buffer)
            } else {
                None
            }
        } else {
            None
        };
        Self {
            managed_buffer,
            buffer_len,
            local_buffer,
        }
    }

    fn get_slice<'a>(
        &self,
        buffer: &'a [u8; BUFFER_SIZE],
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
        if let Some(buffer) = self.local_buffer {
            let slice = self.get_slice(&buffer, starting_position, dest_slice.len())?;
            dest_slice.copy_from_slice(slice);
            Ok(())
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
        if let Some(buffer) = self.local_buffer {
            let type_manager = self.managed_buffer.type_manager();
            let slice = self.get_slice(&buffer, starting_position, slice_len).ok()?;
            Some(ManagedBuffer::new_from_bytes(type_manager, slice))
        } else {
            self.managed_buffer.copy_slice(starting_position, slice_len)
        }
    }

    pub fn type_manager(&self) -> M {
        self.managed_buffer.type_manager()
    }
}
