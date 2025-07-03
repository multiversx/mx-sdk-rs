use unwrap_infallible::UnwrapInfallible;

use crate::codec::multi_types::MultiValueVec;
use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName},
    api::{ErrorApi, ManagedTypeApi},
    codec::{
        DecodeErrorHandler, EncodeErrorHandler, MultiValueConstLength, TopDecode, TopDecodeMulti,
        TopDecodeMultiInput, TopEncodeMulti, TopEncodeMultiOutput,
    },
    contract_base::{ExitCodecErrorHandler, ManagedSerializer},
    err_msg,
    types::{ManagedBuffer, ManagedVec, ManagedVecItem},
};
use core::{iter::FromIterator, marker::PhantomData};

use super::MultiValueEncodedIterator;

/// A multi-value container, that keeps raw values as ManagedBuffer, and which encodes and decodes its length explicitly.
///
/// It allows encoding and decoding of multi-values. Its multi-encoding always starts with the number of items.
///
/// Since items are kept raw, the item type does not need to implement `ManagedVecItem`.
///
/// Behavior:
/// - It is lazy when decoding, in that it keeps them raw and will not decode the values until they are requested.
/// - It is eager when encoding, items are serialized before being added to this structure.
///
/// ## Item length
///
/// Its item type must implement `MultiValueConstLength`, which is a length marker for multi-values.
///
/// Some examples for `MultiValueConstLength`:
/// - MultiValue3 has a "multi-length" of 3
/// - a simple type, like i32, has a "multi-length" of 1
/// - MultiValueEncoded has no known "multi-length", and therefore cannot be used inside `MultiValueEncodedCounted`.
///
/// `MultiValueEncodedCounted` requires this "multi-length" to determine the number of buffers needed to store the encoded values.
///
/// More specifically, the number of buffers (raw length) is equal to the item count multiplied by the item "multi-length".
#[derive(Clone, Default, Debug, PartialEq)]
pub struct MultiValueEncodedCounted<M, T>
where
    M: ManagedTypeApi,
    T: MultiValueConstLength,
{
    raw_buffers: ManagedVec<M, ManagedBuffer<M>>,
    _phantom: PhantomData<T>,
}

impl<M, T> MultiValueEncodedCounted<M, T>
where
    M: ManagedTypeApi,
    T: MultiValueConstLength,
{
    #[inline]
    fn from_raw_vec(raw_buffers: ManagedVec<M, ManagedBuffer<M>>) -> Self {
        MultiValueEncodedCounted {
            raw_buffers,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn new() -> Self {
        MultiValueEncodedCounted::from_raw_vec(ManagedVec::new())
    }
}

impl<M, T> MultiValueEncodedCounted<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopEncodeMulti + MultiValueConstLength,
{
    pub fn push(&mut self, item: T) {
        item.multi_encode_or_handle_err(
            &mut self.raw_buffers,
            ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
        )
        .unwrap_infallible()
    }
}

impl<M, T> IntoIterator for MultiValueEncodedCounted<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopDecodeMulti + MultiValueConstLength,
{
    type Item = T;
    type IntoIter = MultiValueEncodedIterator<M, T>;
    fn into_iter(self) -> Self::IntoIter {
        MultiValueEncodedIterator::new(self.raw_buffers)
    }
}

impl<M> MultiValueEncodedCounted<M, ManagedBuffer<M>>
where
    M: ManagedTypeApi,
{
    pub fn into_vec_of_buffers(self) -> ManagedVec<M, ManagedBuffer<M>> {
        self.raw_buffers
    }
}

impl<M, T> MultiValueEncodedCounted<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: MultiValueConstLength,
{
    /// Length of the underlying data.
    ///
    /// Note:
    /// In general, it is **not** the number of items that can be decoded.
    /// It is the same as `len()` only for single encode items.
    #[inline]
    pub fn raw_len(&self) -> usize {
        self.raw_buffers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.raw_buffers.is_empty()
    }

    /// Number of items. Only available for multi-encode items.
    #[inline]
    pub fn len(&self) -> usize {
        self.raw_len() / T::MULTI_VALUE_CONST_LEN
    }
}

impl<M, T> MultiValueEncodedCounted<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: ManagedVecItem + TopDecode + MultiValueConstLength,
{
    pub fn to_vec(&self) -> ManagedVec<M, T> {
        let mut result = ManagedVec::new();
        let serializer = ManagedSerializer::<M>::new();
        for item in &self.raw_buffers {
            result.push(serializer.top_decode_from_managed_buffer(&item));
        }
        result
    }
}

impl<M, T> TopEncodeMulti for &MultiValueEncodedCounted<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopEncodeMulti + MultiValueConstLength,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        let raw_count = self.raw_buffers.len();
        let count = raw_count / T::MULTI_VALUE_CONST_LEN;
        count.multi_encode_or_handle_err(output, h)?;
        for elem in &self.raw_buffers {
            elem.multi_encode_or_handle_err(output, h)?;
        }
        Ok(())
    }
}

impl<M, T> TopEncodeMulti for MultiValueEncodedCounted<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopEncodeMulti + MultiValueConstLength,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        (&self).multi_encode_or_handle_err(output, h)
    }
}

impl<M, T> TopDecodeMulti for MultiValueEncodedCounted<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopDecodeMulti + MultiValueConstLength,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let count: usize = input.next_value(h)?;
        let raw_count = count * T::MULTI_VALUE_CONST_LEN;
        let mut raw_buffers = ManagedVec::new();
        for _ in 0..raw_count {
            raw_buffers.push(input.next_value(h)?);
        }
        Ok(Self::from_raw_vec(raw_buffers))
    }
}

impl<M, T> TypeAbiFrom<Self> for MultiValueEncodedCounted<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi + MultiValueConstLength,
{
}

impl<M, T> TypeAbiFrom<&Self> for MultiValueEncodedCounted<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi + MultiValueConstLength,
{
}

impl<M, T> TypeAbi for MultiValueEncodedCounted<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi + MultiValueConstLength,
{
    type Unmanaged = MultiValueVec<T::Unmanaged>;

    fn type_name() -> TypeName {
        let mut repr = TypeName::from("counted-variadic<");
        repr.push_str(T::type_name().as_str());
        repr.push('>');
        repr
    }

    fn type_name_rust() -> TypeName {
        crate::abi::type_name_multi_value_encoded::<T>()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        true
    }
}

impl<M, T, U> TypeAbiFrom<MultiValueVec<T>> for MultiValueEncodedCounted<M, U>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopEncodeMulti,
    U: TypeAbiFrom<T> + MultiValueConstLength,
{
}

impl<M, T, U> TypeAbiFrom<MultiValueEncodedCounted<M, T>> for MultiValueVec<U>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopEncodeMulti + MultiValueConstLength,
    U: TypeAbiFrom<T>,
{
}

impl<M, V> FromIterator<V> for MultiValueEncodedCounted<M, V>
where
    M: ManagedTypeApi,
    V: TopEncodeMulti + MultiValueConstLength,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut result: MultiValueEncodedCounted<M, V> = MultiValueEncodedCounted::new();
        iter.into_iter().for_each(|f| result.push(f));
        result
    }
}
