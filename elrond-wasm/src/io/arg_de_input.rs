use crate::{
    api::{EndpointArgumentApi, ManagedTypeApi},
    types::{BigInt, BigUint, ManagedBuffer, ManagedBufferNestedDecodeInput, ManagedType},
    Box,
};
use elrond_codec::{try_execute_then_cast, DecodeError, TopDecodeInput, TryStaticCast};

/// Adapter from the API to the TopDecodeInput trait.
/// Allows objects to be deserialized directly from the API as arguments.
///
/// Of course the implementation provides shortcut deserialization computation paths directly from API:
/// into_u64, into_i64, ...
///
/// This is a performance-critical struct.
/// Since the wasm EndpointArgumentApi (VmApiImpl) is zero-size,
/// it means that this structures translates to a single glorified i32 in wasm.
pub struct ArgDecodeInput<AA>
where
    AA: ManagedTypeApi + EndpointArgumentApi,
{
    api: AA,
    arg_index: i32,
}

impl<AA> ArgDecodeInput<AA>
where
    AA: ManagedTypeApi + EndpointArgumentApi,
{
    #[inline]
    pub fn new(api: AA, arg_index: i32) -> Self {
        ArgDecodeInput { api, arg_index }
    }

    #[inline]
    fn to_managed_buffer(&self) -> ManagedBuffer<AA> {
        let mbuf_handle = self.api.get_argument_managed_buffer_raw(self.arg_index);
        ManagedBuffer::from_raw_handle(mbuf_handle)
    }

    #[inline]
    fn to_big_int(&self) -> BigInt<AA> {
        let bi_handle = self.api.get_argument_big_int_raw(self.arg_index);
        BigInt::from_raw_handle(bi_handle)
    }

    #[inline]
    fn to_big_uint(&self) -> BigUint<AA> {
        let bi_handle = self.api.get_argument_big_uint_raw(self.arg_index);
        BigUint::from_raw_handle(bi_handle)
    }
}

impl<AA> TopDecodeInput for ArgDecodeInput<AA>
where
    AA: ManagedTypeApi + EndpointArgumentApi,
{
    type NestedBuffer = ManagedBufferNestedDecodeInput<AA>;

    #[inline]
    fn byte_len(&self) -> usize {
        self.api.get_argument_len(self.arg_index)
    }

    #[inline]
    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        self.api.get_argument_boxed_bytes(self.arg_index).into_box()
    }

    #[inline]
    fn into_u64(self) -> u64 {
        self.api.get_argument_u64(self.arg_index)
    }

    #[inline]
    fn into_i64(self) -> i64 {
        self.api.get_argument_i64(self.arg_index)
    }

    #[inline]
    fn into_specialized<T, F>(self, else_deser: F) -> Result<T, DecodeError>
    where
        T: TryStaticCast,
        F: FnOnce(Self) -> Result<T, DecodeError>,
    {
        if let Some(result) = try_execute_then_cast(|| self.to_managed_buffer()) {
            Ok(result)
        } else if let Some(result) = try_execute_then_cast(|| self.to_big_uint()) {
            Ok(result)
        } else if let Some(result) = try_execute_then_cast(|| self.to_big_int()) {
            Ok(result)
        } else {
            else_deser(self)
        }
    }

    #[inline]
    fn into_nested_buffer(self) -> Self::NestedBuffer {
        ManagedBufferNestedDecodeInput::new(self.to_managed_buffer())
    }
}
