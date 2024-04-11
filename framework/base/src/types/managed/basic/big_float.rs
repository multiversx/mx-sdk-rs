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
pub struct BigFloat<'a, M: ManagedTypeApi<'a>> {
    pub(crate) handle: M::BigFloatHandle,
}

impl<'a, M: ManagedTypeApi<'a>> ManagedType<'a, M> for BigFloat<'a, M> {
    type OwnHandle = M::BigFloatHandle;

    fn from_handle(handle: M::BigFloatHandle) -> Self {
        BigFloat { handle }
    }

    unsafe fn get_handle(&self) -> M::BigFloatHandle {
        self.handle.clone()
    }

    fn take_handle(mut self) -> Self::OwnHandle {
        core::mem::take(&mut self.handle)
    }

    fn transmute_from_handle_ref(handle_ref: &M::BigFloatHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<'a, M: ManagedTypeApi<'a>> From<&ManagedBuffer<'a, M>> for BigFloat<'a, M> {
    fn from(item: &ManagedBuffer<'a, M>) -> Self {
        BigFloat::from_buffer(item)
    }
}

impl<'a, M: ManagedTypeApi<'a>> From<ManagedBuffer<'a, M>> for BigFloat<'a, M> {
    fn from(item: ManagedBuffer<'a, M>) -> Self {
        BigFloat::from_buffer(&item)
    }
}

impl<'a, M: ManagedTypeApi<'a>> From<&BigUint<'a, M>> for BigFloat<'a, M> {
    fn from(item: &BigUint<'a, M>) -> Self {
        BigFloat::from_big_uint(item)
    }
}

impl<'a, M: ManagedTypeApi<'a>> From<BigUint<'a, M>> for BigFloat<'a, M> {
    fn from(item: BigUint<'a, M>) -> Self {
        BigFloat::from_big_uint(&item)
    }
}

impl<'a, M: ManagedTypeApi<'a>> From<&BigInt<'a, M>> for BigFloat<'a, M> {
    fn from(item: &BigInt<'a, M>) -> Self {
        BigFloat::from_big_int(item)
    }
}

impl<'a, M: ManagedTypeApi<'a>> From<BigInt<'a, M>> for BigFloat<'a, M> {
    fn from(item: BigInt<'a, M>) -> Self {
        BigFloat::from_big_int(&item)
    }
}

macro_rules! big_float_conv_num {
    ($num_ty:ty) => {
        impl<'a, M: ManagedTypeApi<'a>> From<$num_ty> for BigFloat<'a, M> {
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

impl<'a, M> CodecFromSelf for BigFloat<'a, M> where M: ManagedTypeApi<'a> {}

impl<'a, M: ManagedTypeApi<'a>> BigFloat<'a, M> {
    #[inline]
    pub fn neg(&self) -> Self {
        let new_bf_handle: M::BigFloatHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().bf_neg(new_bf_handle.clone(), self.handle.clone());
        BigFloat::from_handle(new_bf_handle)
    }

    #[inline]
    pub fn from_big_uint(big_uint: &BigUint<'a, M>) -> Self {
        let new_bf_handle: M::BigFloatHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().bf_set_bi(new_bf_handle.clone(), big_uint.handle.clone());
        BigFloat::from_handle(new_bf_handle)
    }

    #[inline]
    pub fn from_big_int(big_int: &BigInt<'a, M>) -> Self {
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

    pub fn trunc(&self) -> BigInt<'a, M> {
        let result: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        let api = M::managed_type_impl();
        api.bf_trunc(result.clone(), self.handle.clone());
        BigInt::from_handle(result)
    }

    pub fn floor(&self) -> BigInt<'a, M> {
        let result: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        let api = M::managed_type_impl();
        api.bf_floor(result.clone(), self.handle.clone());
        BigInt::from_handle(result)
    }

    pub fn ceil(&self) -> BigInt<'a, M> {
        let result: M::BigIntHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        let api = M::managed_type_impl();
        api.bf_ceil(result.clone(), self.handle.clone());
        BigInt::from_handle(result)
    }

    pub fn to_fixed_point(&self, denominator: &BigFloat<'a, M>) -> BigInt<'a, M> {
        (self * denominator).trunc()
    }
}

impl<'a, M: ManagedTypeApi<'a>> BigFloat<'a, M> {
    #[inline]
    pub fn zero() -> Self {
        BigFloat::from_handle(M::managed_type_impl().bf_new_zero())
    }

    pub fn from_buffer(managed_buffer: &ManagedBuffer<'a, M>) -> Self {
        let new_bf_handle: M::BigFloatHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl()
            .mb_to_big_float(managed_buffer.handle.clone(), new_bf_handle.clone());
        BigFloat::from_handle(new_bf_handle)
    }

    pub fn to_buffer(&self) -> ManagedBuffer<'a, M> {
        let new_man_buf_handle: M::ManagedBufferHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().mb_from_big_float(self.handle.clone(), new_man_buf_handle.clone());
        ManagedBuffer::from_handle(new_man_buf_handle)
    }
}

impl<'a, M: ManagedTypeApi<'a>> BigFloat<'a, M> {
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
    pub fn magnitude(&self) -> BigFloat<'a, M> {
        let result: M::BigFloatHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().bf_abs(result.clone(), self.handle.clone());
        BigFloat::from_handle(result)
    }

    /// Convert this `BigFloat` into its `Sign` and its magnitude,
    /// the reverse of `BigInt::from_biguint`.
    pub fn to_parts(self) -> (Sign, BigFloat<'a, M>) {
        (self.sign(), self.magnitude())
    }
}

impl<'a, M: ManagedTypeApi<'a>> Clone for BigFloat<'a, M> {
    fn clone(&self) -> Self {
        let new_handle: M::BigFloatHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().bf_clone(new_handle.clone(), self.handle.clone());
        BigFloat::from_handle(new_handle)
    }
}

impl<'a, M: ManagedTypeApi<'a>> TryStaticCast for BigFloat<'a, M> {}

impl<'a, M: ManagedTypeApi<'a>> TopEncode for BigFloat<'a, M> {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.to_buffer().top_encode_or_handle_err(output, h)
    }
}

impl<'a, M: ManagedTypeApi<'a>> TopDecode for BigFloat<'a, M> {
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

impl<'a, M: ManagedTypeApi<'a>> NestedEncode for BigFloat<'a, M> {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.to_buffer().dep_encode_or_handle_err(dest, h)
    }
}

impl<'a, M: ManagedTypeApi<'a>> NestedDecode for BigFloat<'a, M> {
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

impl<'a, M: ManagedTypeApi<'a>> crate::abi::TypeAbi for BigFloat<'a, M> {
    fn type_name() -> String {
        String::from("BigFloat")
    }
}
