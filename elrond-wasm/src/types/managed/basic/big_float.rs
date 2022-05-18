use super::ManagedBuffer;
use core::marker::PhantomData;

use crate::{
    api::{BigFloatApi, Handle, ManagedTypeApi, ManagedTypeApiImpl, Sign, StaticVarApiImpl},
    types::{BigInt, BigUint, ManagedType},
};
use alloc::string::String;

use elrond_codec::{
    DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
    NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, TryStaticCast,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct BigFloat<M: ManagedTypeApi> {
    pub(crate) handle: Handle,
    _phantom: PhantomData<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for BigFloat<M> {
    fn from_raw_handle(handle: Handle) -> Self {
        BigFloat {
            handle,
            _phantom: PhantomData,
        }
    }

    fn get_raw_handle(&self) -> Handle {
        self.handle
    }

    fn transmute_from_handle_ref(handle_ref: &Handle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> From<&ManagedBuffer<M>> for BigFloat<M> {
    fn from(item: &ManagedBuffer<M>) -> Self {
        BigFloat::from_buffer(item)
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for BigFloat<M> {
    fn from(item: ManagedBuffer<M>) -> Self {
        BigFloat::from_buffer(&item)
    }
}

impl<M: ManagedTypeApi> From<&BigUint<M>> for BigFloat<M> {
    fn from(item: &BigUint<M>) -> Self {
        BigFloat::from_big_uint(item)
    }
}

impl<M: ManagedTypeApi> From<BigUint<M>> for BigFloat<M> {
    fn from(item: BigUint<M>) -> Self {
        BigFloat::from_big_uint(&item)
    }
}

impl<M: ManagedTypeApi> From<&BigInt<M>> for BigFloat<M> {
    fn from(item: &BigInt<M>) -> Self {
        BigFloat::from_big_int(item)
    }
}

impl<M: ManagedTypeApi> From<BigInt<M>> for BigFloat<M> {
    fn from(item: BigInt<M>) -> Self {
        BigFloat::from_big_int(&item)
    }
}

macro_rules! big_float_conv_num {
    ($num_ty:ty) => {
        impl<M: ManagedTypeApi> From<$num_ty> for BigFloat<M> {
            #[inline]
            fn from(value: $num_ty) -> Self {
                let new_bf_handle = M::static_var_api_impl().next_handle();
                M::managed_type_impl().bf_set_i64(new_bf_handle, value as i64);
                BigFloat::from_raw_handle(new_bf_handle)
            }
        }
    };
}

// TODO: more coverage, only from i64 currently tested
big_float_conv_num! {i64}
big_float_conv_num! {i32}
big_float_conv_num! {isize}
big_float_conv_num! {i16}
big_float_conv_num! {i8}

impl<M: ManagedTypeApi> BigFloat<M> {
    #[inline]
    pub fn neg(&self) -> Self {
        let new_bf_handle = M::static_var_api_impl().next_handle();
        M::managed_type_impl().bf_neg(new_bf_handle, self.handle);
        BigFloat::from_raw_handle(new_bf_handle)
    }

    #[inline]
    pub fn from_big_uint(big_uint: &BigUint<M>) -> Self {
        let new_bf_handle = M::static_var_api_impl().next_handle();
        M::managed_type_impl().bf_set_bi(new_bf_handle, big_uint.handle);
        BigFloat::from_raw_handle(new_bf_handle)
    }

    #[inline]
    pub fn from_big_int(big_int: &BigInt<M>) -> Self {
        let new_bf_handle = M::static_var_api_impl().next_handle();
        M::managed_type_impl().bf_set_bi(new_bf_handle, big_int.handle);
        BigFloat::from_raw_handle(new_bf_handle)
    }

    #[inline]
    pub fn from_parts(
        integral_part_value: i32,
        fractional_part_value: i32,
        exponent_value: i32,
    ) -> Self {
        let api = M::managed_type_impl();
        let new_bf_handle =
            api.bf_from_parts(integral_part_value, fractional_part_value, exponent_value);
        BigFloat::from_raw_handle(new_bf_handle)
    }

    #[inline]
    pub fn from_frac(numerator_value: i64, denominator_value: i64) -> Self {
        let api = M::managed_type_impl();
        let new_bf_handle = api.bf_from_frac(numerator_value, denominator_value);
        BigFloat::from_raw_handle(new_bf_handle)
    }

    #[inline]
    pub fn from_sci(significand_value: i64, exponent_value: i32) -> Self {
        let api = M::managed_type_impl();
        let new_bf_handle = api.bf_from_sci(significand_value, exponent_value as i64);
        BigFloat::from_raw_handle(new_bf_handle)
    }
}

impl<M: ManagedTypeApi> BigFloat<M> {
    #[inline]
    pub fn zero() -> Self {
        BigFloat::from_raw_handle(M::managed_type_impl().bf_new_zero())
    }

    pub fn from_buffer(managed_buffer: &ManagedBuffer<M>) -> Self {
        let new_bf_handle = M::static_var_api_impl().next_handle();
        M::managed_type_impl().mb_to_big_float(managed_buffer.handle, new_bf_handle);
        BigFloat::from_raw_handle(new_bf_handle)
    }

    pub fn to_buffer(&self) -> ManagedBuffer<M> {
        let new_man_buf_handle = M::static_var_api_impl().next_handle();
        M::managed_type_impl().mb_from_big_float(self.handle, new_man_buf_handle);
        ManagedBuffer::from_raw_handle(new_man_buf_handle)
    }
}

impl<M: ManagedTypeApi> BigFloat<M> {
    pub fn sqrt(&self) -> Self {
        let api = M::managed_type_impl();
        let new_handle = M::static_var_api_impl().next_handle();
        api.bf_sqrt(new_handle, self.handle);
        BigFloat::from_raw_handle(new_handle)
    }

    pub fn pow(&self, exp: u32) -> Self {
        let api = M::managed_type_impl();
        let new_handle = M::static_var_api_impl().next_handle();
        api.bf_pow(new_handle, self.handle, exp as i32);
        BigFloat::from_raw_handle(new_handle)
    }

    /// Returns the sign of the `BigFloat` as a `Sign`.
    pub fn sign(&self) -> Sign {
        match M::managed_type_impl().bf_sign(self.handle) {
            crate::api::Sign::Plus => Sign::Plus,
            crate::api::Sign::NoSign => Sign::NoSign,
            crate::api::Sign::Minus => Sign::Minus,
        }
    }

    /// Returns the magnitude of the `BigFloat`
    pub fn magnitude(&self) -> BigFloat<M> {
        let result = M::static_var_api_impl().next_handle();
        M::managed_type_impl().bf_abs(result, self.handle);
        BigFloat::from_raw_handle(result)
    }

    /// Convert this `BigFloat` into its `Sign` and its magnitude,
    /// the reverse of `BigInt::from_biguint`.
    pub fn to_parts(self) -> (Sign, BigFloat<M>) {
        (self.sign(), self.magnitude())
    }
}

impl<M: ManagedTypeApi> Clone for BigFloat<M> {
    fn clone(&self) -> Self {
        let new_handle = M::static_var_api_impl().next_handle();
        M::managed_type_impl().bf_clone(new_handle, self.handle);
        BigFloat::from_raw_handle(new_handle)
    }
}

impl<M: ManagedTypeApi> TryStaticCast for BigFloat<M> {}

impl<M: ManagedTypeApi> TopEncode for BigFloat<M> {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.to_buffer().top_encode_or_handle_err(output, h)
    }
}

impl<M: ManagedTypeApi> TopDecode for BigFloat<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(BigFloat::from(ManagedBuffer::top_decode_or_handle_err(
            input, h,
        )?))
    }
}

impl<M: ManagedTypeApi> NestedEncode for BigFloat<M> {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.to_buffer().dep_encode_or_handle_err(dest, h)
    }
}

impl<M: ManagedTypeApi> NestedDecode for BigFloat<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(BigFloat::from(ManagedBuffer::dep_decode_or_handle_err(
            input, h,
        )?))
    }
}

impl<M: ManagedTypeApi> crate::abi::TypeAbi for BigFloat<M> {
    fn type_name() -> String {
        String::from("BigFloat")
    }
}
