use crate::{TopDecode, TopEncode};

pub trait MultiValueLength {
    fn multi_value_len(&self) -> usize;
}

pub trait MultiValueConstLength {
    const CONST_LEN: usize;

    fn multi_value_const_len() -> usize {
        Self::CONST_LEN
    }
}

impl<T> MultiValueLength for T
where
    T: TopEncode + TopDecode,
{
    #[inline]
    fn multi_value_len(&self) -> usize {
        1
    }
}

impl<T> MultiValueConstLength for T
where
    T: TopEncode + TopDecode,
{
    const CONST_LEN: usize = 1;
}
