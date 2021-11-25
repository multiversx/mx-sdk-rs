use core::marker::PhantomData;

use super::{ManagedBuffer, ManagedType};
use crate::{
    api::{Handle, ManagedTypeApi},
    types::BoxedBytes,
};
use alloc::string::String;
use elrond_codec::{
    DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, TryStaticCast,
};

#[derive(Debug)]
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
                BigUint::from_raw_handle(M::instance().bi_new(value as i64))
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
        BigUint::from_raw_handle(M::instance().bi_new_zero())
    }

    #[inline]
    pub fn to_u64(&self) -> Option<u64> {
        self.type_manager()
            .bi_to_i64(self.handle)
            .map(|bi| bi as u64)
    }

    #[inline]
    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        let api = M::instance();
        let handle = api.bi_new(0);
        api.bi_set_unsigned_bytes(handle, bytes);
        BigUint {
            handle,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn to_bytes_be(&self) -> BoxedBytes {
        self.type_manager().bi_get_unsigned_bytes(self.handle)
    }

    #[inline]
    pub fn from_bytes_be_buffer(managed_buffer: &ManagedBuffer<M>) -> Self {
        BigUint::from_raw_handle(M::instance().mb_to_big_int_unsigned(managed_buffer.handle))
    }

    #[inline]
    pub fn to_bytes_be_buffer(&self) -> ManagedBuffer<M> {
        ManagedBuffer::from_raw_handle(M::instance().mb_from_big_int_unsigned(self.handle))
    }

    pub fn copy_to_array_big_endian_pad_right(&self, target: &mut [u8; 32]) {
        let mb_handle = self.type_manager().mb_from_big_int_unsigned(self.handle);
        self.type_manager()
            .mb_copy_to_slice_pad_right(mb_handle, &mut target[..]);
    }
}

impl<M: ManagedTypeApi> BigUint<M> {
    #[inline]
    pub fn sqrt(&self) -> Self {
        let handle = self.type_manager().bi_new_zero();
        self.type_manager().bi_sqrt(handle, self.handle);
        BigUint {
            handle,
            _phantom: PhantomData,
        }
    }

    pub fn pow(&self, exp: u32) -> Self {
        let handle = self.type_manager().bi_new_zero();
        let exp_handle = self.type_manager().bi_new(exp as i64);
        self.type_manager().bi_pow(handle, self.handle, exp_handle);
        BigUint {
            handle,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn log2(&self) -> u32 {
        self.type_manager().bi_log2(self.handle)
    }
}

impl<M: ManagedTypeApi> Clone for BigUint<M> {
    fn clone(&self) -> Self {
        let clone_handle = self.type_manager().bi_new_zero();
        self.type_manager()
            .bi_add(clone_handle, clone_handle, self.handle);
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
