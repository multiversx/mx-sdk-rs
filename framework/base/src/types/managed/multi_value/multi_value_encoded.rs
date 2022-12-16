use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::{ErrorApi, ManagedTypeApi},
    codec::{
        try_cast_execute_or_else, CodecFromSelf, DecodeErrorHandler, EncodeErrorHandler, TopDecode,
        TopDecodeMulti, TopDecodeMultiInput, TopDecodeMultiLength, TopEncode, TopEncodeMulti,
        TopEncodeMultiOutput,
    },
    contract_base::{ExitCodecErrorHandler, ManagedSerializer},
    err_msg,
    types::{ManagedArgBuffer, ManagedBuffer, ManagedType, ManagedVec, ManagedVecItem},
};
use core::{iter::FromIterator, marker::PhantomData};

/// A multi-value container, that keeps raw values as ManagedBuffer
/// It allows encoding and decoding of multi-values.
///
/// Since items are kept raw, the item type does not need to implement `ManagedVecItem`.
///
/// Behavior:
/// - It is lazy when decoding, in that it keeps them raw and will not decode the values until they are requested.
/// - It is eager when encoding, items are serialized before being added to this structure.
///
/// Since it can contain multi-values, the number of actual items it contains cannot be determined without fully decoding.
///
#[derive(Clone, Default, Debug, PartialEq)]
pub struct MultiValueEncoded<M, T>
where
    M: ManagedTypeApi,
{
    pub(super) raw_buffers: ManagedVec<M, ManagedBuffer<M>>,
    _phantom: PhantomData<T>,
}

#[deprecated(
    since = "0.29.0",
    note = "Alias kept for backwards compatibility. Replace with `MultiValueEncoded`"
)]
pub type ManagedVarArgs<M, T> = MultiValueEncoded<M, T>;

#[deprecated(
    since = "0.29.0",
    note = "Alias kept for backwards compatibility. Replace with `MultiValueEncoded`"
)]
pub type ManagedMultiResultVec<M, T> = MultiValueEncoded<M, T>;

impl<M, T> MultiValueEncoded<M, T>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from_raw_vec(raw_buffers: ManagedVec<M, ManagedBuffer<M>>) -> Self {
        MultiValueEncoded {
            raw_buffers,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn new() -> Self {
        MultiValueEncoded::from_raw_vec(ManagedVec::new())
    }
}

impl<M, T> MultiValueEncoded<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopEncodeMulti,
{
    pub fn push(&mut self, item: T) {
        let Ok(()) = item.multi_encode_or_handle_err(
            &mut self.raw_buffers,
            ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
        );
    }
}

impl<M, T> From<ManagedVec<M, T>> for MultiValueEncoded<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + TopEncode + 'static,
{
    #[inline]
    #[rustfmt::skip]
    fn from(v: ManagedVec<M, T>) -> Self {
        try_cast_execute_or_else(
            v,
            MultiValueEncoded::from_raw_vec,
            |v| MultiValueEncoded::from(&v),
        )
    }
}

impl<M, T> From<&ManagedVec<M, T>> for MultiValueEncoded<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + TopEncode,
{
    #[inline]
    fn from(v: &ManagedVec<M, T>) -> Self {
        let mut result = MultiValueEncoded::new();
        for item in v.into_iter() {
            result.push(item);
        }
        result
    }
}

impl<M, T> MultiValueEncoded<M, T>
where
    M: ManagedTypeApi,
{
    pub fn to_arg_buffer(&self) -> ManagedArgBuffer<M> {
        ManagedArgBuffer::from_handle(self.raw_buffers.get_handle())
    }
}

impl<M> MultiValueEncoded<M, ManagedBuffer<M>>
where
    M: ManagedTypeApi,
{
    pub fn into_vec_of_buffers(self) -> ManagedVec<M, ManagedBuffer<M>> {
        self.raw_buffers
    }
}

impl<M, T> MultiValueEncoded<M, T>
where
    M: ManagedTypeApi + ErrorApi,
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
}

impl<M, T> MultiValueEncoded<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopDecodeMultiLength,
{
    /// Number of items. Only available for multi-encode items.
    #[inline]
    pub fn len(&self) -> usize {
        self.raw_len() / T::get_len()
    }
}

impl<M, T> MultiValueEncoded<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: ManagedVecItem + TopDecode,
{
    pub fn to_vec(&self) -> ManagedVec<M, T> {
        let mut result = ManagedVec::new();
        let serializer = ManagedSerializer::<M>::new();
        for item in self.raw_buffers.into_iter() {
            result.push(serializer.top_decode_from_managed_buffer(&item));
        }
        result
    }
}

impl<M, T> TopEncodeMulti for &MultiValueEncoded<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopEncodeMulti,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        for elem in self.raw_buffers.into_iter() {
            elem.multi_encode_or_handle_err(output, h)?;
        }
        Ok(())
    }
}

impl<M, T> TopEncodeMulti for MultiValueEncoded<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopEncodeMulti,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        (&self).multi_encode_or_handle_err(output, h)
    }
}

impl<M, T> TopDecodeMulti for MultiValueEncoded<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopDecodeMulti,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let mut raw_buffers = ManagedVec::new();
        while input.has_next() {
            raw_buffers.push(input.next_value(h)?);
        }
        Ok(Self {
            raw_buffers,
            _phantom: PhantomData,
        })
    }
}

impl<M, T> TypeAbi for MultiValueEncoded<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi,
{
    fn type_name() -> TypeName {
        crate::abi::type_name_variadic::<T>()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        true
    }
}

impl<M, T> CodecFromSelf for MultiValueEncoded<M, T> where M: ManagedTypeApi {}

#[cfg(feature = "alloc")]
use crate::codec::{multi_types::MultiValueVec, CodecFrom};

#[cfg(feature = "alloc")]
impl<M, T, U> CodecFrom<MultiValueVec<T>> for MultiValueEncoded<M, U>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopEncodeMulti,
    U: CodecFrom<T>,
{
}

#[cfg(feature = "alloc")]
impl<M, T, U> CodecFrom<MultiValueEncoded<M, T>> for MultiValueVec<U>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopEncodeMulti,
    U: CodecFrom<T>,
{
}

impl<M, V> FromIterator<V> for MultiValueEncoded<M, V>
where
    M: ManagedTypeApi,
    V: TopEncodeMulti,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut result: MultiValueEncoded<M, V> = MultiValueEncoded::new();
        iter.into_iter().for_each(|f| result.push(f));
        result
    }
}
