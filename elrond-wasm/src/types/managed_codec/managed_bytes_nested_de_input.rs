use alloc::boxed::Box;
use elrond_codec::{
    try_execute_then_cast, DecodeError, NestedDecode, NestedDecodeInput,
    OwnedBytesNestedDecodeInput, TryStaticCast,
};

use crate::{
    api::ManagedTypeApi,
    types::{BigInt, BigUint, BoxedBytes, ManagedBuffer},
};

pub struct ManagedBytesNestedDecodeInput<M: ManagedTypeApi> {
    bytes_input: OwnedBytesNestedDecodeInput,
    api: M,
}

impl<M: ManagedTypeApi> ManagedBytesNestedDecodeInput<M> {
    pub fn new(api: M, bytes: Box<[u8]>) -> Self {
        ManagedBytesNestedDecodeInput {
            bytes_input: OwnedBytesNestedDecodeInput::new(bytes),
            api,
        }
    }

    fn read_managed_buffer(&mut self) -> Result<ManagedBuffer<M>, DecodeError> {
        let bytes = BoxedBytes::dep_decode(self)?;
        let result = ManagedBuffer::new_from_bytes(self.api.clone(), bytes.as_slice());
        Ok(result)
    }

    fn read_managed_buffer_or_exit<ExitCtx: Clone>(
        &mut self,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> ManagedBuffer<M> {
        let bytes = BoxedBytes::dep_decode_or_exit(self, c, exit);
        ManagedBuffer::new_from_bytes(self.api.clone(), bytes.as_slice())
    }

    fn read_big_uint(&mut self) -> Result<BigUint<M>, DecodeError> {
        let bytes = BoxedBytes::dep_decode(self)?;
        let result = BigUint::from_bytes_be(self.api.clone(), bytes.as_slice());
        Ok(result)
    }

    fn read_big_uint_or_exit<ExitCtx: Clone>(
        &mut self,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> BigUint<M> {
        let bytes = BoxedBytes::dep_decode_or_exit(self, c, exit);
        BigUint::from_bytes_be(self.api.clone(), bytes.as_slice())
    }

    fn read_big_int(&mut self) -> Result<BigInt<M>, DecodeError> {
        let bytes = BoxedBytes::dep_decode(self)?;
        let result = BigInt::from_signed_bytes_be(self.api.clone(), bytes.as_slice());
        Ok(result)
    }

    fn read_big_int_or_exit<ExitCtx: Clone>(
        &mut self,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> BigInt<M> {
        let bytes = BoxedBytes::dep_decode_or_exit(self, c, exit);
        BigInt::from_signed_bytes_be(self.api.clone(), bytes.as_slice())
    }
}

impl<M: ManagedTypeApi> NestedDecodeInput for ManagedBytesNestedDecodeInput<M> {
    fn remaining_len(&self) -> usize {
        self.bytes_input.remaining_len()
    }

    fn read_into(&mut self, into: &mut [u8]) -> Result<(), DecodeError> {
        self.bytes_input.read_into(into)
    }

    fn read_into_or_exit<ExitCtx: Clone>(
        &mut self,
        into: &mut [u8],
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) {
        self.bytes_input.read_into_or_exit(into, c, exit);
    }

    #[inline]
    fn read_specialized<T, F>(&mut self, else_deser: F) -> Result<T, DecodeError>
    where
        T: TryStaticCast,
        F: FnOnce(&mut Self) -> Result<T, DecodeError>,
    {
        if let Some(result) = try_execute_then_cast(|| self.read_managed_buffer()) {
            result
        } else if let Some(result) = try_execute_then_cast(|| self.read_big_uint()) {
            result
        } else if let Some(result) = try_execute_then_cast(|| self.read_big_int()) {
            result
        } else {
            else_deser(self)
        }
    }

    #[inline]
    fn read_specialized_or_exit<T, ExitCtx, F>(
        &mut self,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
        else_deser: F,
    ) -> T
    where
        T: TryStaticCast,
        F: FnOnce(&mut Self, ExitCtx) -> T,
        ExitCtx: Clone,
    {
        if let Some(result) =
            try_execute_then_cast(|| self.read_managed_buffer_or_exit(c.clone(), exit))
        {
            result
        } else if let Some(result) =
            try_execute_then_cast(|| self.read_big_uint_or_exit(c.clone(), exit))
        {
            result
        } else if let Some(result) =
            try_execute_then_cast(|| self.read_big_int_or_exit(c.clone(), exit))
        {
            result
        } else {
            else_deser(self, c)
        }
    }
}
