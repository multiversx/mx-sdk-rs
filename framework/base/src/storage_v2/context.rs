use core::marker::PhantomData;

use crate::{api, key::DynamicKey};

pub trait StorageContext {
    type ReadAccess: StorageContextRead;
    type WriteAccess: StorageContextWrite;

    unsafe fn unsafe_clone(&self) -> Self;

    fn downcast_read(&self) -> &Self::ReadAccess;

    fn try_downcast_write(&self) -> Option<&Self::WriteAccess>;

    fn subcontext(&self, delta: &str) -> Self;
}

pub trait StorageContextRead: StorageContext {
    fn read_raw(&self) -> String;
}

pub trait StorageContextWrite: StorageContextRead {
    fn write_raw(&self, value: String);
}

/// Layout marker.
///
/// Cannot create instance of this type.
pub enum Layout {}

impl StorageContext for Layout {
    type ReadAccess = NoAccess;
    type WriteAccess = NoAccess;

    unsafe fn unsafe_clone(&self) -> Self {
        unreachable!()
    }

    fn downcast_read(&self) -> &Self::ReadAccess {
        unreachable!()
    }

    fn try_downcast_write(&self) -> Option<&Self::WriteAccess> {
        unreachable!()
    }

    fn subcontext(&self, delta: &str) -> Self {
        unreachable!()
    }
}

pub enum NoAccess {}

impl StorageContext for NoAccess {
    type ReadAccess = NoAccess;
    type WriteAccess = NoAccess;

    unsafe fn unsafe_clone(&self) -> Self {
        unreachable!()
    }

    fn downcast_read(&self) -> &Self::ReadAccess {
        unreachable!()
    }

    fn try_downcast_write(&self) -> Option<&Self::WriteAccess> {
        unreachable!()
    }

    fn subcontext(&self, delta: &str) -> Self {
        unreachable!()
    }
}

impl StorageContextRead for NoAccess {
    fn read_raw(&self) -> String {
        unreachable!()
    }
}

impl StorageContextWrite for NoAccess {
    fn write_raw(&self, value: String) {
        unreachable!()
    }
}

#[derive(Default)]
pub struct SelfRead<'r> {
    key: DynamicKey,
    _phantom: PhantomData<&'r ()>,
}

impl SelfRead<'_> {
    pub fn new(key: DynamicKey) -> Self {
        SelfRead {
            key,
            _phantom: PhantomData,
        }
    }
}

impl StorageContext for SelfRead<'_> {
    type ReadAccess = Self;
    type WriteAccess = NoAccess;

    unsafe fn unsafe_clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            _phantom: PhantomData,
        }
    }

    fn downcast_read(&self) -> &Self::ReadAccess {
        self
    }

    fn try_downcast_write(&self) -> Option<&Self::WriteAccess> {
        None
    }

    fn subcontext(&self, delta: &str) -> Self {
        Self::new(format!("{}{}", &self.key, delta))
    }
}

impl StorageContextRead for SelfRead<'_> {
    fn read_raw(&self) -> String {
        api::get(&self.key)
    }
}

#[derive(Default)]
pub struct SelfWrite<'w> {
    key: DynamicKey,
    _phantom: PhantomData<&'w mut ()>,
}

impl SelfWrite<'_> {
    pub fn new(key: DynamicKey) -> Self {
        SelfWrite {
            key,
            _phantom: PhantomData,
        }
    }
}

impl StorageContext for SelfWrite<'_> {
    type ReadAccess = Self;
    type WriteAccess = Self;

    unsafe fn unsafe_clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            _phantom: PhantomData,
        }
    }

    fn downcast_read(&self) -> &Self::ReadAccess {
        self
    }

    fn try_downcast_write(&self) -> Option<&Self::WriteAccess> {
        Some(self)
    }

    fn subcontext(&self, delta: &str) -> Self {
        Self::new(format!("{}{}", &self.key, delta))
    }
}

impl StorageContextRead for SelfWrite<'_> {
    fn read_raw(&self) -> String {
        api::get(&self.key)
    }
}

impl StorageContextWrite for SelfWrite<'_> {
    fn write_raw(&self, value: String) {
        api::set(self.key.clone(), value);
    }
}
