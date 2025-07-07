use super::ManagedBuffer;

use crate::{
    abi::{TypeAbi, TypeAbiFrom},
    api::{
        use_raw_handle, BigFloatApiImpl, ManagedTypeApi, ManagedTypeApiImpl, Sign, StaticVarApiImpl,
    },
    contract_base::ErrorHelper,
    types::{BigInt, BigUint, Decimals, ManagedDecimalSigned, ManagedType},
};
use alloc::string::String;

use crate::codec::{
    DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
    NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, TryStaticCast,
};

/// Denomiator used for initializing BigFloats from constants.
const DENOMINATOR: i64 = 1_000_000_000;

#[derive(Debug)]
#[repr(transparent)]
pub struct BigFloat<M: ManagedTypeApi> {
    pub(crate) handle: M::BigFloatHandle,
}

impl<M: ManagedTypeApi> ManagedType<M> for BigFloat<M> {
    type OwnHandle = M::BigFloatHandle;

    unsafe fn from_handle(handle: M::BigFloatHandle) -> Self {
        BigFloat { handle }
    }

    fn get_handle(&self) -> M::BigFloatHandle {
        self.handle.clone()
    }

    unsafe fn forget_into_handle(self) -> Self::OwnHandle {
        unsafe {
            let handle = core::ptr::read(&self.handle);
            core::mem::forget(self);
            handle
        }
    }

    fn transmute_from_handle_ref(handle_ref: &M::BigFloatHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }

    fn transmute_from_handle_ref_mut(handle_ref: &mut M::BigFloatHandle) -> &mut Self {
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
                unsafe {
                    let result = BigFloat::new_uninit();
                    M::managed_type_impl().bf_set_i64(result.get_handle(), value as i64);
                    result
                }
            }
        }
    };
}

big_float_conv_num! {i64}
big_float_conv_num! {i32}
big_float_conv_num! {isize}
big_float_conv_num! {i16}
big_float_conv_num! {i8}

impl<M: ManagedTypeApi> BigFloat<M> {
    pub fn neg(&self) -> Self {
        unsafe {
            let result = BigFloat::new_uninit();
            M::managed_type_impl().bf_neg(result.get_handle(), self.handle.clone());
            result
        }
    }

    pub fn abs(&self) -> Self {
        unsafe {
            let result = BigFloat::new_uninit();
            M::managed_type_impl().bf_abs(result.get_handle(), self.handle.clone());
            result
        }
    }

    pub fn from_big_int(big_int: &BigInt<M>) -> Self {
        unsafe {
            let result = BigFloat::new_uninit();
            M::managed_type_impl().bf_set_bi(result.get_handle(), big_int.handle.clone());
            result
        }
    }

    pub fn from_big_uint(big_uint: &BigUint<M>) -> Self {
        Self::from_big_int(big_uint.as_big_int())
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
        unsafe { BigFloat::from_handle(new_bf_handle) }
    }

    #[inline]
    pub fn from_frac(numerator_value: i64, denominator_value: i64) -> Self {
        let api = M::managed_type_impl();
        let new_bf_handle = api.bf_from_frac(numerator_value, denominator_value);
        unsafe { BigFloat::from_handle(new_bf_handle) }
    }

    #[inline]
    pub fn from_sci(significand_value: i64, exponent_value: i32) -> Self {
        let api = M::managed_type_impl();
        let new_bf_handle = api.bf_from_sci(significand_value, exponent_value as i64);
        unsafe { BigFloat::from_handle(new_bf_handle) }
    }

    pub fn trunc(&self) -> BigInt<M> {
        unsafe {
            let result = BigInt::new_uninit();
            M::managed_type_impl().bf_trunc(result.get_handle(), self.handle.clone());
            result
        }
    }

    pub fn floor(&self) -> BigInt<M> {
        unsafe {
            let result = BigInt::new_uninit();
            M::managed_type_impl().bf_floor(result.get_handle(), self.handle.clone());
            result
        }
    }

    pub fn ceil(&self) -> BigInt<M> {
        unsafe {
            let result = BigInt::new_uninit();
            M::managed_type_impl().bf_ceil(result.get_handle(), self.handle.clone());
            result
        }
    }

    pub fn to_fixed_point(&self, denominator: &BigFloat<M>) -> BigInt<M> {
        (self * denominator).trunc()
    }

    pub fn to_managed_decimal_signed<T: Decimals>(
        &self,
        decimals: T,
    ) -> ManagedDecimalSigned<M, T> {
        ManagedDecimalSigned::<M, T>::from_big_float(self, decimals)
    }

    /// Computes the natural logarithm of the current number.
    ///
    /// The error is around +/- 0.00006, for all inputs.
    ///
    /// Will return `None` for zero or negative numbers.
    pub fn ln(&self) -> Option<Self> {
        if self <= &0i64 {
            return None;
        }

        let one = BigFloat::from(1i64);
        match self.cmp(&one) {
            core::cmp::Ordering::Less => {
                let inv = &one / self;
                debug_assert!(inv > one);
                Some(inv.ln_gt_one().neg())
            },
            core::cmp::Ordering::Equal => Some(BigFloat::from(0i64)),
            core::cmp::Ordering::Greater => Some(self.ln_gt_one()),
        }
    }

