use crate::{
    DecodeErrorHandler, EncodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput, TopEncodeMulti,
    TopEncodeMultiOutput,
};

/// A smart contract argument or result that can be missing.
///
/// If arguments stop before this argument, None will be returned.
/// As an endpoint result, the contract decides if it produces it or not.
///
/// As a principle, optional arguments or results should come last,
/// otherwise there is ambiguity as to how to interpret what comes after.
#[must_use]
#[derive(Clone)]
pub enum OptionalValue<T> {
    Some(T),
    None,
}

impl<T> From<Option<T>> for OptionalValue<T> {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(arg) => OptionalValue::Some(arg),
            None => OptionalValue::None,
        }
    }
}

impl<T> OptionalValue<T> {
    pub fn into_option(self) -> Option<T> {
        match self {
            OptionalValue::Some(arg) => Some(arg),
            OptionalValue::None => None,
        }
    }
}

impl<T> TopEncodeMulti for OptionalValue<T>
where
    T: TopEncodeMulti,
{
    type DecodeAs = Self;

    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        if let OptionalValue::Some(t) = self {
            t.multi_encode_or_handle_err(output, h)?;
        }
        Ok(())
    }
}

impl<T> TopDecodeMulti for OptionalValue<T>
where
    T: TopDecodeMulti,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        if input.has_next() {
            Ok(OptionalValue::Some(T::multi_decode_or_handle_err(
                input, h,
            )?))
        } else {
            Ok(OptionalValue::None)
        }
    }
}
