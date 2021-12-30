use core::marker::PhantomData;

use super::{ManagedBuffer, ManagedType};
use crate::{
    api::{BigIntApi, Handle, ManagedBufferApi, ManagedTypeApi, ManagedTypeApiImpl},
    hex_util::encode_bytes_as_hex,
    types::BoxedBytes,
};
use alloc::string::String;
use elrond_codec::{
    DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, TryStaticCast,
};

#[repr(transparent)]
pub struct BigUint<M: ManagedTypeApi> {
    pub(crate) handle: Handle,
    _phantom: PhantomData<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for BigUint<M> {
    #[doc(hidden)]
    fn from_raw_handle(handle: Handle) -> Self {
        BigUint {
            handle,
            _phantom: PhantomData,
        }
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.handle
    }

    fn transmute_from_handle_ref(handle_ref: &Handle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for BigUint<M> {
    #[inline]
    fn from(item: ManagedBuffer<M>) -> Self {
        BigUint::from_bytes_be_buffer(&item)
    }
}

impl<M: ManagedTypeApi> From<&ManagedBuffer<M>> for BigUint<M> {
    #[inline]
    fn from(item: &ManagedBuffer<M>) -> Self {
        BigUint::from_bytes_be_buffer(item)
    }
}

macro_rules! big_uint_conv_num {
    ($num_ty:ty) => {
        impl<M: ManagedTypeApi> From<$num_ty> for BigUint<M> {
            #[inline]
            fn from(value: $num_ty) -> Self {
                BigUint::from_raw_handle(M::managed_type_impl().bi_new(value as i64))
            }
        }
    };
}

big_uint_conv_num! {u64}
big_uint_conv_num! {u32}
big_uint_conv_num! {usize}
big_uint_conv_num! {u16}
big_uint_conv_num! {u8}

impl<M: ManagedTypeApi> Default for BigUint<M> {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

/// More conversions here.
impl<M: ManagedTypeApi> BigUint<M> {
    #[inline]
    pub fn zero() -> Self {
        BigUint::from_raw_handle(M::managed_type_impl().bi_new_zero())
    }

    #[inline]
    pub fn to_u64(&self) -> Option<u64> {
        let api = M::managed_type_impl();
        api.bi_to_i64(self.handle).map(|bi| bi as u64)
    }

    #[inline]
    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        let api = M::managed_type_impl();
        let handle = api.bi_new(0);
        api.bi_set_unsigned_bytes(handle, bytes);
        BigUint {
            handle,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn to_bytes_be(&self) -> BoxedBytes {
        let api = M::managed_type_impl();
        api.bi_get_unsigned_bytes(self.handle)
    }

    #[inline]
    pub fn from_bytes_be_buffer(managed_buffer: &ManagedBuffer<M>) -> Self {
        BigUint::from_raw_handle(
            M::managed_type_impl().mb_to_big_int_unsigned(managed_buffer.handle),
        )
    }

    #[inline]
    pub fn to_bytes_be_buffer(&self) -> ManagedBuffer<M> {
        ManagedBuffer::from_raw_handle(M::managed_type_impl().mb_from_big_int_unsigned(self.handle))
    }

    pub fn copy_to_array_big_endian_pad_right(&self, target: &mut [u8; 32]) {
        let api = M::managed_type_impl();
        let mb_handle = api.mb_from_big_int_unsigned(self.handle);
        api.mb_copy_to_slice_pad_right(mb_handle, &mut target[..]);
    }
}

impl<M: ManagedTypeApi> BigUint<M> {
    #[inline]
    #[must_use]
    pub fn sqrt(&self) -> Self {
        let api = M::managed_type_impl();
        let handle = api.bi_new_zero();
        api.bi_sqrt(handle, self.handle);
        BigUint {
            handle,
            _phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn pow(&self, exp: u32) -> Self {
        let api = M::managed_type_impl();
        let handle = api.bi_new_zero();
        let exp_handle = api.bi_new(exp as i64);
        api.bi_pow(handle, self.handle, exp_handle);
        BigUint {
            handle,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn log2(&self) -> u32 {
        let api = M::managed_type_impl();
        api.bi_log2(self.handle)
    }
}

impl<M: ManagedTypeApi> Clone for BigUint<M> {
    fn clone(&self) -> Self {
        let api = M::managed_type_impl();
        let clone_handle = api.bi_new_zero();
        api.bi_add(clone_handle, clone_handle, self.handle);
        BigUint {
            handle: clone_handle,
            _phantom: PhantomData,
        }
    }
}

impl<M: ManagedTypeApi> TryStaticCast for BigUint<M> {}

impl<M: ManagedTypeApi> TopEncode for BigUint<M> {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        output.set_specialized(self, |else_output| {
            else_output.set_slice_u8(self.to_bytes_be().as_slice());
            Ok(())
        })
    }
}

impl<M: ManagedTypeApi> NestedEncode for BigUint<M> {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        dest.push_specialized((), self, |else_output| {
            self.to_bytes_be().as_slice().dep_encode(else_output)
        })
    }
}

impl<M: ManagedTypeApi> NestedDecode for BigUint<M> {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        input.read_specialized((), |_| Err(DecodeError::UNSUPPORTED_OPERATION))
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        input.read_specialized_or_exit((), c, exit, |_, c| {
            exit(c, DecodeError::UNSUPPORTED_OPERATION)
        })
    }
}

impl<M: ManagedTypeApi> TopDecode for BigUint<M> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        input.into_specialized(|_| Err(DecodeError::UNSUPPORTED_OPERATION))
    }
}

impl<M: ManagedTypeApi> crate::abi::TypeAbi for BigUint<M> {
    fn type_name() -> String {
        String::from("BigUint")
    }
}

impl<M: ManagedTypeApi> core::fmt::Debug for BigUint<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BigUint")
            .field("handle", &self.handle)
            .field(
                "hex-value-be",
                &encode_bytes_as_hex(self.to_bytes_be().as_slice()),
            )
            .finish()
    }
}
