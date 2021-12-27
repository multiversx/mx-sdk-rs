use core::{marker::PhantomData, ops::Deref};

use alloc::boxed::Box;
use elrond_codec::{
    DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, TryStaticCast,
};

use crate::api::{Handle, ManagedTypeApi};

use super::{ManagedRef, ManagedType};

/// Encapsulates the same handle as the base managed type, but restricts operations to only readonly ones.
/// This makes it safe to be copied as-is, since any number of immutable references are allowed.
///
/// It can be thought of as a "smart" pointer to immutable data.
///
/// Also note that unlike the `ManagedReadonly`, the data is owned.
pub struct ManagedReadonly<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    _phantom_m: PhantomData<M>,
    _phantom_t: PhantomData<T>,
    handle: Handle,
}

impl<M, T> ManagedReadonly<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    pub fn new(value: T) -> Self {
        Self {
            _phantom_m: PhantomData,
            _phantom_t: PhantomData,
            handle: value.get_raw_handle(),
        }
    }

    #[inline]
    pub fn as_content_ref(&self) -> ManagedRef<'_, M, T> {
        ManagedRef {
            _phantom_m: PhantomData,
            _phantom_t: PhantomData,
            handle: self.handle,
        }
    }
}

impl<M, T> Copy for ManagedReadonly<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
}

impl<M, T> Clone for ManagedReadonly<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<M, T> Deref for ManagedReadonly<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        Self::Target::transmute_from_handle_ref(&self.handle)
    }
}

impl<M, T> PartialEq for ManagedReadonly<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<M, T> Eq for ManagedReadonly<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + PartialEq,
{
}

impl<M, T> ManagedType<M> for ManagedReadonly<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    fn from_raw_handle(handle: Handle) -> Self {
        Self::new(T::from_raw_handle(handle))
    }

    fn get_raw_handle(&self) -> Handle {
        self.handle
    }

    fn transmute_from_handle_ref(handle_ref: &Handle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M, T> From<T> for ManagedReadonly<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<M: ManagedTypeApi, T: ManagedType<M> + 'static> TryStaticCast for ManagedReadonly<M, T> {}

impl<M, T> TopEncode for ManagedReadonly<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + TopEncode,
{
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.deref().top_encode(output)
    }
}

impl<M, T> NestedEncode for ManagedReadonly<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + NestedEncode,
{
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.deref().dep_encode(dest)
    }
}

impl<M, T> TopDecode for ManagedReadonly<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + TopDecode,
{
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        T::top_decode(input).map(|value| value.into())
    }
}

impl<M, T> NestedDecode for ManagedReadonly<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + NestedDecode,
{
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        T::dep_decode(input).map(|value| value.into())
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        T::dep_decode_or_exit(input, c, exit).into()
    }
}

impl<M, T> TopDecodeInput for ManagedReadonly<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + TopDecodeInput,
{
    type NestedBuffer = T::NestedBuffer;

    fn byte_len(&self) -> usize {
        self.deref().byte_len()
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        T::from_raw_handle(self.handle).into_boxed_slice_u8()
    }

    fn into_u64(self) -> u64 {
        T::from_raw_handle(self.handle).into_u64()
    }

    fn into_specialized<TSC, F>(self, else_deser: F) -> Result<TSC, DecodeError>
    where
        TSC: TryStaticCast,
        F: FnOnce(Self) -> Result<TSC, DecodeError>,
    {
        T::from_raw_handle(self.handle).into_specialized(|value| else_deser(value.into()))
    }

    fn into_nested_buffer(self) -> Self::NestedBuffer {
        T::from_raw_handle(self.handle).into_nested_buffer()
    }
}

impl<M, T> core::fmt::Debug for ManagedReadonly<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ManagedReadonly")
            .field(self.deref())
            .finish()
    }
}
