use core::marker::PhantomData;

use crate::{
    api::{
        ErrorApi, ManagedTypeApi, StorageReadApi, StorageReadApiImpl, StorageWriteApi,
        StorageWriteApiImpl as _,
    },
    storage::StorageKey,
    storage_v2::DynamicKey,
    types::{ManagedBuffer, ManagedType},
};

#[allow(dead_code)]
pub trait StorageContext<A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    type ReadAccess: StorageContextRead<A>;
    type WriteAccess: StorageContextWrite<A>;

    unsafe fn unsafe_clone(&self) -> Self;

    fn downcast_read(&self) -> &Self::ReadAccess;

    fn try_downcast_write(&self) -> Option<&Self::WriteAccess>;

    fn subcontext(&self, delta: StorageKey<A>) -> Self;
}

#[allow(dead_code)]
pub trait StorageContextRead<R>: StorageContext<R>
where
    R: ManagedTypeApi + ErrorApi + 'static,
{
    fn read_raw(&self) -> StorageKey<R>;
}

#[allow(dead_code)]
pub trait StorageContextWrite<W>: StorageContextRead<W>
where
    W: ManagedTypeApi + ErrorApi + 'static,
{
    fn write_raw(&self, value: ManagedBuffer<W>);
}

/// Layout marker.
///
/// Cannot create instance of this type.
#[allow(dead_code)]
pub enum Layout {}

impl<A> StorageContext<A> for Layout
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    type ReadAccess = NoAccess<A>;
    type WriteAccess = NoAccess<A>;

    unsafe fn unsafe_clone(&self) -> Self {
        unreachable!()
    }

    fn downcast_read(&self) -> &Self::ReadAccess {
        unreachable!()
    }

    fn try_downcast_write(&self) -> Option<&Self::WriteAccess> {
        unreachable!()
    }

    fn subcontext(&self, _delta: StorageKey<A>) -> Self {
        unreachable!()
    }
}

pub enum NoAccess<A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    _Phantom(PhantomData<A>),
}

impl<A> StorageContext<A> for NoAccess<A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    type ReadAccess = NoAccess<A>;
    type WriteAccess = NoAccess<A>;

    unsafe fn unsafe_clone(&self) -> Self {
        unreachable!()
    }

    fn downcast_read(&self) -> &Self::ReadAccess {
        unreachable!()
    }

    fn try_downcast_write(&self) -> Option<&Self::WriteAccess> {
        unreachable!()
    }

    fn subcontext(&self, _delta: StorageKey<A>) -> Self {
        unreachable!()
    }
}

impl<A> StorageContextRead<A> for NoAccess<A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    fn read_raw(&self) -> StorageKey<A> {
        unreachable!()
    }
}

impl<A> StorageContextWrite<A> for NoAccess<A>
where
    A: ManagedTypeApi + ErrorApi + 'static,
{
    fn write_raw(&self, _value: ManagedBuffer<A>) {
        unreachable!()
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct SelfRead<'r, M>
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    key: DynamicKey<M>,
    _phantom: PhantomData<&'r ()>,
}

#[allow(dead_code)]
impl<M> SelfRead<'_, M>
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    pub fn new(key: DynamicKey<M>) -> Self {
        SelfRead {
            key,
            _phantom: PhantomData,
        }
    }
}

impl<A> StorageContext<A> for SelfRead<'_, A>
where
    A: StorageReadApi + ManagedTypeApi + ErrorApi + 'static,
{
    type ReadAccess = Self;
    type WriteAccess = NoAccess<A>;

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

    fn subcontext(&self, delta: StorageKey<A>) -> Self {
        let mut subcontext_key = self.key.clone();
        subcontext_key.append_managed_buffer(&delta.buffer);
        Self::new(subcontext_key)
    }
}

impl<A> StorageContextRead<A> for SelfRead<'_, A>
where
    A: StorageReadApi + ManagedTypeApi + ErrorApi + 'static,
{
    fn read_raw(&self) -> StorageKey<A> {
        unsafe {
            let result = ManagedBuffer::new_uninit();
            A::storage_read_api_impl()
                .storage_load_managed_buffer_raw(self.key.get_handle(), result.get_handle());
            StorageKey::from(result)
        }
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct SelfWrite<'w, M>
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    key: DynamicKey<M>,
    _phantom: PhantomData<&'w mut ()>,
}

#[allow(dead_code)]
impl<M> SelfWrite<'_, M>
where
    M: ManagedTypeApi + ErrorApi + 'static,
{
    pub fn new(key: DynamicKey<M>) -> Self {
        SelfWrite {
            key,
            _phantom: PhantomData,
        }
    }
}

impl<A> StorageContext<A> for SelfWrite<'_, A>
where
    A: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + 'static,
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

    fn subcontext(&self, delta: StorageKey<A>) -> Self {
        let mut subcontext_key = self.key.clone();
        subcontext_key.append_managed_buffer(&delta.buffer);
        Self::new(subcontext_key)
    }
}

impl<A> StorageContextRead<A> for SelfWrite<'_, A>
where
    A: StorageWriteApi + StorageReadApi + ManagedTypeApi + ErrorApi + 'static,
{
    fn read_raw(&self) -> StorageKey<A> {
        unsafe {
            let result = ManagedBuffer::new_uninit();
            A::storage_read_api_impl()
                .storage_load_managed_buffer_raw(self.key.get_handle(), result.get_handle());
            StorageKey::from(result)
        }
    }
}

impl<A> StorageContextWrite<A> for SelfWrite<'_, A>
where
    A: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + 'static,
{
    fn write_raw(&self, value: ManagedBuffer<A>) {
        A::storage_write_api_impl()
            .storage_store_managed_buffer_raw(self.key.get_handle(), value.handle.clone());
    }
}
