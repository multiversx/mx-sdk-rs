use core::marker::PhantomData;

use crate::{api::ManagedTypeApi, storage_v2::DynamicKey, types::ManagedBuffer};

pub trait StorageContext<M: ManagedTypeApi> {
    type ReadAccess: StorageContextRead<M>;
    type WriteAccess: StorageContextWrite<M>;

    unsafe fn unsafe_clone(&self) -> Self;

    fn downcast_read(&self) -> &Self::ReadAccess;

    fn try_downcast_write(&self) -> Option<&Self::WriteAccess>;

    fn subcontext(&self, delta: ManagedBuffer<M>) -> Self;
}

pub trait StorageContextRead<M: ManagedTypeApi>: StorageContext<M> {
    fn read_raw(&self) -> ManagedBuffer<M>;
}

pub trait StorageContextWrite<M: ManagedTypeApi>: StorageContextRead<M> {
    fn write_raw(&self, value: ManagedBuffer<M>);
}

/// Layout marker.
///
/// Cannot create instance of this type.
pub enum Layout {}

impl<M> StorageContext<M> for Layout
where
    M: ManagedTypeApi,
{
    type ReadAccess = NoAccess<M>;
    type WriteAccess = NoAccess<M>;

    unsafe fn unsafe_clone(&self) -> Self {
        unreachable!()
    }

    fn downcast_read(&self) -> &Self::ReadAccess {
        unreachable!()
    }

    fn try_downcast_write(&self) -> Option<&Self::WriteAccess> {
        unreachable!()
    }

    fn subcontext(&self, delta: ManagedBuffer<M>) -> Self {
        unreachable!()
    }
}

pub enum NoAccess<M>
where
    M: ManagedTypeApi,
{
    _Phantom(PhantomData<M>),
}

impl<M> StorageContext<M> for NoAccess<M>
where
    M: ManagedTypeApi,
{
    type ReadAccess = NoAccess<M>;
    type WriteAccess = NoAccess<M>;

    unsafe fn unsafe_clone(&self) -> Self {
        unreachable!()
    }

    fn downcast_read(&self) -> &Self::ReadAccess {
        unreachable!()
    }

    fn try_downcast_write(&self) -> Option<&Self::WriteAccess> {
        unreachable!()
    }

    fn subcontext(&self, delta: ManagedBuffer<M>) -> Self {
        unreachable!()
    }
}

impl<M> StorageContextRead<M> for NoAccess<M>
where
    M: ManagedTypeApi,
{
    fn read_raw(&self) -> ManagedBuffer<M> {
        unreachable!()
    }
}

impl<M> StorageContextWrite<M> for NoAccess<M>
where
    M: ManagedTypeApi,
{
    fn write_raw(&self, value: ManagedBuffer<M>) {
        unreachable!()
    }
}

#[derive(Default)]
pub struct SelfRead<'r, M>
where
    M: ManagedTypeApi,
{
    key: DynamicKey<M>,
    _phantom: PhantomData<&'r ()>,
}

impl<M> SelfRead<'_, M>
where
    M: ManagedTypeApi,
{
    pub fn new(key: DynamicKey<M>) -> Self {
        SelfRead {
            key,
            _phantom: PhantomData,
        }
    }
}

impl<M> StorageContext<M> for SelfRead<'_, M>
where
    M: ManagedTypeApi,
{
    type ReadAccess = Self;
    type WriteAccess = NoAccess<M>;

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

    fn subcontext(&self, delta: ManagedBuffer<M>) -> Self {
        Self::new(self.key.clone().concat(delta))
    }
}

impl<M> StorageContextRead<M> for SelfRead<'_, M>
where
    M: ManagedTypeApi,
{
    fn read_raw(&self) -> ManagedBuffer<M> {
        // api::get()

        // api::get(&self.key)
        ManagedBuffer::new()
    }
}

#[derive(Default)]
pub struct SelfWrite<'w, M>
where
    M: ManagedTypeApi,
{
    key: DynamicKey<M>,
    _phantom: PhantomData<&'w mut ()>,
}

impl<M> SelfWrite<'_, M>
where
    M: ManagedTypeApi,
{
    pub fn new(key: DynamicKey<M>) -> Self {
        SelfWrite {
            key,
            _phantom: PhantomData,
        }
    }
}

impl<M> StorageContext<M> for SelfWrite<'_, M>
where
    M: ManagedTypeApi,
{
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

    fn subcontext(&self, delta: ManagedBuffer<M>) -> Self {
        Self::new(self.key.clone().concat(delta))
    }
}

impl<M> StorageContextRead<M> for SelfWrite<'_, M>
where
    M: ManagedTypeApi,
{
    fn read_raw(&self) -> ManagedBuffer<M> {
        // api::get(&self.key)
        ManagedBuffer::new()
    }
}

impl<M> StorageContextWrite<M> for SelfWrite<'_, M>
where
    M: ManagedTypeApi,
{
    fn write_raw(&self, value: ManagedBuffer<M>) {
        // api::set(self.key.clone(), value);
    }
}
