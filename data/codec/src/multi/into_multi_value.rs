use super::{TopDecodeMulti, TopEncodeMulti};

/// Defines conversion of a type to its multi-value representation.
///
/// Consumes input.
pub trait IntoMultiValue {
    type MultiValue: TopEncodeMulti + TopDecodeMulti;

    fn into_multi_value(self) -> Self::MultiValue;
}
