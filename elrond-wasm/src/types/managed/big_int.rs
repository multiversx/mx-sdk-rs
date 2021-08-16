use core::cmp::Ordering;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

use alloc::string::String;

use crate::api::ManagedTypeApi;
use crate::types::BoxedBytes;

use super::ManagedBuffer;

#[derive(Debug)]
pub struct BigInt<M: ManagedTypeApi> {
    handle: i32,
    api: M,
}

// BigInt sign.
pub enum Sign {
    Minus,
    NoSign,
    Plus,
}

impl<M: ManagedTypeApi> From<&ManagedBuffer<M>> for BigInt<M> {
    fn from(item: &ManagedBuffer<M>) -> Self {
        BigInt::from_signed_bytes_be_buffer(item)
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for BigInt<M> {
    fn from(item: ManagedBuffer<M>) -> Self {
        BigInt::from_signed_bytes_be_buffer(&item)
    }
}

/// More conversions here.
impl<M: ManagedTypeApi> BigInt<M> {
    pub fn from_i64(value: i64, api: M) -> Self {
        BigInt {
            handle: api.bi_new(value),
            api,
        }
    }

    pub fn from_i32(value: i32, api: M) -> Self {
        BigInt {
            handle: api.bi_new(value as i64),
            api,
        }
    }

    pub fn to_i64(&self) -> Option<i64> {
        self.api.bi_to_i64(self.handle)
    }

    pub fn from_signed_bytes_be(bytes: &[u8], api: M) -> Self {
        let handle = api.bi_new(0);
        api.bi_set_signed_bytes(handle, bytes);
        BigInt { handle, api }
    }

    pub fn to_signed_bytes_be(&self) -> BoxedBytes {
        self.api.bi_get_signed_bytes(self.handle)
    }

    pub fn from_signed_bytes_be_buffer(managed_buffer: &ManagedBuffer<M>) -> Self {
        BigInt {
            handle: managed_buffer
                .api
                .mb_to_big_int_signed(managed_buffer.handle),
            api: managed_buffer.api.clone(),
        }
    }

    pub fn to_signed_bytes_be_buffer(&self) -> ManagedBuffer<M> {
        ManagedBuffer {
            handle: self.api.mb_from_big_int_signed(self.handle),
            api: self.api.clone(),
        }
    }
}

impl<M: ManagedTypeApi> Clone for BigInt<M> {
    fn clone(&self) -> Self {
        let clone_handle = self.api.bi_new_zero();
        self.api.bi_add(clone_handle, clone_handle, self.handle);
        BigInt {
            handle: clone_handle,
            api: self.api.clone(),
        }
    }
}

impl<M: ManagedTypeApi> BigInt<M> {
    // TODO: convert BigUint after new implementation
    pub fn abs_uint(&self) -> Self {
        let result = self.api.bi_new_zero();
        self.api.bi_abs(result, self.handle);
        BigInt {
            handle: result,
            api: self.api.clone(),
        }
    }

    pub fn sign(&self) -> Sign {
        match self.api.bi_sign(self.handle) {
            crate::api::Sign::Plus => Sign::Plus,
            crate::api::Sign::NoSign => Sign::NoSign,
            crate::api::Sign::Minus => Sign::Minus,
        }
    }
}

macro_rules! binary_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $trait for BigInt<M> {
            type Output = BigInt<M>;

            fn $method(self, other: BigInt<M>) -> BigInt<M> {
                self.api.$api_func(self.handle, self.handle, other.handle);
                BigInt {
                    handle: self.handle,
                    api: self.api.clone(),
                }
            }
        }

        impl<'a, 'b, M: ManagedTypeApi> $trait<&'b BigInt<M>> for &'a BigInt<M> {
            type Output = BigInt<M>;

            fn $method(self, other: &BigInt<M>) -> BigInt<M> {
                let result = self.api.bi_new_zero();
                self.api.$api_func(result, self.handle, other.handle);
                BigInt {
                    handle: result,
                    api: self.api.clone(),
                }
            }
        }
    };
}

binary_operator! {Add, add, bi_add}
binary_operator! {Sub, sub, bi_sub}
binary_operator! {Mul, mul, bi_mul}
binary_operator! {Div, div, bi_t_div}
binary_operator! {Rem, rem, bi_t_mod}

