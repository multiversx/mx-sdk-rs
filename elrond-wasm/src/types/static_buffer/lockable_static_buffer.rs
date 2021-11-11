use crate::api::InvalidSliceError;

const BUFFER_SIZE: usize = 10000;

#[derive(Debug)]
pub struct LockableStaticBuffer {
    pub buffer: [u8; BUFFER_SIZE],
    pub locked: bool,
    pub used_size: usize,
}

impl LockableStaticBuffer {
    pub const fn new() -> Self {
        LockableStaticBuffer {
            buffer: [0u8; BUFFER_SIZE],
            locked: false,
            used_size: 0,
        }
    }

    pub const fn new_locked() -> Self {
        LockableStaticBuffer {
            buffer: [0u8; BUFFER_SIZE],
            locked: true,
            used_size: 0,
        }
    }

    /// Returns true if locked successfully.
    pub fn try_lock_with_copy_bytes<F: FnOnce(&mut [u8])>(
        &mut self,
        len: usize,
        copy_bytes: F,
    ) -> bool {
        if self.locked || len > BUFFER_SIZE {
            false
        } else {
            self.locked = true;
            self.used_size = len;
            copy_bytes(&mut self.buffer[..len]);
            true
        }
    }

    pub fn unlock(&mut self) {
        self.locked = false;
        self.used_size = 0;
    }

    pub fn len(&self) -> usize {
        self.used_size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn capacity() -> usize {
        BUFFER_SIZE
    }

    pub fn remaining_capacity(&self) -> usize {
        BUFFER_SIZE - self.used_size
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.buffer[..self.used_size]
    }

    pub fn load_slice(
        &self,
        starting_position: usize,
        dest: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        if starting_position + dest.len() <= self.used_size {
            dest.copy_from_slice(&self.buffer[starting_position..starting_position + dest.len()]);
            Ok(())
        } else {
            Err(InvalidSliceError)
        }
    }

    pub fn try_extend_from_slice(&mut self, bytes: &[u8]) -> bool {
        self.try_extend_from_copy_bytes(bytes.len(), |dest| dest.copy_from_slice(bytes))
    }

    pub fn try_extend_from_copy_bytes<F: FnOnce(&mut [u8])>(
        &mut self,
        len: usize,
        copy_bytes: F,
    ) -> bool {
        if len > self.remaining_capacity() {
            return false;
        }

        let new_size = self.used_size + len;
        copy_bytes(&mut self.buffer[self.used_size..new_size]);
        self.used_size = new_size;
        true
    }
}
