use super::ManagedBuffer;

use crate::{
    api::{
        use_raw_handle, BigFloatApiImpl, ManagedTypeApi, ManagedTypeApiImpl, Sign, StaticVarApiImpl,
    },
    types::{BigInt, BigUint, ManagedType},
};
use alloc::string::String;

use crate::codec::{
    CodecFromSelf, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
    TryStaticCast,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct BigFloat<M: ManagedTypeApi> {
    pub(crate) handle: M::BigFloatHandle,
}

impl<M: ManagedTypeApi> ManagedType<M> for BigFloat<M> {
    type OwnHandle = M::BigFloatHandle;

    fn from_handle(handle: M::BigFloatHandle) -> Self {
        BigFloat { handle }
    }

    fn get_handle(&self) -> M::BigFloatHandle {
        self.handle.clone()
    }

    fn transmute_from_handle_ref(handle_ref: &M::BigFloatHandle) -> &Self {
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
                let new_bf_handle: M::BigFloatHandle =
                    use_raw_handle(M::static_var_api_impl().next_handle());
                M::managed_type_impl().bf_set_i64(new_bf_handle.clone(), value as i64);
                BigFloat::from_handle(new_bf_handle)
            }
        }
    };
}

big_float_conv_num! {i64}
big_float_conv_num! {i32}
big_float_conv_num! {isize}
big_float_conv_num! {i16}
big_float_conv_num! {i8}

impl<M> CodecFromSelf for BigFloat<M> where M: ManagedTypeApi {}

impl<M: ManagedTypeApi> BigFloat<M> {
    #[inline]
    pub fn neg(&self) -> Self {
        let new_bf_handle: M::BigFloatHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().bf_neg(new_bf_handle.clone(), self.handle.clone());
        BigFloat::from_handle(new_bf_handle)
    }

    #[inline]
    pub fn from_big_uint(big_uint: &BigUint<M>) -> Self {
        let new_bf_handle: M::BigFloatHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().bf_set_bi(new_bf_handle.clone(), big_uint.handle.clone());
        BigFloat::from_handle(new_bf_handle)
    }

    #[inline]
    pub fn from_big_int(big_int: &BigInt<M>) -> Self {
        let new_bf_handle: M::BigFloatHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().bf_set_bi(new_bf_handle.clone(), big_int.handle.clone());
        BigFloat::from_handle(new_bf_handle)
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
        BigFloat::from_handle(new_bf_handle)
    }

    #[inline]
    pub fn from_frac(numerator_value: i64, denominator_value: i64) -> Self {
        let api = M::managed_type_impl();
        let new_bf_handle = api.bf_from_frac(numerator_value, denominator_value);
        BigFloat::from_handle(new_bf_handle)
    }

    #[inline]
    pub fn from_sci(significand_value: i64, exponent_value: i32) -> Self {
        let api = M::managed_type_impl();
        let new_bf_handle = api.bf_from_sci(significand_value, exponent_value as i64);
        BigFloat::from_handle(new_bf_handle)
    }

    pub fn trunc(&self) -> BigInt<M> {
        let result: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        let api = M::managed_type_impl();
        api.bf_trunc(result.clone(), self.handle.clone());
        BigInt::from_handle(result)
    }

    pub fn floor(&self) -> BigInt<M> {
        let result: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        let api = M::managed_type_impl();
        api.bf_floor(result.clone(), self.handle.clone());
        BigInt::from_handle(result)
    }

    pub fn ceil(&self) -> BigInt<M> {
        let result: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        let api = M::managed_type_impl();
        api.bf_ceil(result.clone(), self.handle.clone());
        BigInt::from_handle(result)
    }

    pub fn to_fixed_point(&self, denominator: &BigFloat<M>) -> BigInt<M> {
        (self * denominator).trunc()
    }
}

impl<M: ManagedTypeApi> BigFloat<M> {
    #[inline]
    pub fn zero() -> Self {
        BigFloat::from_handle(M::managed_type_impl().bf_new_zero())
    }

    pub fn from_buffer(managed_buffer: &ManagedBuffer<M>) -> Self {
        let new_bf_handle: M::BigFloatHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl()
            .mb_to_big_float(managed_buffer.handle.clone(), new_bf_handle.clone());
        BigFloat::from_handle(new_bf_handle)
    }

    pub fn to_buffer(&self) -> ManagedBuffer<M> {
        let new_man_buf_handle: M::ManagedBufferHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().mb_from_big_float(self.handle.clone(), new_man_buf_handle.clone());
        ManagedBuffer::from_handle(new_man_buf_handle)
    }
}

impl<M: ManagedTypeApi> BigFloat<M> {
    pub fn sqrt(&self) -> Self {
        let api = M::managed_type_impl();
        let new_handle: M::BigFloatHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        api.bf_sqrt(new_handle.clone(), self.handle.clone());
        BigFloat::from_handle(new_handle)
    }

    pub fn pow(&self, exp: i32) -> Self {
        let api = M::managed_type_impl();
        let new_handle: M::BigFloatHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        api.bf_pow(new_handle.clone(), self.handle.clone(), exp);
        BigFloat::from_handle(new_handle)
    }

    /// Returns the sign of the `BigFloat` as a `Sign`.
    pub fn sign(&self) -> Sign {
        match M::managed_type_impl().bf_sign(self.handle.clone()) {
            crate::api::Sign::Plus => Sign::Plus,
            crate::api::Sign::NoSign => Sign::NoSign,
            crate::api::Sign::Minus => Sign::Minus,
        }
    }

    /// Returns the magnitude of the `BigFloat`
    pub fn magnitude(&self) -> BigFloat<M> {
        let result: M::BigFloatHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().bf_abs(result.clone(), self.handle.clone());
        BigFloat::from_handle(result)
    }

    /// Convert this `BigFloat` into its `Sign` and its magnitude,
    /// the reverse of `BigInt::from_biguint`.
    pub fn to_parts(self) -> (Sign, BigFloat<M>) {
        (self.sign(), self.magnitude())
    }
}

impl<M: ManagedTypeApi> Clone for BigFloat<M> {
    fn clone(&self) -> Self {
        let new_handle: M::BigFloatHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().bf_clone(new_handle.clone(), self.handle.clone());
        BigFloat::from_handle(new_handle)
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
