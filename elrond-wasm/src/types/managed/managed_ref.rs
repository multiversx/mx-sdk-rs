use core::{marker::PhantomData, ops::Deref};

use alloc::boxed::Box;
use elrond_codec::{
    DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, TryStaticCast,
};

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
    fn from_raw_handle(handle: Handle) -> Self {
        Self::new(T::from_raw_handle(handle))
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
        Self::from_raw_handle(self.get_raw_handle())
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
        Self::new(T::from_raw_handle(value.get_raw_handle()))
    }
}

pub trait AsManagedRef<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    fn as_managed_ref(&self) -> ManagedRef<M, T>;
}

impl<M, T> AsManagedRef<M, T> for T
where
    M: ManagedTypeApi,
    T: ManagedType<M>,
{
    fn as_managed_ref(&self) -> ManagedRef<M, T> {
        self.into()
    }
}

impl<M: ManagedTypeApi, T: ManagedType<M> + 'static> TryStaticCast for ManagedRef<M, T> {}

impl<M, T> TopEncode for ManagedRef<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + TopEncode,
{
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.value.top_encode(output)
    }
}

impl<M, T> NestedEncode for ManagedRef<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + NestedEncode,
{
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.value.dep_encode(dest)
    }
}

impl<M, T> TopDecode for ManagedRef<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + TopDecode,
{
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        T::top_decode(input).map(|value| value.into())
    }
}

impl<M, T> NestedDecode for ManagedRef<M, T>
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

impl<M, T> TopDecodeInput for ManagedRef<M, T>
where
    M: ManagedTypeApi,
    T: ManagedType<M> + TopDecodeInput,
{
    type NestedBuffer = T::NestedBuffer;

    fn byte_len(&self) -> usize {
        self.value.byte_len()
    }

    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        self.value.into_boxed_slice_u8()
    }

    fn into_u64(self) -> u64 {
        self.value.into_u64()
    }

    fn into_specialized<TSC, F>(self, else_deser: F) -> Result<TSC, DecodeError>
    where
        TSC: TryStaticCast,
        F: FnOnce(Self) -> Result<TSC, DecodeError>,
    {
        self.value
            .into_specialized(|value| else_deser(value.into()))
    }

    fn into_nested_buffer(self) -> Self::NestedBuffer {
        self.value.into_nested_buffer()
    }
}
