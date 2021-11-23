use core::marker::PhantomData;

use crate::{
    api::{InvalidSliceError, ManagedTypeApi},
    types::{ManagedBuffer, ManagedType},
};

use super::LockableStaticBuffer;

pub struct StaticBufferRef<M: ManagedTypeApi> {
    _phantom: PhantomData<M>,
}

impl<M: ManagedTypeApi> StaticBufferRef<M> {
    fn new() -> Self {
        StaticBufferRef {
            _phantom: PhantomData,
        }
    }

    pub fn try_new_from_copy_bytes<F: FnOnce(&mut [u8])>(
        len: usize,
        copy_bytes: F,
    ) -> Option<Self> {
        M::instance().with_lockable_static_buffer(|lsb| {
            if lsb.try_lock_with_copy_bytes(len, copy_bytes) {
                Some(StaticBufferRef::new())
            } else {
                None
            }
        })
    }

    pub fn try_new(bytes: &[u8]) -> Option<Self> {
        Self::try_new_from_copy_bytes(bytes.len(), |dest| dest.copy_from_slice(bytes))
    }

    pub fn try_from_managed_buffer(managed_buffer: &ManagedBuffer<M>) -> Option<Self> {
        if managed_buffer
            .type_manager()
            .mb_overwrite_static_buffer(managed_buffer.get_raw_handle())
        {
            Some(StaticBufferRef::new())
        } else {
            None
        }
    }
}

impl<M: ManagedTypeApi> Drop for StaticBufferRef<M> {
    fn drop(&mut self) {
        M::instance().with_lockable_static_buffer(|lsb| {
            lsb.unlock();
        })
    }
}

impl<M: ManagedTypeApi> StaticBufferRef<M> {
    pub fn len(&self) -> usize {
        M::instance().with_lockable_static_buffer(|lsb| lsb.len())
    }

    pub fn is_empty(&self) -> bool {
        M::instance().with_lockable_static_buffer(|lsb| lsb.is_empty())
    }

    pub fn capacity(&self) -> usize {
        LockableStaticBuffer::capacity()
    }

    pub fn remaining_capacity(&self) -> usize {
        M::instance().with_lockable_static_buffer(|lsb| lsb.remaining_capacity())
    }

    pub fn with_buffer_contents<R, F: FnMut(&[u8]) -> R>(&self, mut f: F) -> R {
        M::instance().with_lockable_static_buffer(|lsb| f(lsb.as_slice()))
    }

    pub fn contents_eq(&self, bytes: &[u8]) -> bool {
        M::instance().with_lockable_static_buffer(|lsb| lsb.as_slice() == bytes)
    }

    pub fn load_slice(
        &self,
        starting_position: usize,
        dest: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        M::instance().with_lockable_static_buffer(|lsb| lsb.load_slice(starting_position, dest))
    }

    pub fn try_extend_from_slice(&mut self, bytes: &[u8]) -> bool {
        self.try_extend_from_copy_bytes(bytes.len(), |dest| dest.copy_from_slice(bytes))
    }

    pub fn try_extend_from_copy_bytes<F: FnOnce(&mut [u8])>(
        &mut self,
        len: usize,
        copy_bytes: F,
    ) -> bool {
        M::instance()
            .with_lockable_static_buffer(|lsb| lsb.try_extend_from_copy_bytes(len, copy_bytes))
    }
}
