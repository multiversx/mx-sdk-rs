use crate::{CodecFrom, TopDecodeMulti, TopEncodeMulti};

/// Signals that we can safely serialize `Self` in order to obtain a `T` on the other size.
pub trait CodecInto<T>: TopEncodeMulti
where
    T: TopDecodeMulti,
{
}

impl<F, I> CodecInto<F> for I
where
    I: TopEncodeMulti,
    F: CodecFrom<I>,
{
}
