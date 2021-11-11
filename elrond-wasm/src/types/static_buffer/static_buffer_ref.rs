use crate::{
    api::{InvalidSliceError, ManagedTypeApi},
    types::{ManagedBuffer, ManagedType},
};

use super::LockableStaticBuffer;

pub struct StaticBufferRef<M: ManagedTypeApi> {
    api: M,
}

impl<M: ManagedTypeApi> StaticBufferRef<M> {
    pub fn try_new_from_copy_bytes<F: FnOnce(&mut [u8])>(
        api: M,
        len: usize,
        copy_bytes: F,
    ) -> Option<Self> {
        api.clone().with_lockable_static_buffer(|lsb| {
            if lsb.try_lock_with_copy_bytes(len, copy_bytes) {
                Some(StaticBufferRef { api })
            } else {
                None
            }
        })
    }

    pub fn try_new(api: M, bytes: &[u8]) -> Option<Self> {
        Self::try_new_from_copy_bytes(api, bytes.len(), |dest| dest.copy_from_slice(bytes))
    }

    pub fn try_from_managed_buffer(managed_buffer: &ManagedBuffer<M>) -> Option<Self> {
        if managed_buffer
            .type_manager()
            .mb_overwrite_static_buffer(managed_buffer.get_raw_handle())
        {
            Some(StaticBufferRef {
                api: managed_buffer.type_manager(),
            })
        } else {
            None
        }
    }
}

impl<M: ManagedTypeApi> Drop for StaticBufferRef<M> {
    fn drop(&mut self) {
        self.api.with_lockable_static_buffer(|lsb| {
            lsb.unlock();
        })
    }
}

impl<M: ManagedTypeApi> StaticBufferRef<M> {
    pub fn len(&self) -> usize {
        self.api.with_lockable_static_buffer(|lsb| lsb.len())
    }

    pub fn is_empty(&self) -> bool {
        self.api.with_lockable_static_buffer(|lsb| lsb.is_empty())
    }

    pub fn capacity(&self) -> usize {
        LockableStaticBuffer::capacity()
    }

    pub fn remaining_capacity(&self) -> usize {
        self.api
            .with_lockable_static_buffer(|lsb| lsb.remaining_capacity())
    }

    pub fn with_buffer_contents<R, F: FnMut(&[u8]) -> R>(&self, mut f: F) -> R {
        self.api
            .with_lockable_static_buffer(|lsb| f(lsb.as_slice()))
    }

    pub fn contents_eq(&self, bytes: &[u8]) -> bool {
        self.api
            .with_lockable_static_buffer(|lsb| lsb.as_slice() == bytes)
    }

    pub fn load_slice(
        &self,
        starting_position: usize,
        dest: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        self.api
            .with_lockable_static_buffer(|lsb| lsb.load_slice(starting_position, dest))
    }

    pub fn try_extend_from_slice(&mut self, bytes: &[u8]) -> bool {
        self.try_extend_from_copy_bytes(bytes.len(), |dest| dest.copy_from_slice(bytes))
    }

    pub fn try_extend_from_copy_bytes<F: FnOnce(&mut [u8])>(
        &mut self,
        len: usize,
        copy_bytes: F,
    ) -> bool {
        self.api
            .with_lockable_static_buffer(|lsb| lsb.try_extend_from_copy_bytes(len, copy_bytes))
    }
}