    /// Computes the natural logarithm for values between 1 and 2. Performs very poorly outside of this interval.
    fn ln_between_one_and_two(&self) -> Self {
        let mut result = BigFloat::from_frac(-56570851, DENOMINATOR); // -0.056570851
        result *= self;
        result += BigFloat::from_frac(447179550, DENOMINATOR); // 0.44717955
        result *= self;
        result += BigFloat::from_frac(-1469956800, DENOMINATOR); // -1.4699568
        result *= self;
        result += BigFloat::from_frac(2821202600, DENOMINATOR); // 2.8212026
        result *= self;
        result += BigFloat::from_frac(-1741793900, DENOMINATOR); // -1.7417939

        result
    }

    /// Computes the natural logarithm for values > 1.
    fn ln_gt_one(&self) -> Self {
        // find the highest power of 2 less than or equal to self
        let trunc_val = self.trunc();
        let trunc_val_unsigned = trunc_val
            .into_big_uint()
            .unwrap_or_sc_panic("log argument must be positive");

        // start with approximation, based on position of the most significant bit
        let Some(log2_floor) = trunc_val_unsigned.log2_floor() else {
            // means the input was zero, practically unreachable
            return BigFloat::from(0i64);
        };

        let divisor = BigFloat::from(1 << log2_floor);
        let x = self / &divisor; // normalize to [1.0, 2.0]

        debug_assert!(x >= 1);
        debug_assert!(x <= 2);

        let mut result = x.ln_between_one_and_two();

        let ln_of_2 = BigFloat::from_frac(693147180, DENOMINATOR); // 0.69314718
        result += BigFloat::from(log2_floor as i32) * ln_of_2;

        result
    }

    #[inline]
    pub fn zero() -> Self {
        unsafe { BigFloat::from_handle(M::managed_type_impl().bf_new_zero()) }
    }

    pub fn from_buffer(managed_buffer: &ManagedBuffer<M>) -> Self {
        unsafe {
            let result = BigFloat::new_uninit();
            M::managed_type_impl()
                .mb_to_big_float(managed_buffer.handle.clone(), result.get_handle());
            result
        }
    }

    pub fn to_buffer(&self) -> ManagedBuffer<M> {
        unsafe {
            let result = ManagedBuffer::new_uninit();
            M::managed_type_impl().mb_from_big_float(self.get_handle(), result.get_handle());
            result
        }
    }

    /// Creates a new object, without initializing it.
    ///
    /// ## Safety
    ///
    /// The value needs to be initialized after creation, otherwise the VM will halt the first time the value is attempted to be read.
    pub unsafe fn new_uninit() -> Self {
        let new_handle: M::BigFloatHandle = use_raw_handle(M::static_var_api_impl().next_handle());
        BigFloat::from_handle(new_handle)
    }
}

impl<M: ManagedTypeApi> BigFloat<M> {
    pub fn sqrt(&self) -> Self {
        unsafe {
            let result = BigFloat::new_uninit();
            M::managed_type_impl().bf_sqrt(result.get_handle(), self.handle.clone());
            result
        }
    }

    pub fn pow(&self, exp: i32) -> Self {
        unsafe {
            let result = BigFloat::new_uninit();
            M::managed_type_impl().bf_pow(result.get_handle(), self.handle.clone(), exp);
            result
        }
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
        unsafe {
            let result = BigFloat::new_uninit();
            M::managed_type_impl().bf_abs(result.get_handle(), self.handle.clone());
            result
        }
    }

    /// Convert this `BigFloat` into its `Sign` and its magnitude,
    /// the reverse of `BigInt::from_biguint`.
    pub fn to_parts(self) -> (Sign, BigFloat<M>) {
        (self.sign(), self.magnitude())
    }
}

impl<M: ManagedTypeApi> From<f64> for BigFloat<M> {
    fn from(x: f64) -> Self {
        const PREC: i64 = 1_000_000_000;
        Self::from_frac((x * PREC as f64) as i64, PREC)
    }
}

impl<M: ManagedTypeApi> From<f32> for BigFloat<M> {
    fn from(x: f32) -> Self {
        Self::from(x as f64)
    }
}

impl<M: ManagedTypeApi> BigFloat<M> {
    /// Warning: cannot be used in contracts. It is only meant to simplify certain tests.
    ///
    /// It might also not be optimal with respect to precision.
    pub fn to_f64(&self) -> f64 {
        const PREC: i64 = 1_000_000_000;
        let mut rescaled = Self::from(PREC);
        rescaled *= self;
        let ln_units = rescaled.trunc().to_i64().unwrap_or_else(|| {
            ErrorHelper::<M>::signal_error_with_message("BigFloat out of precision range")
        });
        ln_units as f64 / PREC as f64
    }
}

impl<M: ManagedTypeApi> Clone for BigFloat<M> {
    fn clone(&self) -> Self {
        unsafe {
            let result = BigFloat::new_uninit();
            M::managed_type_impl().bf_clone(result.get_handle(), self.handle.clone());
            result
        }
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

impl<M> TypeAbiFrom<BigFloat<M>> for f64 where M: ManagedTypeApi {}

impl<M> TypeAbiFrom<Self> for BigFloat<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&Self> for BigFloat<M> where M: ManagedTypeApi {}

impl<M: ManagedTypeApi> TypeAbi for BigFloat<M> {
    type Unmanaged = f64;

    fn type_name() -> String {
        String::from("BigFloat")
    }
}
