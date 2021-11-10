use crate::api::InvalidSliceError;

const BUFFER_SIZE: usize = 1000;

static mut BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

static mut LOCKED: bool = false;
static mut USED_SIZE: usize = 0;

pub struct StaticBufferRef;

impl StaticBufferRef {
    pub fn try_new_from_copy_bytes<F: FnOnce(&mut [u8])>(
        len: usize,
        copy_bytes: F,
    ) -> Option<Self> {
        unsafe {
            if LOCKED {
                None
            } else if len > BUFFER_SIZE {
                None
            } else {
                LOCKED = true;
                USED_SIZE = len;
                copy_bytes(&mut BUFFER[..len]);
                Some(StaticBufferRef)
            }
        }
    }

    pub fn try_new(bytes: &[u8]) -> Option<Self> {
        Self::try_new_from_copy_bytes(bytes.len(), |dest| dest.copy_from_slice(bytes))
    }

    pub fn len(&self) -> usize {
        unsafe { USED_SIZE }
    }

    pub fn capacity(&self) -> usize {
        BUFFER_SIZE
    }

    pub fn remaining_capacity(&self) -> usize {
        unsafe { BUFFER_SIZE - USED_SIZE }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { &BUFFER[..USED_SIZE] }
    }

    pub fn load_slice(
        &self,
        starting_position: usize,
        dest: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        unsafe {
            if starting_position + dest.len() <= USED_SIZE {
                dest.copy_from_slice(&BUFFER[starting_position..starting_position + dest.len()]);
                Ok(())
            } else {
                Err(InvalidSliceError)
            }
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

        unsafe {
            let new_size = USED_SIZE + len;
            copy_bytes(&mut BUFFER[USED_SIZE..new_size]);
            USED_SIZE = new_size;
        }
        true
    }
}

pub trait Drop {
    fn drop(&mut self) {
        unsafe {
            LOCKED = false;
            USED_SIZE = 0;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_extend_from_slice() {
        let mut s = StaticBufferRef::try_new(b"z").unwrap();
        assert!(s.try_extend_from_slice(b"abc"));
        assert!(s.try_extend_from_slice(b"def"));
        assert_eq!(s.as_slice(), b"zabcdef");
    }
}
