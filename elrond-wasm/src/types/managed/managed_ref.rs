use core::{marker::PhantomData, ops::Deref};

use crate::api::{Handle, ManagedTypeApi};

use super::ManagedType;

pub struct ManagedRef<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    _phantom: PhantomData<M>,
    value: T,
}

impl<M, T> ManagedRef<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    pub fn new(value: T) -> Self {
        Self {
            _phantom: PhantomData,
            value,
        }
    }
}

impl<M, T> Deref for ManagedRef<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<M, T> ManagedType<M> for ManagedRef<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    fn from_raw_handle(api: M, raw_handle: Handle) -> Self {
        Self::new(T::from_raw_handle(api, raw_handle))
    }

    fn get_raw_handle(&self) -> Handle {
        self.value.get_raw_handle()
    }

    fn type_manager(&self) -> M {
        self.value.type_manager()
    }
}

impl<M, T> Clone for ManagedRef<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    fn clone(&self) -> Self {
        Self::from_raw_handle(self.type_manager(), self.get_raw_handle())
    }
}

impl<M, T> From<T> for ManagedRef<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<'a, M, T> From<&T> for ManagedRef<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    fn from(value: &T) -> Self {
        Self::new(T::from_raw_handle(
            value.type_manager(),
            value.get_raw_handle(),
        ))
    }
}
