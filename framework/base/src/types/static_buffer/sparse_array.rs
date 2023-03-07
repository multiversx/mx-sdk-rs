use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::{ErrorApi, ErrorApiImpl},
    codec::{self, arrayvec::ArrayVec, NestedDecode, NestedEncode, TopDecode, TopEncode},
};
use core::marker::PhantomData;

const EMPTY_ENTRY: usize = 0;
static INVALID_INDEX_ERR_MSG: &[u8] = b"Index out of bounds";

/// A special type of array that initially holds the values from 0 to N
/// If array[i] == i, then the default value (0) is stored instead
#[derive(Clone)]
pub struct SparseArray<E, const CAPACITY: usize>
where
    E: ErrorApi,
{
    array: [usize; CAPACITY],
    len: usize,
    _phantom: PhantomData<E>,
}

impl<E, const CAPACITY: usize> SparseArray<E, CAPACITY>
where
    E: ErrorApi,
{
    /// initializes a sparse array that holds the values from range [0, len)
    pub fn new(len: usize) -> Self {
        if len > CAPACITY {
            E::error_api_impl().signal_error(b"Length exceeds capacity");
        }

        SparseArray {
            array: [0usize; CAPACITY],
            len,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the underlying array as a slice, without converting 0-values to their actual value
    #[inline]
    pub fn as_raw_slice(&self) -> &[usize] {
        &self.array[..self.len]
    }

    /// Gets the value at the given `index`.
    /// If the value is 0, then `index` is returned.
    pub fn get(&self, index: usize) -> usize {
        self.require_valid_index(index);
        self.get_item_unchecked(index)
    }

    /// Sets the value at the given `index`.
    /// If the `index` and `value` are equal, then `0` is stored.
    pub fn set(&mut self, index: usize, value: usize) {
        self.require_valid_index(index);
        self.set_item_unchecked(index, value);
    }

    /// Removes the value at the given index.
    /// The value at `index` is set to the last item in the array
    /// and length is decremented
    pub fn swap_remove(&mut self, index: usize) -> usize {
        self.require_valid_index(index);

        let last_item_index = self.len - 1;
        let last_item = self.get_item_unchecked(last_item_index);

        let current_item = if index != last_item_index {
            let item_at_index = self.get_item_unchecked(index);
            self.set_item_unchecked(index, last_item);

            item_at_index
        } else {
            last_item
        };

        self.set_item_unchecked(last_item_index, EMPTY_ENTRY);
        self.len -= 1;

        current_item
    }

    fn get_item_unchecked(&self, index: usize) -> usize {
        let value = self.array[index];
        if value == EMPTY_ENTRY {
            index
        } else {
            value
        }
    }

    fn set_item_unchecked(&mut self, index: usize, value: usize) {
        if index == value {
            self.array[index] = EMPTY_ENTRY;
        } else {
            self.array[index] = value;
        }
    }

    fn require_valid_index(&self, index: usize) {
        if index >= self.len {
            E::error_api_impl().signal_error(INVALID_INDEX_ERR_MSG);
        }
    }

    pub fn iter(&self) -> SparseArrayIterator<E, CAPACITY> {
        SparseArrayIterator::new(self)
    }
}

impl<'a, E, const CAPACITY: usize> IntoIterator for &'a SparseArray<E, CAPACITY>
where
    E: ErrorApi,
{
    type Item = usize;

    type IntoIter = SparseArrayIterator<'a, E, CAPACITY>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct SparseArrayIterator<'a, E, const CAPACITY: usize>
where
    E: ErrorApi,
{
    array_ref: &'a SparseArray<E, CAPACITY>,
    current_index: usize,
    last_index: usize,
}

impl<'a, E, const CAPACITY: usize> SparseArrayIterator<'a, E, CAPACITY>
where
    E: ErrorApi,
{
    pub fn new(array: &'a SparseArray<E, CAPACITY>) -> Self {
        Self {
            array_ref: array,
            current_index: 0,
            last_index: array.len - 1,
        }
    }
}

impl<'a, E, const CAPACITY: usize> Iterator for SparseArrayIterator<'a, E, CAPACITY>
where
    E: ErrorApi,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next_index = self.current_index;
        if next_index > self.last_index {
            return None;
        }

        self.current_index += 1;

        Some(self.array_ref.get(next_index))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.last_index - self.current_index + 1;
        (remaining, Some(remaining))
    }
}

impl<'a, E, const CAPACITY: usize> ExactSizeIterator for SparseArrayIterator<'a, E, CAPACITY> where
    E: ErrorApi
{
}

impl<'a, E, const CAPACITY: usize> DoubleEndedIterator for SparseArrayIterator<'a, E, CAPACITY>
where
    E: ErrorApi,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let next_index = self.last_index;
        if next_index < self.current_index {
            return None;
        }

        self.last_index -= 1;

        Some(self.array_ref.get(next_index))
    }
}

