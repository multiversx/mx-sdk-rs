use core::marker::PhantomData;

use alloc::{format, vec::Vec};

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName},
    api::{ErrorApi, ManagedTypeApi},
    codec::{
        DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
        TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
    },
    contract_base::ExitCodecErrorHandler,
    err_msg,
    types::{ManagedBuffer, ManagedBufferNestedDecodeInput, ManagedVec, ManagedVecItem},
};

/// A list of items, kept in their encoded form, in a single managed buffer.
///
/// The underlying buffer has the same layout as the top-encoding of a `ManagedVec<T>`,
/// i.e. the nested encodings of all items, concatenated.
///
/// This means that:
/// - saving it to storage, or returning it from an endpoint, is a no-op;
/// - loading it from storage or from an argument does not decode anything;
/// - `push` only encodes the new item and appends it;
/// - items are decoded lazily, one by one, while iterating.
///
/// It is useful for large lists that are mostly appended to, or iterated once,
/// where decoding into a `ManagedVec` and re-encoding would cost a lot of gas.
pub struct ManagedVecEncoded<M, T>
where
    M: ManagedTypeApi,
{
    buffer: ManagedBuffer<M>,
    _phantom: PhantomData<T>,
}

impl<M, T> ManagedVecEncoded<M, T>
where
    M: ManagedTypeApi,
{
    #[inline]
    pub fn new() -> Self {
        Self::from_raw_buffer(ManagedBuffer::new())
    }

    /// Wraps an already encoded buffer. No validation is performed.
    #[inline]
    pub fn from_raw_buffer(buffer: ManagedBuffer<M>) -> Self {
        ManagedVecEncoded {
            buffer,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn as_raw_buffer(&self) -> &ManagedBuffer<M> {
        &self.buffer
    }

    #[inline]
    pub fn into_raw_buffer(self) -> ManagedBuffer<M> {
        self.buffer
    }

    /// True if there are no items.
    ///
    /// The number of items cannot be known without decoding them all,
    /// but an empty list is always encoded as an empty buffer.
    ///
    /// Not reliable for item types whose nested encoding can be empty (e.g. `()`).
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

impl<M, T> ManagedVecEncoded<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: NestedEncode,
{
    /// Encodes the item and appends it to the underlying buffer.
    pub fn push(&mut self, item: T) {
        let Ok(()) = item.dep_encode_or_handle_err(
            &mut self.buffer,
            ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
        );
    }
}

impl<M, T> ManagedVecEncoded<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: NestedDecode,
{
    /// Iterates over the items, decoding them one by one.
    ///
    /// Works on a copy of the underlying buffer.
    /// Use `into_iter` on an owned value to avoid the copy.
    pub fn iter(&self) -> ManagedVecEncodedIterator<M, T> {
        ManagedVecEncodedIterator::new(self.buffer.clone())
    }
}

impl<M, T> Default for ManagedVecEncoded<M, T>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<M, T> Clone for ManagedVecEncoded<M, T>
where
    M: ManagedTypeApi,
{
    fn clone(&self) -> Self {
        Self::from_raw_buffer(self.buffer.clone())
    }
}

/// Decodes the items of a [`ManagedVecEncoded`] one by one.
///
/// Signals an error if the encoded data is invalid.
pub struct ManagedVecEncodedIterator<M, T>
where
    M: ManagedTypeApi,
{
    input: ManagedBufferNestedDecodeInput<M>,
    _phantom: PhantomData<T>,
}

impl<M, T> ManagedVecEncodedIterator<M, T>
where
    M: ManagedTypeApi,
{
    fn new(buffer: ManagedBuffer<M>) -> Self {
        ManagedVecEncodedIterator {
            input: ManagedBufferNestedDecodeInput::new(buffer),
            _phantom: PhantomData,
        }
    }
}

impl<M, T> Iterator for ManagedVecEncodedIterator<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: NestedDecode,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.input.remaining_len() == 0 {
            return None;
        }

        let Ok(item) = T::dep_decode_or_handle_err(
            &mut self.input,
            ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_DECODE_ERROR),
        );
        Some(item)
    }
}

impl<M, T> IntoIterator for ManagedVecEncoded<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: NestedDecode,
{
    type Item = T;
    type IntoIter = ManagedVecEncodedIterator<M, T>;

    fn into_iter(self) -> Self::IntoIter {
        ManagedVecEncodedIterator::new(self.buffer)
    }
}

impl<M, T> IntoIterator for &ManagedVecEncoded<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: NestedDecode,
{
    type Item = T;
    type IntoIter = ManagedVecEncodedIterator<M, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<M, T> From<ManagedVec<M, T>> for ManagedVecEncoded<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: ManagedVecItem + NestedEncode,
{
    fn from(vec: ManagedVec<M, T>) -> Self {
        let mut buffer = ManagedBuffer::new();
        let Ok(()) = vec.top_encode_or_handle_err(
            &mut buffer,
            ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
        );
        Self::from_raw_buffer(buffer)
    }
}

impl<M, T> From<ManagedVecEncoded<M, T>> for ManagedVec<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: ManagedVecItem + NestedDecode,
{
    fn from(encoded: ManagedVecEncoded<M, T>) -> Self {
        let Ok(vec) = ManagedVec::top_decode_or_handle_err(
            encoded.buffer,
            ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_DECODE_ERROR),
        );
        vec
    }
}

impl<M, T> TopEncode for ManagedVecEncoded<M, T>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.buffer.top_encode_or_handle_err(output, h)
    }
}

impl<M, T> TopDecode for ManagedVecEncoded<M, T>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Self::from_raw_buffer(
            ManagedBuffer::top_decode_or_handle_err(input, h)?,
        ))
    }
}

impl<M, T> TypeAbiFrom<Self> for ManagedVecEncoded<M, T> where M: ManagedTypeApi {}
impl<M, T> TypeAbiFrom<&Self> for ManagedVecEncoded<M, T> where M: ManagedTypeApi {}

impl<M, T, U> TypeAbiFrom<ManagedVec<M, U>> for ManagedVecEncoded<M, T>
where
    M: ManagedTypeApi,
    U: ManagedVecItem,
    T: TypeAbiFrom<U>,
{
}

impl<M, T, U> TypeAbiFrom<ManagedVecEncoded<M, U>> for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + TypeAbiFrom<U>,
{
}

impl<M, T, U> TypeAbiFrom<Vec<U>> for ManagedVecEncoded<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbiFrom<U>,
{
}

impl<M, T, U> TypeAbiFrom<ManagedVecEncoded<M, U>> for Vec<T>
where
    M: ManagedTypeApi,
    T: TypeAbiFrom<U>,
{
}

impl<M, T> TypeAbi for ManagedVecEncoded<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi,
{
    type Unmanaged = Vec<T::Unmanaged>;

    /// It is semantically equivalent to any list of `T`.
    fn type_name() -> TypeName {
        <&[T] as TypeAbi>::type_name()
    }

    fn type_name_rust() -> TypeName {
        format!("ManagedVecEncoded<$API, {}>", T::type_name_rust())
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}
