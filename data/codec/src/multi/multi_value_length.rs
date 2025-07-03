use crate::{TopDecode, TopEncode};

/// Indicates that a multi-value has a countable number of single items contained.
///
/// This applies to single items as well, which have a multi value length of 1.
pub trait MultiValueLength {
    /// The number of single items contained a multi-value.
    fn multi_value_len(&self) -> usize;
}

/// Indicates that a multi-value has a fixed (constant) number of single items contained.
///
/// This applies to:
/// - single items
/// - multi-value tuples
pub trait MultiValueConstLength {
    /// The fixed (constant) number of single items contained a multi-value.
    ///
    /// Its value is:
    /// - for single items: 1,
    /// - for multi-value tuples: the sum of the fixed lengths of their items, if applicable.
    const MULTI_VALUE_CONST_LEN: usize;
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
    const MULTI_VALUE_CONST_LEN: usize = 1;
}
