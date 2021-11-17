use super::{ManagedBuffer, ManagedDefault, ManagedFrom, ManagedType};
use crate::{
    api::{Handle, ManagedTypeApi, Sign},
    types::{BigInt, BigUint},
};
use alloc::string::String;

use elrond_codec::{
    DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, TryStaticCast,
};

#[derive(Debug)]
pub struct BigFloat<M: ManagedTypeApi> {
    pub(crate) handle: Handle,
    pub(crate) api: M,
}

impl<M, U> ManagedFrom<M, U> for BigFloat<M>
where
    M: ManagedTypeApi,
    U: Into<i64>,
{
    fn managed_from(api: M, value: U) -> Self {
        BigFloat::from_i64(api, value.into())
    }
}

impl<M: ManagedTypeApi> ManagedFrom<M, &ManagedBuffer<M>> for BigFloat<M> {
    #[inline]
    fn managed_from(_api: M, item: &ManagedBuffer<M>) -> Self {
        Self::from(item)
    }
}

impl<M: ManagedTypeApi> ManagedFrom<M, ManagedBuffer<M>> for BigFloat<M> {
    #[inline]
    fn managed_from(_api: M, item: ManagedBuffer<M>) -> Self {
        Self::from(item)
    }
}

impl<M: ManagedTypeApi> ManagedFrom<M, &BigUint<M>> for BigFloat<M> {
    #[inline]
    fn managed_from(_api: M, item: &BigUint<M>) -> Self {
        Self::from(item)
    }
}

impl<M: ManagedTypeApi> ManagedFrom<M, BigUint<M>> for BigFloat<M> {
    #[inline]
    fn managed_from(_api: M, item: BigUint<M>) -> Self {
        Self::from(item)
    }
}

impl<M: ManagedTypeApi> ManagedFrom<M, &BigInt<M>> for BigFloat<M> {
    #[inline]
    fn managed_from(_api: M, item: &BigInt<M>) -> Self {
        Self::from(item)
    }
}

impl<M: ManagedTypeApi> ManagedFrom<M, BigInt<M>> for BigFloat<M> {
    #[inline]
    fn managed_from(_api: M, item: BigInt<M>) -> Self {
        Self::from(item)
    }
}

impl<M: ManagedTypeApi> ManagedType<M> for BigFloat<M> {
    #[doc(hidden)]
    fn from_raw_handle(api: M, raw_handle: Handle) -> Self {
        BigFloat {
            handle: raw_handle,
            api,
        }
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.handle
    }