impl<'a, E, const CAPACITY: usize> Clone for SparseArrayIterator<'a, E, CAPACITY>
where
    E: ErrorApi,
{
    fn clone(&self) -> Self {
        Self {
            array_ref: self.array_ref,
            current_index: self.current_index,
            last_index: self.last_index,
        }
    }
}

impl<E, const CAPACITY: usize> TopEncode for SparseArray<E, CAPACITY>
where
    E: ErrorApi,
{
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: codec::TopEncodeOutput,
        H: codec::EncodeErrorHandler,
    {
        let mut nested_buffer = output.start_nested_encode();
        for item in self.iter() {
            item.dep_encode_or_handle_err(&mut nested_buffer, h)?;
        }
        output.finalize_nested_encode(nested_buffer);

        Ok(())
    }
}

impl<E, const CAPACITY: usize> NestedEncode for SparseArray<E, CAPACITY>
where
    E: ErrorApi,
{
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: codec::NestedEncodeOutput,
        H: codec::EncodeErrorHandler,
    {
        self.len.dep_encode_or_handle_err(dest, h)?;
        for item in self.iter() {
            item.dep_encode_or_handle_err(dest, h)?;
        }

        Ok(())
    }
}

impl<E, const CAPACITY: usize> TopDecode for SparseArray<E, CAPACITY>
where
    E: ErrorApi,
{
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: codec::TopDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        match ArrayVec::<usize, CAPACITY>::top_decode(input) {
            Ok(array_vec) => {
                let len = array_vec.len();
                let mut array = [0usize; CAPACITY];
                let array_slice = &mut array[..len];
                array_slice.copy_from_slice(array_vec.as_slice());

                Ok(Self {
                    array,
                    len: array_vec.len(),
                    _phantom: PhantomData,
                })
            },
            Err(e) => Err(h.handle_error(e)),
        }
    }
}

impl<E, const CAPACITY: usize> NestedDecode for SparseArray<E, CAPACITY>
where
    E: ErrorApi,
{
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: codec::NestedDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        match ArrayVec::<usize, CAPACITY>::dep_decode(input) {
            Ok(array_vec) => {
                let len = array_vec.len();
                let mut array = [0usize; CAPACITY];
                let array_slice = &mut array[..len];
                array_slice.copy_from_slice(array_vec.as_slice());

                Ok(Self {
                    array,
                    len: array_vec.len(),
                    _phantom: PhantomData,
                })
            },
            Err(e) => Err(h.handle_error(e)),
        }
    }
}

impl<E, const CAPACITY: usize> TypeAbi for SparseArray<E, CAPACITY>
where
    E: ErrorApi,
{
    /// It is semantically equivalent to any list of `usize`.
    fn type_name() -> TypeName {
        <&[usize] as TypeAbi>::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        usize::provide_type_descriptions(accumulator);
    }
}
