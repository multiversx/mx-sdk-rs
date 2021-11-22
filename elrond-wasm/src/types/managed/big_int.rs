use super::{BigUint, ManagedBuffer, ManagedDefault, ManagedFrom, ManagedType, Sign};
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
    pub(crate) api: M,
}

impl<M: ManagedTypeApi> ManagedType<M> for BigInt<M> {
    #[doc(hidden)]
    fn from_raw_handle(api: M, raw_handle: Handle) -> Self {
        BigInt {
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

impl<M: ManagedTypeApi> ManagedDefault<M> for BigInt<M> {
    #[inline]
    fn managed_default(api: M) -> Self {
        Self::from_i64(api, 0)
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

impl<M, U> ManagedFrom<M, U> for BigInt<M>
where
    M: ManagedTypeApi,
    U: Into<i64>,
{
    fn managed_from(api: M, value: U) -> Self {
        BigInt {
            handle: api.bi_new(value.into()),
            api,
        }
    }
}

/// More conversions here.
impl<M: ManagedTypeApi> BigInt<M> {
    #[inline]
    pub fn zero(api: M) -> Self {
        BigInt {
            handle: api.bi_new_zero(),
            api,
        }
    }

    #[inline]
    pub fn from_i64(api: M, value: i64) -> Self {
        BigInt {
            handle: api.bi_new(value),
            api,
        }
    }

    #[inline]
    pub fn from_i32(api: M, value: i32) -> Self {
        BigInt {
            handle: api.bi_new(value as i64),
            api,
        }
    }

    #[inline]
    pub fn to_i64(&self) -> Option<i64> {
        self.api.bi_to_i64(self.handle)
    }

    #[inline]
    pub fn from_signed_bytes_be(api: M, bytes: &[u8]) -> Self {
        let handle = api.bi_new(0);
        api.bi_set_signed_bytes(handle, bytes);
        BigInt { handle, api }
    }

    #[inline]
    pub fn to_signed_bytes_be(&self) -> BoxedBytes {
        self.api.bi_get_signed_bytes(self.handle)
    }

    #[inline]
    pub fn from_signed_bytes_be_buffer(managed_buffer: &ManagedBuffer<M>) -> Self {
        BigInt {
            handle: managed_buffer
                .api
                .mb_to_big_int_signed(managed_buffer.handle),
            api: managed_buffer.api.clone(),
        }
    }

    #[inline]
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
    pub fn from_biguint(sign: Sign, unsigned: BigUint<M>) -> Self {
        let api = M::instance();
        if sign.is_minus() {
            api.bi_neg(unsigned.handle, unsigned.handle);
        }
        BigInt {
            handle: unsigned.handle,
            api,
        }
    }

    /// Returns the sign of the `BigInt` as a `Sign`.
    pub fn sign(&self) -> Sign {
        match self.api.bi_sign(self.handle) {
            crate::api::Sign::Plus => Sign::Plus,
            crate::api::Sign::NoSign => Sign::NoSign,
            crate::api::Sign::Minus => Sign::Minus,
        }
    }

    /// Returns the magnitude of the `BigInt` as a `BigUint`.
    pub fn magnitude(&self) -> BigUint<M> {
        let result = self.api.bi_new_zero();
        self.api.bi_abs(result, self.handle);
        BigUint::from_raw_handle_no_api(result)
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
            Some(BigUint::from_raw_handle_no_api(self.handle))
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
        let handle = self.api.bi_new_zero();
        let exp_handle = self.api.bi_new(exp as i64);
        self.api.bi_pow(handle, self.handle, exp_handle);
        BigInt {
            handle,
            api: self.api.clone(),
        }
    }
}