    #[inline]
    fn type_manager(&self) -> M {
        self.api.clone()
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

impl<M: ManagedTypeApi> ManagedDefault<M> for BigFloat<M> {
    #[inline]
    fn managed_default(api: M) -> Self {
        Self::from_i64(api, 0)
    }
}

impl<M: ManagedTypeApi> BigFloat<M> {
    #[inline]
    pub fn neg(&self) -> Self {
        let new_bf_handle = self.api.bf_new_zero();
        self.api.bf_neg(new_bf_handle, self.handle);
        BigFloat {
            handle: new_bf_handle,
            api: self.api.clone(),
        }
    }

    #[inline]
    pub fn from_i64(api: M, small_value: i64) -> Self {
        let new_bf_handle = api.bf_new_zero();
        api.bf_set_i64(new_bf_handle, small_value);
        BigFloat {
            handle: new_bf_handle,
            api,
        }
    }

    #[inline]
    pub fn from_big_uint(big_uint: &BigUint<M>) -> Self {
        let new_bf_handle = big_uint.api.bf_new_zero();
        big_uint.api.bf_set_bi(new_bf_handle, big_uint.handle);
        BigFloat {
            handle: new_bf_handle,
            api: big_uint.api.clone(),
        }
    }

    #[inline]
    pub fn from_big_int(big_int: &BigInt<M>) -> Self {
        let new_bf_handle = big_int.api.bf_new_zero();
        big_int.api.bf_set_bi(new_bf_handle, big_int.handle);
        BigFloat {
            handle: new_bf_handle,
            api: big_int.api.clone(),
        }
    }

    #[inline]
    pub fn from_parts(
        api: M,
        integral_part_value: i32,
        fractional_part_value: i32,
        exponent_value: i32,
    ) -> Self {
        let new_bf_handle =
            api.bf_from_parts(integral_part_value, fractional_part_value, exponent_value);
        BigFloat {
            handle: new_bf_handle,
            api,
        }
    }

    #[inline]
    pub fn from_frac(api: M, numerator_value: i64, denominator_value: i64) -> Self {
        let new_bf_handle = api.bf_from_frac(numerator_value, denominator_value);
        BigFloat {
            handle: new_bf_handle,
            api,
        }
    }

    #[inline]
    pub fn from_sci(api: M, significand_value: i64, exponent_value: i32) -> Self {
        let new_bf_handle = api.bf_from_sci(significand_value, exponent_value as i64);
        BigFloat {
            handle: new_bf_handle,
            api,
        }
    }
}

impl<M: ManagedTypeApi> BigFloat<M> {
    #[inline]
    pub fn zero(api: M) -> Self {
        BigFloat {
            handle: api.bf_new_zero(),
            api,
        }
    }

    #[inline]
    pub fn from_buffer(managed_buffer: &ManagedBuffer<M>) -> Self {
        BigFloat {
            handle: managed_buffer.api.mb_to_big_float(managed_buffer.handle),
            api: managed_buffer.api.clone(),
        }
    }

    #[inline]
    pub fn to_buffer(&self) -> ManagedBuffer<M> {
        ManagedBuffer {
            handle: self.api.mb_from_big_float(self.handle),
            api: self.api.clone(),
        }
    }
}

impl<M: ManagedTypeApi> BigFloat<M> {
    #[inline]
    pub fn sqrt(&self) -> Self {
        let new_handle = self.api.bf_new_zero();
        self.api.bf_sqrt(new_handle, self.handle);
        BigFloat {
            handle: new_handle,
            api: self.api.clone(),
        }
    }

    pub fn pow(&self, exp: u32) -> Self {
        let new_handle = self.api.bf_new_zero();
        self.api.bf_pow(new_handle, self.handle, exp as i32);
        BigFloat {
            handle: new_handle,
            api: self.api.clone(),
        }
    }

    /// Returns the sign of the `BigFloat` as a `Sign`.
    pub fn sign(&self) -> Sign {
        match self.api.bf_sign(self.handle) {
            crate::api::Sign::Plus => Sign::Plus,
            crate::api::Sign::NoSign => Sign::NoSign,
            crate::api::Sign::Minus => Sign::Minus,
        }
    }

    /// Returns the magnitude of the `BigFloat`
    pub fn magnitude(&self) -> BigFloat<M> {
        let result = self.api.bf_new_zero();
        self.api.bf_abs(result, self.handle);
        BigFloat {
            handle: result,
            api: self.api.clone(),
        }
    }

    /// Convert this `BigFloat` into its `Sign` and its magnitude,
    /// the reverse of `BigInt::from_biguint`.
    pub fn to_parts(self) -> (Sign, BigFloat<M>) {
        (self.sign(), self.magnitude())
    }
}

impl<M: ManagedTypeApi> Clone for BigFloat<M> {
    fn clone(&self) -> Self {
        let new_handle = self.api.bf_new_zero();
        self.api.bf_clone(new_handle, self.handle);
        BigFloat {
            handle: new_handle,
            api: self.api.clone(),
        }
    }
}

impl<M: ManagedTypeApi> TryStaticCast for BigFloat<M> {}

impl<M: ManagedTypeApi> TopEncode for BigFloat<M> {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.to_buffer().top_encode(output)
    }
}

impl<M: ManagedTypeApi> TopDecode for BigFloat<M> {
    #[inline]
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        Ok(BigFloat::from(ManagedBuffer::top_decode(input)?))
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        BigFloat::from(ManagedBuffer::top_decode_or_exit(input, c, exit))
    }
}

impl<M: ManagedTypeApi> NestedEncode for BigFloat<M> {
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.to_buffer().dep_encode(dest)
    }
}

impl<M: ManagedTypeApi> NestedDecode for BigFloat<M> {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(BigFloat::from(ManagedBuffer::dep_decode(input)?))
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        BigFloat::from(ManagedBuffer::dep_decode_or_exit(input, c, exit))
    }
}

impl<M: ManagedTypeApi> crate::abi::TypeAbi for BigFloat<M> {
    fn type_name() -> String {
        String::from("BigFloat")
    }
}
