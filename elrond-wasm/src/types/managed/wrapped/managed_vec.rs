use crate::{
    abi::{TypeAbi, TypeDescriptionContainer},
    api::{ErrorApiImpl, Handle, InvalidSliceError, ManagedTypeApi},
    types::{
        ArgBuffer, BoxedBytes, ManagedBuffer, ManagedBufferNestedDecodeInput, ManagedType,
        ManagedVecItem, ManagedVecRef, ManagedVecRefIterator,
    },
};
use alloc::{string::String, vec::Vec};
use core::marker::PhantomData;
use elrond_codec::{
    DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
    NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeMultiOutput,
    TopEncodeOutput,
};

pub(crate) const INDEX_OUT_OF_RANGE_MSG: &[u8] = b"ManagedVec index out of range";

/// A list of items that lives inside a managed buffer.
/// Items can be either stored there in full (e.g. `u32`),
/// or just via handle (e.g. `BigUint<M>`).
#[repr(transparent)]
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

    #[doc(hidden)]
    fn transmute_from_handle_ref(handle_ref: &Handle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
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

    pub fn try_get(&self, index: usize) -> Option<T::Ref<'_>> {
        let byte_index = index * T::PAYLOAD_SIZE;
        let mut load_result = Ok(());
        let result = unsafe {
            T::from_byte_reader_as_borrow(|dest_slice| {
                load_result = self.buffer.load_slice(byte_index, dest_slice);
            })
        };
        match load_result {
            Ok(_) => Some(result),
            Err(_) => None,
        }
    }

    /// Retrieves element at index, if the index is valid.
    /// Otherwise, signals an error and terminates execution.
    pub fn get(&self, index: usize) -> T::Ref<'_> {
        match self.try_get(index) {
            Some(result) => result,
            None => M::error_api_impl().signal_error(INDEX_OUT_OF_RANGE_MSG),
        }
    }

    pub fn get_mut(&mut self, index: usize) -> ManagedVecRef<M, T> {
        ManagedVecRef::new(self.get_raw_handle(), index)
    }

    pub(super) unsafe fn get_unsafe(&self, index: usize) -> T {
        let byte_index = index * T::PAYLOAD_SIZE;
        let mut load_result = Ok(());
        let result = T::from_byte_reader(|dest_slice| {
            load_result = self.buffer.load_slice(byte_index, dest_slice);
        });

        match load_result {
            Ok(_) => result,
            Err(_) => M::error_api_impl().signal_error(INDEX_OUT_OF_RANGE_MSG),
        }
    }

    pub fn set(&mut self, index: usize, item: &T) -> Result<(), InvalidSliceError> {
        let byte_index = index * T::PAYLOAD_SIZE;
        item.to_byte_writer(|slice| self.buffer.set_slice(byte_index, slice))
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

    pub fn remove(&mut self, index: usize) {
        let len = self.len();
        if index >= len {
            M::error_api_impl().signal_error(INDEX_OUT_OF_RANGE_MSG);
        }

        let part_before = if index > 0 {
            match self.slice(0, index) {
                Some(s) => s,
                None => M::error_api_impl().signal_error(INDEX_OUT_OF_RANGE_MSG),
            }
        } else {
            ManagedVec::new()
        };
        let part_after = if index < len {
            match self.slice(index + 1, len) {
                Some(s) => s,
                None => M::error_api_impl().signal_error(INDEX_OUT_OF_RANGE_MSG),
            }
        } else {
            ManagedVec::new()
        };

        self.buffer = part_before.buffer;
        self.buffer.append(&part_after.buffer);
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
    pub fn with_self_as_vec<R, F>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Vec<T>) -> R,
    {
        let new = ManagedVec::new();
        let old = core::mem::replace(self, new);
        let mut temp_vec = Vec::new();
        for item in old.into_iter() {
            temp_vec.push(item);
        }
        let result = f(&mut temp_vec);
        for new_item in temp_vec {
            self.push(new_item);
        }
        result
    }

    pub fn iter(&self) -> ManagedVecRefIterator<M, T> {
        ManagedVecRefIterator::new(self)
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
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        if T::SKIPS_RESERIALIZATION {
            self.buffer.top_encode_or_handle_err(output, h)
        } else {
            let mut nested_buffer = output.start_nested_encode();
            for item in self {
                item.dep_encode_or_handle_err(&mut nested_buffer, h)?;
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
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.len().dep_encode_or_handle_err(dest, h)?;
        for item in self {
            item.dep_encode_or_handle_err(dest, h)?;
        }
        Ok(())
    }
}

impl<M, T> TopDecode for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + NestedDecode,
{
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        let buffer = ManagedBuffer::top_decode_or_handle_err(input, h)?;
        if T::SKIPS_RESERIALIZATION {
            Ok(ManagedVec::new_from_raw_buffer(buffer))
        } else {
            let mut result = ManagedVec::new();
            let mut nested_de_input = ManagedBufferNestedDecodeInput::new(buffer);
            while nested_de_input.remaining_len() > 0 {
                result.push(T::dep_decode_or_handle_err(&mut nested_de_input, h)?);
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
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        let size = usize::dep_decode_or_handle_err(input, h)?;
        let mut result = ManagedVec::new();
        for _ in 0..size {
            result.push(T::dep_decode_or_handle_err(input, h)?);
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

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

/// For compatibility with the older Arwen EI.
pub fn managed_vec_of_buffers_to_arg_buffer<M: ManagedTypeApi>(
    managed_vec: ManagedVec<M, ManagedBuffer<M>>,
) -> ArgBuffer {
    let mut arg_buffer = ArgBuffer::new();
    for buffer in &managed_vec {
        arg_buffer.push_argument_bytes(buffer.to_boxed_bytes().as_slice());
    }
    arg_buffer
}

pub fn managed_vec_from_slice_of_boxed_bytes<M: ManagedTypeApi>(
    data: &[BoxedBytes],
) -> ManagedVec<M, ManagedBuffer<M>> {
    let mut result = ManagedVec::new();
    for boxed_bytes in data {
        result.push(ManagedBuffer::new_from_bytes(boxed_bytes.as_slice()));
    }
    result
}

impl<M, T> core::fmt::Debug for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + core::fmt::Debug + Clone,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut dbg_list = f.debug_list();
        for item in self.into_iter() {
            dbg_list.entry(&item);
        }
        dbg_list.finish()
    }
}

impl<M> TopEncodeMultiOutput for ManagedVec<M, ManagedBuffer<M>>
where
    M: ManagedTypeApi,
{
    fn push_single_value<T, H>(&mut self, arg: &T, h: H) -> Result<(), H::HandledErr>
    where
        T: TopEncode,
        H: EncodeErrorHandler,
    {
        let mut result = ManagedBuffer::new();
        arg.top_encode_or_handle_err(&mut result, h)?;
        self.push(result);
        Ok(())
    }
}