macro_rules! binary_assign_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $trait<BigInt<M>> for BigInt<M> {
            #[inline]
            fn $method(&mut self, other: Self) {
                self.api.$api_func(self.handle, self.handle, other.handle);
            }
        }

        impl<M: ManagedTypeApi> $trait<&BigInt<M>> for BigInt<M> {
            #[inline]
            fn $method(&mut self, other: &BigInt<M>) {
                self.api.$api_func(self.handle, self.handle, other.handle);
            }
        }
    };
}

binary_assign_operator! {AddAssign, add_assign, bi_add}
binary_assign_operator! {SubAssign, sub_assign, bi_sub}
binary_assign_operator! {MulAssign, mul_assign, bi_mul}
binary_assign_operator! {DivAssign, div_assign, bi_t_div}
binary_assign_operator! {RemAssign, rem_assign, bi_t_mod}

impl<M: ManagedTypeApi> PartialEq for BigInt<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.api.bi_cmp(self.handle, other.handle).is_eq()
    }
}

impl<M: ManagedTypeApi> Eq for BigInt<M> {}

impl<M: ManagedTypeApi> PartialOrd for BigInt<M> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<M: ManagedTypeApi> Ord for BigInt<M> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.api.bi_cmp(self.handle, other.handle)
    }
}

fn cmp_i64<M: ManagedTypeApi>(bi: &BigInt<M>, other: i64) -> Ordering {
    if other == 0 {
        match bi.api.bi_sign(bi.handle) {
            crate::api::Sign::Plus => Ordering::Greater,
            crate::api::Sign::NoSign => Ordering::Equal,
            crate::api::Sign::Minus => Ordering::Less,
        }
    } else {
        bi.api.bi_cmp(bi.handle, bi.api.bi_new(other))
    }
}

impl<M: ManagedTypeApi> PartialEq<i64> for BigInt<M> {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        cmp_i64(self, *other).is_eq()
    }
}

impl<M: ManagedTypeApi> PartialOrd<i64> for BigInt<M> {
    #[inline]
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        Some(cmp_i64(self, *other))
    }
}

impl<M: ManagedTypeApi> Neg for BigInt<M> {
    type Output = BigInt<M>;

    fn neg(self) -> Self::Output {
        let result = self.api.bi_new_zero();
        self.api.bi_neg(result, self.handle);
        BigInt {
            handle: result,
            api: self.api.clone(),
        }
    }
}

use crate::elrond_codec::*;

impl<M: ManagedTypeApi> TopEncode for BigInt<M> {
    const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        if !output.set_specialized(&self.to_signed_bytes_be_buffer()) {
            output.set_slice_u8(self.to_signed_bytes_be().as_slice());
        }
        Ok(())
    }
}

impl<M: ManagedTypeApi> NestedEncode for BigInt<M> {
    const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        if !dest.push_specialized(&self.to_signed_bytes_be_buffer()) {
            dest.write(self.to_signed_bytes_be().as_slice());
        }
        Ok(())
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        _c: ExitCtx,
        _exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        if !dest.push_specialized(&self.to_signed_bytes_be_buffer()) {
            dest.write(self.to_signed_bytes_be().as_slice());
        }
    }
}

impl<M: ManagedTypeApi> NestedDecode for BigInt<M> {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        if let Some(managed_buffer) = input.read_specialized::<ManagedBuffer<M>>()? {
            Ok(BigInt::from_signed_bytes_be_buffer(&managed_buffer))
        } else {
            Err(DecodeError::UNSUPPORTED_OPERATION)
        }
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        if let Some(managed_buffer) =
            input.read_specialized_or_exit::<ManagedBuffer<M>, ExitCtx>(c.clone(), exit)
        {
            BigInt::from_signed_bytes_be_buffer(&managed_buffer)
        } else {
            exit(c, DecodeError::UNSUPPORTED_OPERATION)
        }
    }
}

impl<M: ManagedTypeApi> TopDecode for BigInt<M> {
    const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        if let Some(managed_buffer) = input.into_specialized::<ManagedBuffer<M>>() {
            Ok(managed_buffer.into())
        } else {
            Err(DecodeError::UNSUPPORTED_OPERATION)
        }
    }
}

impl<M: ManagedTypeApi> crate::abi::TypeAbi for BigInt<M> {
    fn type_name() -> String {
        String::from("BigInt")
    }
}

impl<M: ManagedTypeApi> BigInt<M> {
    pub fn pow(&self, exp: u32) -> Self {
        let handle = self.api.bi_new_zero();
        let exp_handle = self.api.bi_new(exp as i64);
        self.api.bi_pow(handle, self.handle, exp_handle);
        BigInt {
            handle,
            api: self.api.clone(),
        }
    }
}
