use core::{borrow::Borrow, marker::PhantomData, ops::Deref};

use crate::codec::{
    EncodeErrorHandler, NestedEncode, NestedEncodeOutput, TopEncode, TopEncodeOutput,
};

use crate::{api::ManagedTypeApi, types::ManagedType};
use crate::api::UnsafeClone;

pub(super) enum ValueOrRef<'a, T> {
    Value(T),
    Ref(&'a T),
}

impl<'a, T> Deref for ValueOrRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match *self {
            ValueOrRef::Value(ref val) => val,
            ValueOrRef::Ref(ref_val) => ref_val,
        }
    }
}

impl<'a, T: UnsafeClone> UnsafeClone for ValueOrRef<'a, T> {
    unsafe fn unsafe_clone(&self) -> Self {
        match self {
            ValueOrRef::Value(value) => { ValueOrRef::Value(value.unsafe_clone()) }
            ValueOrRef::Ref(reference) => { ValueOrRef::Ref(reference) }
        }
    }
}

/// A very efficient reference to a managed type, with copy semantics.
///
/// It copies the handle and knows how to deref back.
pub struct ManagedRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    pub(super) _phantom_m: PhantomData<M>,
    pub(super) _phantom_t: PhantomData<&'a T>,
    pub(super) handle: ValueOrRef<'a, T::OwnHandle>,
}

impl<'a, M, T> ManagedRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    pub fn new(value: &'a T) -> Self {
        Self {
            _phantom_m: PhantomData,
            _phantom_t: PhantomData,
            handle: ValueOrRef::Ref(value.get_handle()),
        }
    }

    #[doc(hidden)]
    pub(crate) fn wrap_handle_ref(handle_ref: &'a T::OwnHandle) -> Self {
        Self {
            _phantom_m: PhantomData,
            _phantom_t: PhantomData,
            handle: ValueOrRef::Ref(handle_ref),
        }
    }

    #[doc(hidden)]
    pub(crate) fn wrap_handle(handle: T::OwnHandle) -> Self {
        Self {
            _phantom_m: PhantomData,
            _phantom_t: PhantomData,
            handle: ValueOrRef::Value(handle),
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn get_raw_handle_of_ref(&'a self) -> &'a T::OwnHandle {
        &self.handle
    }
}

impl<'a, M, T> ManagedRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + Clone,
{
    pub fn clone_value(&self) -> T {
        self.deref().clone()
    }
}

impl<'a, M, T> Clone for ManagedRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            _phantom_m: PhantomData,
            _phantom_t: PhantomData,
            handle: unsafe { self.handle.unsafe_clone() }, // Fine thanks to the lifetime 'a which ensures the handle won't be dropped
        }
    }
}

impl<'a, M, T> Deref for ManagedRef<'a, M, T>
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

impl<'a, M, T> Borrow<T> for ManagedRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    #[inline]
    fn borrow(&self) -> &T {
        self.deref()
    }
}

impl<'a, M, T> From<&'a T> for ManagedRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    #[inline]
    fn from(value_ref: &'a T) -> Self {
        Self::new(value_ref)
    }
}

impl<'a, M, T> PartialEq for ManagedRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<'a, M, T> Eq for ManagedRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + PartialEq,
{
}

impl<'a, M, T> TopEncode for ManagedRef<'a, M, T>
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

impl<'a, M, T> NestedEncode for ManagedRef<'a, M, T>
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

impl<'a, M, T> core::fmt::Debug for ManagedRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ManagedRef").field(self.deref()).finish()
    }
}
