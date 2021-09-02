use super::{ManagedBuffer, ManagedType, ManagedVecItem};
use crate::{
    abi::TypeAbi,
    api::{Handle, ManagedTypeApi},
    types::{ArgBuffer, ManagedBufferNestedDecodeInput},
};
use alloc::string::String;
use core::marker::PhantomData;
use elrond_codec::{
    DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

/// A list of items that lives inside a managed buffer.
/// Items can be either stored there in full (e.g. `u32`),
/// or just via handle (e.g. `BigUint<M>`).
#[derive(Debug)]
pub struct ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    pub(crate) buffer: ManagedBuffer<M>,
    _phantom: PhantomData<T>,
}

impl<M, T> ManagedType<M> for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    #[inline]
    fn from_raw_handle(api: M, handle: Handle) -> Self {
        ManagedVec {
            buffer: ManagedBuffer::from_raw_handle(api, handle),
            _phantom: PhantomData,
        }
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.buffer.get_raw_handle()
    }

    #[inline]
    fn type_manager(&self) -> M {
        self.buffer.type_manager()
    }
}

impl<M, T> ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    #[inline]
    pub fn new_empty(api: M) -> Self {
        ManagedVec {
            buffer: ManagedBuffer::new_empty(api),
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn new_from_raw_buffer(buffer: ManagedBuffer<M>) -> Self {
        ManagedVec {
            buffer,
            _phantom: PhantomData,
        }
    }

    /// Length of the underlying buffer in bytes.
    #[inline]
    pub fn byte_len(&self) -> usize {
        self.buffer.len()
    }

    /// Number of items.
    #[inline]
    pub fn len(&self) -> usize {
        self.byte_len() / T::PAYLOAD_SIZE
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.byte_len() == 0
    }

    pub fn push(&mut self, item: T) {
        item.to_byte_writer(|bytes| {
            self.buffer.append_bytes(bytes);
        });
    }

    pub fn get(&self, index: usize) -> Option<T> {
        let byte_index = index * T::PAYLOAD_SIZE;
        let mut load_result = Ok(());
        let result = T::from_byte_reader(self.type_manager(), |dest_slice| {
            load_result = self.buffer.load_slice(byte_index, dest_slice);
        });
        match load_result {
            Ok(_) => Some(result),
            Err(_) => None,
        }
    }
}

impl<M, T> TopEncode for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        if T::NEEDS_RESERIALIZATION {
            let mut nested_buffer = output.start_nested_encode();
            for item in self {
                item.dep_encode(&mut nested_buffer)?;
            }
            output.finalize_nested_encode(nested_buffer);
            Ok(())
        } else {
            self.buffer.top_encode(output)
        }
    }
}

impl<M, T> NestedEncode for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        if T::NEEDS_RESERIALIZATION {
            self.len().dep_encode(dest)?;
            for item in self {
                item.dep_encode(dest)?;
            }
            Ok(())
        } else {
            self.buffer.dep_encode(dest)?;
            Ok(())
        }
    }
}

impl<M, T> TopDecode for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        let buffer = ManagedBuffer::top_decode(input)?;
        if T::NEEDS_RESERIALIZATION {
            let mut result = ManagedVec::new_empty(buffer.type_manager());
            let mut nested_de_input = ManagedBufferNestedDecodeInput::new(buffer);
            while nested_de_input.remaining_len() > 0 {
                result.push(T::dep_decode(&mut nested_de_input)?);
            }
            Ok(result)
        } else {
            Ok(ManagedVec::new_from_raw_buffer(buffer))
        }
    }
}

impl<M, T> NestedDecode for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    fn dep_decode<I: NestedDecodeInput>(_input: &mut I) -> Result<Self, DecodeError> {
        // TODO: this is more complex and requires more specialization
        // not immediately needed
        Err(DecodeError::UNSUPPORTED_OPERATION)
    }
}

impl<M, T> TypeAbi for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    /// It is semantically equivalent to any list of `T`.
    fn type_name() -> String {
        <&[T] as TypeAbi>::type_name()
    }
}

/// For compatibility with the older Arwen EI.
pub fn managed_vec_of_buffers_to_arg_buffer<M: ManagedTypeApi>(
    managed_vec: &ManagedVec<M, ManagedBuffer<M>>,
) -> ArgBuffer {
    let mut arg_buffer = ArgBuffer::new();
    for buffer in managed_vec {
        arg_buffer.push_argument_bytes(buffer.to_boxed_bytes().as_slice());
    }
    arg_buffer
}
