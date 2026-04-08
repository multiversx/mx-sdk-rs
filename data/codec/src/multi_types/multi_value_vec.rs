use crate::{
    DecodeErrorHandler, EncodeErrorHandler, MultiValueConstLength, MultiValueLength,
    TopDecodeMulti, TopDecodeMultiInput, TopEncodeMulti, TopEncodeMultiOutput,
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

impl<T> MultiValueLength for MultiValueVec<T>
where
    T: MultiValueConstLength,
{
    #[inline]
    fn multi_value_len(&self) -> usize {
        self.len() * T::MULTI_VALUE_CONST_LEN
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn new_is_empty() {
        let mv: MultiValueVec<u32> = MultiValueVec::new();
        assert!(mv.is_empty());
        assert_eq!(mv.len(), 0);
    }

    #[test]
    fn from_vec() {
        let mv = MultiValueVec::from(vec![1u32, 2, 3]);
        assert_eq!(mv.len(), 3);
        assert_eq!(mv.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn from_array() {
        let mv = MultiValueVec::from([10u64, 20, 30]);
        assert_eq!(mv.len(), 3);
        assert_eq!(mv.as_slice(), &[10, 20, 30]);
    }

    #[test]
    fn into_vec_round_trip() {
        let original = vec![5u8, 10, 15, 20];
        let mv = MultiValueVec::from(original.clone());
        assert_eq!(mv.into_vec(), original);
    }

    #[test]
    fn push_and_len() {
        let mut mv = MultiValueVec::new();
        mv.push(1u32);
        mv.push(2);
        mv.push(3);
        assert_eq!(mv.len(), 3);
        assert!(!mv.is_empty());
        assert_eq!(mv.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn iter() {
        let mv = MultiValueVec::from(vec![10u32, 20, 30]);
        let sum: u32 = mv.iter().sum();
        assert_eq!(sum, 60);
    }

    #[test]
    fn from_iterator() {
        let mv: MultiValueVec<u32> = (1..=5).collect();
        assert_eq!(mv.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn clone_and_eq() {
        let mv = MultiValueVec::from(vec![1u32, 2, 3]);
        let mv_clone = mv.clone();
        assert_eq!(mv, mv_clone);
    }

    #[test]
    fn ne() {
        let mv1 = MultiValueVec::from(vec![1u32, 2]);
        let mv2 = MultiValueVec::from(vec![1u32, 3]);
        assert_ne!(mv1, mv2);
    }

    #[test]
    fn debug_format() {
        let mv = MultiValueVec::from(vec![42u32]);
        let debug_str = alloc::format!("{:?}", mv);
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn default_is_empty() {
        let mv: MultiValueVec<i64> = MultiValueVec::default();
        assert!(mv.is_empty());
        assert_eq!(mv.into_vec(), Vec::<i64>::new());
    }

    #[test]
    fn from_empty_array() {
        let mv = MultiValueVec::from([0u8; 0]);
        assert!(mv.is_empty());
    }
}
