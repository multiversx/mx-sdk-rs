use core::ops::DerefMut;
use core::{borrow::Borrow, marker::PhantomData, ops::Deref};

use crate::codec::{
    EncodeErrorHandler, NestedEncode, NestedEncodeOutput, TopEncode, TopEncodeOutput,
};

use crate::{api::ManagedTypeApi, types::ManagedType};

/// A very efficient mutable reference to a managed type.
///
/// It can be dereferenced mutably (DerefMut).
pub struct ManagedRefMut<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    pub(super) _phantom_m: PhantomData<M>,
    pub(super) _phantom_t: PhantomData<&'a mut T>,
    pub(super) handle: T::OwnHandle,
}

impl<'a, M, T> ManagedRefMut<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    pub fn new(value: &'a mut T) -> Self {
        Self {
            _phantom_m: PhantomData,
            _phantom_t: PhantomData,
            handle: value.get_handle(),
        }
    }

    /// Will completely disregard lifetimes, use with care.
    #[doc(hidden)]
    pub(crate) unsafe fn wrap_handle(handle: T::OwnHandle) -> Self {
        Self {
            _phantom_m: PhantomData,
            _phantom_t: PhantomData,
            handle,
        }
    }
}

impl<'a, M, T> ManagedRefMut<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + Clone,
{
    /// Syntactic sugar for dereferencing and cloning the object.
    pub fn clone_value(&self) -> T {
        self.deref().clone()
    }
}

impl<'a, M, T> Clone for ManagedRefMut<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            _phantom_m: PhantomData,
            _phantom_t: PhantomData,
            handle: self.handle.clone(),
        }
    }
}

impl<'a, M, T> Deref for ManagedRefMut<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        Self::Target::transmute_from_handle_ref(&self.handle)
    }
}

impl<'a, M, T> DerefMut for ManagedRefMut<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        Self::Target::transmute_from_handle_ref_mut(&mut self.handle)
    }
}

impl<'a, M, T> Borrow<T> for ManagedRefMut<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    #[inline]
    fn borrow(&self) -> &T {
        self.deref()
    }
}

impl<'a, M, T> From<&'a mut T> for ManagedRefMut<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    #[inline]
    fn from(value_ref: &'a mut T) -> Self {
        Self::new(value_ref)
    }
}

impl<'a, M, T> PartialEq for ManagedRefMut<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<'a, M, T> Eq for ManagedRefMut<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + PartialEq,
{
}

impl<'a, M, T> TopEncode for ManagedRefMut<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + TopEncode,
{
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.deref().top_encode_or_handle_err(output, h)
    }
}

impl<'a, M, T> NestedEncode for ManagedRefMut<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + NestedEncode,
{
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.deref().dep_encode_or_handle_err(dest, h)
    }
}

impl<'a, M, T> core::fmt::Debug for ManagedRefMut<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ManagedRefMut").field(self.deref()).finish()
    }
}
