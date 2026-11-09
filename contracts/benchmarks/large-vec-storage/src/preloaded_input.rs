use multiversx_sc::{
    api::ManagedTypeApi,
    codec::{
        DecodeError, DecodeErrorHandler, NestedDecodeInput, TryStaticCast, try_execute_then_cast,
    },
    types::{BigInt, BigUint, ManagedBuffer},
};

const CACHE_SIZE: usize = 1 << 17;

static mut CACHE: [u8; CACHE_SIZE] = [0u8; CACHE_SIZE];

/// Copies the whole managed buffer into wasm memory, with a single VM call.
pub fn preload_bytes<M: ManagedTypeApi>(buffer: &ManagedBuffer<M>) -> &'static [u8] {
    let len = buffer.len();
    if len > CACHE_SIZE {
        multiversx_sc::contract_base::ErrorHelper::<M>::signal_error_with_message(
            "preload cache too small",
        );
    }
    unsafe {
        let cache = &mut *core::ptr::addr_of_mut!(CACHE);
        let dest = &mut cache[..len];
        buffer.load_slice(0, dest);
        dest
    }
}

/// Prototype nested decode input, with the bytes already loaded in wasm memory.
///
/// - `FROM_BYTES = false`: nested managed types are copied via `mBufferCopyByteSlice` (pre-#1964 behaviour);
/// - `FROM_BYTES = true`: nested managed types are created from the preloaded bytes.
pub struct PreloadedNestedDecodeInput<'a, M, const FROM_BYTES: bool>
where
    M: ManagedTypeApi,
{
    buffer: &'a ManagedBuffer<M>,
    bytes: &'a [u8],
    index: usize,
}

impl<'a, M, const FROM_BYTES: bool> PreloadedNestedDecodeInput<'a, M, FROM_BYTES>
where
    M: ManagedTypeApi,
{
    pub fn new(buffer: &'a ManagedBuffer<M>, bytes: &'a [u8]) -> Self {
        PreloadedNestedDecodeInput {
            buffer,
            bytes,
            index: 0,
        }
    }

    fn take<H: DecodeErrorHandler>(&mut self, size: usize, h: H) -> Result<usize, H::HandledErr> {
        if size > self.remaining_len() {
            return Err(h.handle_error(DecodeError::INPUT_TOO_SHORT));
        }
        let start = self.index;
        self.index += size;
        Ok(start)
    }

    fn read_size<H: DecodeErrorHandler>(&mut self, h: H) -> Result<usize, H::HandledErr> {
        let mut arr = [0u8; 4];
        self.read_into(&mut arr, h)?;
        Ok(u32::from_be_bytes(arr) as usize)
    }

    fn read_managed_buffer<H: DecodeErrorHandler>(
        &mut self,
        h: H,
    ) -> Result<ManagedBuffer<M>, H::HandledErr> {
        let size = self.read_size(h)?;
        let start = self.take(size, h)?;
        if FROM_BYTES {
            Ok(ManagedBuffer::new_from_bytes(
                &self.bytes[start..start + size],
            ))
        } else {
            Ok(self.buffer.copy_slice(start, size).unwrap())
        }
    }

    fn read_big_uint<H: DecodeErrorHandler>(&mut self, h: H) -> Result<BigUint<M>, H::HandledErr> {
        if FROM_BYTES {
            let size = self.read_size(h)?;
            let start = self.take(size, h)?;
            Ok(BigUint::from_bytes_be(&self.bytes[start..start + size]))
        } else {
            Ok(BigUint::from_bytes_be_buffer(&self.read_managed_buffer(h)?))
        }
    }

    fn read_big_int<H: DecodeErrorHandler>(&mut self, h: H) -> Result<BigInt<M>, H::HandledErr> {
        Ok(BigInt::from_signed_bytes_be_buffer(
            &self.read_managed_buffer(h)?,
        ))
    }
}

impl<M, const FROM_BYTES: bool> NestedDecodeInput for PreloadedNestedDecodeInput<'_, M, FROM_BYTES>
where
    M: ManagedTypeApi,
{
    fn remaining_len(&self) -> usize {
        self.bytes.len() - self.index
    }

    fn peek_into<H>(&mut self, into: &mut [u8], h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        if into.len() > self.remaining_len() {
            return Err(h.handle_error(DecodeError::INPUT_TOO_SHORT));
        }
        into.copy_from_slice(&self.bytes[self.index..self.index + into.len()]);
        Ok(())
    }

    fn read_into<H>(&mut self, into: &mut [u8], h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        self.peek_into(into, h)?;
        self.index += into.len();
        Ok(())
    }

    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        T::type_eq::<ManagedBuffer<M>>() || T::type_eq::<BigUint<M>>() || T::type_eq::<BigInt<M>>()
    }

    fn read_specialized<T, C, H>(&mut self, _context: C, h: H) -> Result<T, H::HandledErr>
    where
        T: TryStaticCast,
        C: TryStaticCast,
        H: DecodeErrorHandler,
    {
        if let Some(result) = try_execute_then_cast(|| self.read_managed_buffer(h)) {
            result
        } else if let Some(result) = try_execute_then_cast(|| self.read_big_uint(h)) {
            result
        } else if let Some(result) = try_execute_then_cast(|| self.read_big_int(h)) {
            result
        } else {
            Err(h.handle_error(DecodeError::UNSUPPORTED_OPERATION))
        }
    }
}
