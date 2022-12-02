use crate::{
    DecodeErrorHandler, EncodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput, TopEncodeMulti,
    TopEncodeMultiOutput,
};
use alloc::vec::Vec;
use core::iter::FromIterator;

/// Structure that allows taking a variable number of arguments
/// or returning a variable number of results in a smart contract endpoint.
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct MultiValueVec<T>(pub Vec<T>);

impl<T> From<Vec<T>> for MultiValueVec<T> {
    fn from(v: Vec<T>) -> Self {
        MultiValueVec(v)
    }
}

impl<T, const N: usize> From<[T; N]> for MultiValueVec<T>
where
    T: Clone,
{
    fn from(arr: [T; N]) -> Self {
        MultiValueVec(arr[..].to_vec())
    }
}

impl<T> MultiValueVec<T> {
    #[inline]
    pub fn new() -> Self {
        MultiValueVec(Vec::new())
    }
}

impl<T> MultiValueVec<T> {
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.0.iter()
    }
}

impl<T> FromIterator<T> for MultiValueVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let v = Vec::<T>::from_iter(iter);
        MultiValueVec(v)
    }
}

impl<T> TopEncodeMulti for MultiValueVec<T>
where
    T: TopEncodeMulti,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        for elem in self.0.iter() {
            elem.multi_encode_or_handle_err(output, h)?;
        }
        Ok(())
    }
}

impl<T> TopDecodeMulti for MultiValueVec<T>
where
    T: TopDecodeMulti,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let mut result_vec: Vec<T> = Vec::new();
        while input.has_next() {
            result_vec.push(T::multi_decode_or_handle_err(input, h)?);
        }
        Ok(Self(result_vec))
    }
}
