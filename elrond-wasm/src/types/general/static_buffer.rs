use crate::api::InvalidSliceError;

const BUFFER_SIZE: usize = 10000;

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
            if LOCKED || len > BUFFER_SIZE {
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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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

impl Drop for StaticBufferRef {
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

    #[test]
    fn test_lock_unlock() {
        {
            let s = StaticBufferRef::try_new(b"first").unwrap();
            assert_eq!(s.as_slice(), b"first");
            // should unlock here
        }

        let s = StaticBufferRef::try_new(b"another").unwrap();
        assert!(StaticBufferRef::try_new(b"no, locked").is_none());
        assert_eq!(s.as_slice(), b"another");
    }

    #[test]
    fn test_extend_past_buffer_limits() {
        let mut s = StaticBufferRef::try_new(&[]).unwrap();
        assert!(s.try_extend_from_slice(&[22; BUFFER_SIZE - 1]));
        assert!(s.try_extend_from_slice(&[33; 1]));
        assert!(!s.try_extend_from_slice(&[44; 1]));
    }

    fn new_should_fail() {
        let buffer_option = StaticBufferRef::try_new(b"test");
        assert!(buffer_option.is_none());
    }

    fn new_should_succeed() {
        let buffer_option = StaticBufferRef::try_new(b"test");
        assert!(buffer_option.is_some());
    }

    #[test]
    fn test_lock_2() {
        let buffer_option = StaticBufferRef::try_new(b"locking_test");
        new_should_fail();
        assert!(buffer_option.is_some());
        let s1_buffer = buffer_option.unwrap();
        new_should_fail();
        assert_eq!(s1_buffer.as_slice(), b"locking_test");
        new_should_fail();
        drop(s1_buffer);
        new_should_succeed();
        new_should_succeed();
    }
}
