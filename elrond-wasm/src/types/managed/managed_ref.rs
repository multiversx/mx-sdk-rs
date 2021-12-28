use core::{marker::PhantomData, ops::Deref};

use elrond_codec::{EncodeError, NestedEncode, NestedEncodeOutput, TopEncode, TopEncodeOutput};

use crate::api::{Handle, ManagedTypeApi};

use super::ManagedType;

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
    pub(super) handle: Handle,
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
            handle: value.get_raw_handle(),
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn get_raw_handle_of_ref(self) -> Handle {
        self.handle
    }
}

impl<'a, M, T> Copy for ManagedRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
}

impl<'a, M, T> Clone for ManagedRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
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
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.deref().top_encode(output)
    }
}

impl<'a, M, T> NestedEncode for ManagedRef<'a, M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + NestedEncode,
{
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.deref().dep_encode(dest)
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
