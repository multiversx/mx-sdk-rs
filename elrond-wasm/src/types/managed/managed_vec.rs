use super::{ManagedBuffer, ManagedType, ManagedVecItem, ManagedVecIterator};
use crate::{
    abi::TypeAbi,
    api::{Handle, ManagedTypeApi},
    types::{ArgBuffer, BoxedBytes, ManagedBufferNestedDecodeInput},
};
use alloc::{string::String, vec::Vec};
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
    T: ManagedVecItem,
{
    pub(crate) buffer: ManagedBuffer<M>,
    _phantom: PhantomData<T>,
}

impl<M, T> ManagedType<M> for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    #[inline]
    fn from_raw_handle(handle: Handle) -> Self {
        ManagedVec {
            buffer: ManagedBuffer::from_raw_handle(handle),
            _phantom: PhantomData,
        }
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.buffer.get_raw_handle()
    }
}

impl<M, T> ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    #[inline]
    pub fn new() -> Self {
        ManagedVec {
            buffer: ManagedBuffer::new(),
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
}

impl<M, T, I> From<Vec<I>> for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
    I: Into<T>,
{
    fn from(v: Vec<I>) -> Self {
        let mut result = Self::new();
        for item in v.into_iter() {
            result.push(item.into());
        }
        result
    }
}

impl<M, T> Default for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<M, T> ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
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

    /// Retrieves element at index, if the index is valid.
    /// Warning! Ownership around this method is murky, managed items are copied without respecting ownership.
    /// TODO: Find a way to fix it by returning some kind of reference to the item, not the owned type.
    pub fn get(&self, index: usize) -> Option<T> {
        let byte_index = index * T::PAYLOAD_SIZE;
        let mut load_result = Ok(());
        let result = T::from_byte_reader(|dest_slice| {
            load_result = self.buffer.load_slice(byte_index, dest_slice);
        });
        match load_result {
            Ok(_) => Some(result),
            Err(_) => None,
        }
    }

    pub fn slice(&self, start_index: usize, end_index: usize) -> Option<Self> {
        let byte_start = start_index * T::PAYLOAD_SIZE;
        let byte_end = end_index * T::PAYLOAD_SIZE;
        let opt_buffer = self.buffer.copy_slice(byte_start, byte_end - byte_start);
        opt_buffer.map(ManagedVec::new_from_raw_buffer)
    }

    pub fn push(&mut self, item: T) {
        item.to_byte_writer(|bytes| {
            self.buffer.append_bytes(bytes);
        });
    }

    /// New `ManagedVec` instance with 1 element in it.
    pub fn from_single_item(item: T) -> Self {
        let mut result = ManagedVec::new();
        result.push(item);
        result
    }

    pub fn overwrite_with_single_item(&mut self, item: T) {
        item.to_byte_writer(|bytes| {
            self.buffer.overwrite(bytes);
        });
    }

    /// Appends all the contents of another managed vec at the end of the current one.
    /// Consumes the other vec in the process.
    pub fn append_vec(&mut self, item: ManagedVec<M, T>) {
        self.buffer.append(&item.buffer);
    }

    /// Removes all items while retaining the handle.
    pub fn clear(&mut self) {
        self.buffer.overwrite(&[]);
    }

    pub fn into_vec(self) -> Vec<T> {
        let mut v = Vec::new();
        for item in self.into_iter() {
            v.push(item);
        }
        v
    }

    /// Temporarily converts self to a `Vec<T>`.
    /// All operations performed on the temporary vector get saved back to the underlying buffer.
    pub fn with_self_as_vec<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Vec<T>),
    {
        let new = ManagedVec::new();
        let old = core::mem::replace(self, new);
        let mut temp_vec = Vec::new();
        for item in old.into_iter() {
            temp_vec.push(item);
        }
        f(&mut temp_vec);
        for new_item in temp_vec {
            self.push(new_item);
        }
    }

    pub fn iter(&self) -> ManagedVecIterator<M, T> {
        ManagedVecIterator::new(self)
    }
}

impl<M, T> Clone for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + Clone,
{
    fn clone(&self) -> Self {
        let mut result = ManagedVec::new();
        for item in self.into_iter() {
            result.push(item.clone())
        }
        result
    }
}

impl<M, T> PartialEq for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.buffer == other.buffer {
            return true;
        }
        let self_len = self.buffer.byte_len();
        let other_len = other.buffer.byte_len();
        if self_len != other_len {
            return false;
        }
        let mut byte_index = 0;
        while byte_index < self_len {
            let self_item = T::from_byte_reader(|dest_slice| {
                let _ = self.buffer.load_slice(byte_index, dest_slice);
            });
            let other_item = T::from_byte_reader(|dest_slice| {
                let _ = other.buffer.load_slice(byte_index, dest_slice);
            });
            if self_item != other_item {
                return false;
            }
            byte_index += T::PAYLOAD_SIZE;
        }
        true
    }
}

impl<M, T> Eq for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + PartialEq,
{
}

impl<M, T> TopEncode for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + NestedEncode,
{
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        if T::SKIPS_RESERIALIZATION {
            self.buffer.top_encode(output)
        } else {
            let mut nested_buffer = output.start_nested_encode();
            for item in self {
                item.dep_encode(&mut nested_buffer)?;
            }
            output.finalize_nested_encode(nested_buffer);
            Ok(())
        }
    }
}

impl<M, T> NestedEncode for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + NestedEncode,
{
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.len().dep_encode(dest)?;
        for item in self {
            item.dep_encode(dest)?;
        }
        Ok(())
    }
}

impl<M, T> TopDecode for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + NestedDecode,
{
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        let buffer = ManagedBuffer::top_decode(input)?;
        if T::SKIPS_RESERIALIZATION {
            Ok(ManagedVec::new_from_raw_buffer(buffer))
        } else {
            let mut result = ManagedVec::new();
            let mut nested_de_input = ManagedBufferNestedDecodeInput::new(buffer);
            while nested_de_input.remaining_len() > 0 {
                result.push(T::dep_decode(&mut nested_de_input)?);
            }
            Ok(result)
        }
    }
}

impl<M, T> NestedDecode for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + NestedDecode,
{
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        let size = usize::dep_decode(input)?;
        let mut result = ManagedVec::new();
        for _ in 0..size {
            result.push(T::dep_decode(input)?);
        }
        Ok(result)
    }
}

impl<M, T> TypeAbi for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + TypeAbi,
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

pub fn managed_vec_from_slice_of_boxed_bytes<M: ManagedTypeApi>(
    _api: M,
    data: &[BoxedBytes],
) -> ManagedVec<M, ManagedBuffer<M>> {
    let mut result = ManagedVec::new();
    for boxed_bytes in data {
        result.push(ManagedBuffer::new_from_bytes(boxed_bytes.as_slice()));
    }
    result
}
