use alloc::string::String;

use crate::api::{Handle, ManagedTypeApi};

use super::ManagedBuffer;
use crate::types::BoxedBytes;

#[derive(Debug)]
pub struct BigUint<M: ManagedTypeApi> {
    pub(super) handle: Handle,
    pub(super) api: M,
}

impl<M: ManagedTypeApi> From<&ManagedBuffer<M>> for BigUint<M> {
    fn from(item: &ManagedBuffer<M>) -> Self {
        BigUint::from_bytes_be_buffer(item)
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for BigUint<M> {
    fn from(item: ManagedBuffer<M>) -> Self {
        BigUint::from_bytes_be_buffer(&item)
    }
}

/// More conversions here.
impl<M: ManagedTypeApi> BigUint<M> {
    pub fn from_u64(value: u64, api: M) -> Self {
        BigUint {
            handle: api.bi_new(value as i64),
            api,
        }
    }

    pub fn from_u32(value: u32, api: M) -> Self {
        BigUint {
            handle: api.bi_new(value as i64),
            api,
        }
    }

    pub fn to_u64(&self) -> Option<u64> {
        self.api.bi_to_i64(self.handle).map(|bi| bi as u64)
    }

    pub fn from_bytes_be(bytes: &[u8], api: M) -> Self {
        let handle = api.bi_new(0);
        api.bi_set_unsigned_bytes(handle, bytes);
        BigUint { handle, api }
    }

    pub fn to_bytes_be(&self) -> BoxedBytes {
        self.api.bi_get_unsigned_bytes(self.handle)
    }

    pub fn from_bytes_be_buffer(managed_buffer: &ManagedBuffer<M>) -> Self {
        BigUint {
            handle: managed_buffer
                .api
                .mb_to_big_int_unsigned(managed_buffer.handle),
            api: managed_buffer.api.clone(),
        }
    }

    pub fn to_bytes_be_buffer(&self) -> ManagedBuffer<M> {
        ManagedBuffer {
            handle: self.api.mb_from_big_int_unsigned(self.handle),
            api: self.api.clone(),
        }
    }
}

impl<M: ManagedTypeApi> BigUint<M> {
    pub fn sqrt(&self) -> Self {
        let handle = self.api.bi_new_zero();
        self.api.bi_sqrt(handle, self.handle);
        BigUint {
            handle,
            api: self.api.clone(),
        }
    }

    pub fn pow(&self, exp: u32) -> Self {
        let handle = self.api.bi_new_zero();
        let exp_handle = self.api.bi_new(exp as i64);
        self.api.bi_pow(handle, self.handle, exp_handle);
        BigUint {
            handle,
            api: self.api.clone(),
        }
    }

    pub fn log2(&self) -> u32 {
        self.api.bi_log2(self.handle)
    }
}

impl<M: ManagedTypeApi> Clone for BigUint<M> {
    fn clone(&self) -> Self {
        let clone_handle = self.api.bi_new_zero();
        self.api.bi_add(clone_handle, clone_handle, self.handle);
        BigUint {
            handle: clone_handle,
            api: self.api.clone(),
        }
    }
}

use crate::elrond_codec::*;

impl<M: ManagedTypeApi> TopEncode for BigUint<M> {
    const TYPE_INFO: TypeInfo = TypeInfo::BigUint;

    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        if !output.set_specialized(&self.to_bytes_be_buffer()) {
            output.set_slice_u8(self.to_bytes_be().as_slice());
        }
        Ok(())
    }
}

impl<M: ManagedTypeApi> NestedEncode for BigUint<M> {
    const TYPE_INFO: TypeInfo = TypeInfo::BigUint;

    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        if !dest.push_specialized(&self.to_bytes_be_buffer()) {
            dest.write(self.to_bytes_be().as_slice());
        }
        Ok(())
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        _c: ExitCtx,
        _exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        if !dest.push_specialized(&self.to_bytes_be_buffer()) {
            dest.write(self.to_bytes_be().as_slice());
        }
    }
}

impl<M: ManagedTypeApi> NestedDecode for BigUint<M> {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        if let Some(managed_buffer) = input.read_specialized::<ManagedBuffer<M>>()? {
            Ok(BigUint::from_bytes_be_buffer(&managed_buffer))
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
            BigUint::from_bytes_be_buffer(&managed_buffer)
        } else {
            exit(c, DecodeError::UNSUPPORTED_OPERATION)
        }
    }
}

impl<M: ManagedTypeApi> TopDecode for BigUint<M> {
    const TYPE_INFO: TypeInfo = TypeInfo::BigUint;

    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        if let Some(managed_buffer) = input.into_specialized::<ManagedBuffer<M>>() {
            Ok(managed_buffer.into())
        } else {
            Err(DecodeError::UNSUPPORTED_OPERATION)
        }
    }
}

impl<M: ManagedTypeApi> crate::abi::TypeAbi for BigUint<M> {
    fn type_name() -> String {
        String::from("BigUint")
    }
}
