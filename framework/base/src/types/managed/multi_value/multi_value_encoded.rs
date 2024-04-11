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
pub struct MultiValueEncoded<'a, M, T>
where
    M: ManagedTypeApi<'a>,
{
    pub(super) raw_buffers: ManagedVec<'a, M, ManagedBuffer<'a, M>>,
    _phantom: PhantomData<T>,
}

#[deprecated(
    since = "0.29.0",
    note = "Alias kept for backwards compatibility. Replace with `MultiValueEncoded`"
)]
pub type ManagedVarArgs<'a, M, T> = MultiValueEncoded<'a, M, T>;

#[deprecated(
    since = "0.29.0",
    note = "Alias kept for backwards compatibility. Replace with `MultiValueEncoded`"
)]
pub type ManagedMultiResultVec<'a, M, T> = MultiValueEncoded<'a, M, T>;

impl<'a, M, T> MultiValueEncoded<'a, M, T>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn from_raw_vec(raw_buffers: ManagedVec<'a, M, ManagedBuffer<'a, M>>) -> Self {
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

impl<'a, M, T> MultiValueEncoded<'a, M, T>
where
    M: ManagedTypeApi<'a> + ErrorApi,
    T: TopEncodeMulti,
{
    pub fn push(&mut self, item: T) {
        let Ok(()) = item.multi_encode_or_handle_err(
            &mut self.raw_buffers,
            ExitCodecErrorHandler::<'a, M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
        );
    }
}

impl<'a, M, T> From<ManagedVec<'a, M, T>> for MultiValueEncoded<'a, M, T>
where
    M: ManagedTypeApi<'a>,
    T: ManagedVecItem + TopEncode + 'static,
{
    #[inline]
    #[rustfmt::skip]
    fn from(v: ManagedVec<'a, M, T>) -> Self {
        try_cast_execute_or_else(
            v,
            MultiValueEncoded::from_raw_vec,
            |v| MultiValueEncoded::from(&v),
        )
    }
}

impl<'a, M, T> From<&ManagedVec<'a, M, T>> for MultiValueEncoded<'a, M, T>
where
    M: ManagedTypeApi<'a>,
    T: ManagedVecItem + TopEncode,
{
    #[inline]
    fn from(v: &ManagedVec<'a, M, T>) -> Self {
        let mut result = MultiValueEncoded::new();
        for item in v.into_iter() {
            result.push(item);
        }
        result
    }
}

impl<'a, M, T> MultiValueEncoded<'a, M, T>
where
    M: ManagedTypeApi<'a>,
{
    pub fn to_arg_buffer(self) -> ManagedArgBuffer<'a, M> {
        ManagedArgBuffer::from_handle(self.raw_buffers.take_handle())
    }
}

impl<'a, M> MultiValueEncoded<'a, M, ManagedBuffer<'a, M>>
where
    M: ManagedTypeApi<'a>,
{
    pub fn into_vec_of_buffers(self) -> ManagedVec<'a, M, ManagedBuffer<'a, M>> {
        self.raw_buffers
    }
}

impl<'a, M, T> MultiValueEncoded<'a, M, T>
where
    M: ManagedTypeApi<'a> + ErrorApi,
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

impl<'a, M, T> MultiValueEncoded<'a, M, T>
where
    M: ManagedTypeApi<'a> + ErrorApi,
    T: TopDecodeMultiLength,
{
    /// Number of items. Only available for multi-encode items.
    #[inline]
    pub fn len(&self) -> usize {
        self.raw_len() / T::get_len()
    }
}

impl<'a, M, T> MultiValueEncoded<'a, M, T>
where
    M: ManagedTypeApi<'a> + ErrorApi,
    T: ManagedVecItem + TopDecode,
{
    pub fn to_vec(&self) -> ManagedVec<'a, M, T> {
        let mut result = ManagedVec::new();
        let serializer = ManagedSerializer::<'a, M>::new();
        for item in self.raw_buffers.into_iter() {
            result.push(serializer.top_decode_from_managed_buffer(&item));
        }
        result
    }
}

impl<'a, M, T> TopEncodeMulti for &MultiValueEncoded<'a, M, T>
where
    M: ManagedTypeApi<'a> + ErrorApi,
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

impl<'a, M, T> TopEncodeMulti for MultiValueEncoded<'a, M, T>
where
    M: ManagedTypeApi<'a> + ErrorApi,
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

impl<'a, M, T> TopDecodeMulti for MultiValueEncoded<'a, M, T>
where
    M: ManagedTypeApi<'a> + ErrorApi,
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

impl<'a, M, T> TypeAbi for MultiValueEncoded<'a, M, T>
where
    M: ManagedTypeApi<'a>,
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

impl<'a, M, T> CodecFromSelf for MultiValueEncoded<'a, M, T> where M: ManagedTypeApi<'a> {}

#[cfg(feature = "alloc")]
use crate::codec::{multi_types::MultiValueVec, CodecFrom};

#[cfg(feature = "alloc")]
impl<'a, M, T, U> CodecFrom<MultiValueVec<T>> for MultiValueEncoded<'a, M, U>
where
    M: ManagedTypeApi<'a> + ErrorApi,
    T: TopEncodeMulti,
    U: CodecFrom<T>,
{
}

#[cfg(feature = "alloc")]
impl<'a, M, T, U> CodecFrom<MultiValueEncoded<'a, M, T>> for MultiValueVec<U>
where
    M: ManagedTypeApi<'a> + ErrorApi,
    T: TopEncodeMulti,
    U: CodecFrom<T>,
{
}

impl<'a, M, V> FromIterator<V> for MultiValueEncoded<'a, M, V>
where
    M: ManagedTypeApi<'a>,
    V: TopEncodeMulti,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut result: MultiValueEncoded<'a, M, V> = MultiValueEncoded::new();
        iter.into_iter().for_each(|f| result.push(f));
        result
    }
}
