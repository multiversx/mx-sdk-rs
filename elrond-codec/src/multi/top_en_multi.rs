use crate::{
    DefaultErrorHandler, EncodeError, EncodeErrorHandler, TopEncode, TopEncodeMultiOutput,
};

pub trait TopEncodeMulti: Sized {
    /// Attempt to serialize the value to ouput.
    fn multi_encode<O>(&self, output: &mut O) -> Result<(), EncodeError>
    where
        O: TopEncodeMultiOutput,
    {
        self.multi_encode_or_handle_err(output, DefaultErrorHandler)
    }

    /// Version of `top_encode` that can handle errors as soon as they occur.
    /// For instance in can exit immediately and make sure that if it returns, it is a success.
    /// By not deferring error handling, this can lead to somewhat smaller bytecode.
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        match self.multi_encode(output) {
            Ok(()) => Ok(()),
            Err(e) => Err(h.handle_error(e)),
        }
    }
}

/// All single top encode types also work as multi-value encode types.
impl<T> TopEncodeMulti for T
where
    T: TopEncode,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        output.push_single_value(self, h)
    }
}

pub fn multi_encode_iter_or_handle_err<T, Iter, O, H>(
    iterator: Iter,
    output: &mut O,
    h: H,
) -> Result<(), H::HandledErr>
where
    T: TopEncodeMulti,
    Iter: Iterator<Item = T>,
    O: TopEncodeMultiOutput,
    H: EncodeErrorHandler,
{
    for item in iterator {
        item.multi_encode_or_handle_err(output, h)?;
    }
    Ok(())
}
