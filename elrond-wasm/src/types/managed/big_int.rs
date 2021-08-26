use super::ManagedBuffer;
use crate::api::{Handle, ManagedTypeApi};
use crate::types::BoxedBytes;
use alloc::string::String;
use elrond_codec::{
    DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, TypeInfo,
};

#[derive(Debug)]
pub struct BigInt<M: ManagedTypeApi> {
    pub(super) handle: Handle,
    pub(super) api: M,
}

// BigInt sign.
#[allow(clippy::enum_variant_names)]
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

impl<M: ManagedTypeApi> TopEncode for BigInt<M> {
    const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        output.set_specialized(&self.to_signed_bytes_be_buffer(), || {
            self.to_signed_bytes_be().into_box()
        });
        Ok(())
    }
}

impl<M: ManagedTypeApi> NestedEncode for BigInt<M> {
    const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        if dest.push_specialized(&self.to_signed_bytes_be_buffer()) {
            Ok(())
        } else {
            self.to_signed_bytes_be().dep_encode(dest)
        }
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        if !dest.push_specialized(&self.to_signed_bytes_be_buffer()) {
            self.to_signed_bytes_be().dep_encode_or_exit(dest, c, exit);
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
