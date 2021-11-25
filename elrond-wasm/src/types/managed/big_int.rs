use core::marker::PhantomData;

use super::{BigUint, ManagedBuffer, ManagedType, Sign};
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
pub struct BigInt<M: ManagedTypeApi> {
    pub(crate) handle: Handle,
    _phantom: PhantomData<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for BigInt<M> {
    #[doc(hidden)]
    fn from_raw_handle(handle: Handle) -> Self {
        BigInt {
            handle,
            _phantom: PhantomData,
        }
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.handle
    }
}

impl<M: ManagedTypeApi> Default for BigInt<M> {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl<M: ManagedTypeApi> From<&ManagedBuffer<M>> for BigInt<M> {
    #[inline]
    fn from(item: &ManagedBuffer<M>) -> Self {
        BigInt::from_signed_bytes_be_buffer(item)
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for BigInt<M> {
    #[inline]
    fn from(item: ManagedBuffer<M>) -> Self {
        BigInt::from_signed_bytes_be_buffer(&item)
    }
}

macro_rules! big_int_conv_num {
    ($num_ty:ty) => {
        impl<M: ManagedTypeApi> From<$num_ty> for BigInt<M> {
            #[inline]
            fn from(value: $num_ty) -> Self {
                BigInt::from_raw_handle(M::instance().bi_new(value as i64))
            }
        }
    };
}

// TODO: more coverage, only from i64 currently tested
big_int_conv_num! {i64}
big_int_conv_num! {i32}
big_int_conv_num! {isize}
big_int_conv_num! {i16}
big_int_conv_num! {i8}

impl<M: ManagedTypeApi> BigInt<M> {
    #[inline]
    pub fn zero() -> Self {
        BigInt::from_raw_handle(M::instance().bi_new_zero())
    }

    #[inline]
    pub fn to_i64(&self) -> Option<i64> {
        M::instance().bi_to_i64(self.handle)
    }

    #[inline]
    pub fn from_signed_bytes_be(bytes: &[u8]) -> Self {
        let api = M::instance();
        let handle = api.bi_new(0);
        api.bi_set_signed_bytes(handle, bytes);
        BigInt::from_raw_handle(handle)
    }

    #[inline]
    pub fn to_signed_bytes_be(&self) -> BoxedBytes {
        let api = M::instance();
        api.bi_get_signed_bytes(self.handle)
    }

    #[inline]
    pub fn from_signed_bytes_be_buffer(managed_buffer: &ManagedBuffer<M>) -> Self {
        BigInt::from_raw_handle(M::instance().mb_to_big_int_signed(managed_buffer.handle))
    }

    #[inline]
    pub fn to_signed_bytes_be_buffer(&self) -> ManagedBuffer<M> {
        ManagedBuffer::from_raw_handle(M::instance().mb_from_big_int_signed(self.handle))
    }
}

impl<M: ManagedTypeApi> Clone for BigInt<M> {
    fn clone(&self) -> Self {
        let api = M::instance();
        let clone_handle = api.bi_new_zero();
        api.bi_add(clone_handle, clone_handle, self.handle);
        BigInt::from_raw_handle(clone_handle)
    }
}

impl<M: ManagedTypeApi> BigInt<M> {
    pub fn from_biguint(sign: Sign, unsigned: BigUint<M>) -> Self {
        let api = M::instance();
        if sign.is_minus() {
            api.bi_neg(unsigned.handle, unsigned.handle);
        }
        BigInt::from_raw_handle(unsigned.handle)
    }

    /// Returns the sign of the `BigInt` as a `Sign`.
    pub fn sign(&self) -> Sign {
        let api = M::instance();
        match api.bi_sign(self.handle) {
            crate::api::Sign::Plus => Sign::Plus,
            crate::api::Sign::NoSign => Sign::NoSign,
            crate::api::Sign::Minus => Sign::Minus,
        }
    }

    /// Returns the magnitude of the `BigInt` as a `BigUint`.
    pub fn magnitude(&self) -> BigUint<M> {
        let api = M::instance();
        let result = api.bi_new_zero();
        api.bi_abs(result, self.handle);
        BigUint::from_raw_handle(result)
    }

    /// Convert this `BigInt` into its `Sign` and `BigUint` magnitude,
    /// the reverse of `BigInt::from_biguint`.
    pub fn to_parts(self) -> (Sign, BigUint<M>) {
        (self.sign(), self.magnitude())
    }

    /// Converts this `BigInt` into a `BigUint`, if it's not negative.
    pub fn into_biguint(self) -> Option<BigUint<M>> {
        if let Sign::Minus = self.sign() {
            None
        } else {
            Some(BigUint::from_raw_handle(self.handle))
        }
    }
}

impl<M: ManagedTypeApi> TryStaticCast for BigInt<M> {}

impl<M: ManagedTypeApi> TopEncode for BigInt<M> {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        output.set_specialized(self, |else_output| {
            else_output.set_slice_u8(self.to_signed_bytes_be().as_slice());
            Ok(())
        })
    }
}

impl<M: ManagedTypeApi> NestedEncode for BigInt<M> {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        dest.push_specialized((), self, |else_output| {
            self.to_signed_bytes_be().dep_encode(else_output)
        })
    }
}

impl<M: ManagedTypeApi> NestedDecode for BigInt<M> {
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

impl<M: ManagedTypeApi> TopDecode for BigInt<M> {
    #[inline]
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        input.into_specialized(|_| Err(DecodeError::UNSUPPORTED_OPERATION))
    }
}

impl<M: ManagedTypeApi> crate::abi::TypeAbi for BigInt<M> {
    fn type_name() -> String {
        String::from("BigInt")
    }
}

impl<M: ManagedTypeApi> BigInt<M> {
    pub fn pow(&self, exp: u32) -> Self {
        let api = M::instance();
        let handle = api.bi_new_zero();
        let exp_handle = api.bi_new(exp as i64);
        api.bi_pow(handle, self.handle, exp_handle);
        BigInt::from_raw_handle(handle)
    }
}
