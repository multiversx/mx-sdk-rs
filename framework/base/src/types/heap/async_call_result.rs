use crate::{
    abi::{TypeAbi, TypeName},
    codec::{
        DecodeErrorHandler, EncodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput,
        TopEncodeMulti, TopEncodeMultiOutput,
    },
    types::heap::BoxedBytes,
};

pub struct AsyncCallError {
    pub err_code: u32,
    pub err_msg: BoxedBytes,
}

pub enum AsyncCallResult<T> {
    Ok(T),
    Err(AsyncCallError),
}

impl<T> AsyncCallResult<T> {
    #[inline]
    pub fn is_ok(&self) -> bool {
        matches!(self, AsyncCallResult::Ok(_))
    }

    #[inline]
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

impl<T> TopDecodeMulti for AsyncCallResult<T>
where
    T: TopDecodeMulti,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let err_code: u32 = input.next_value(h)?;
        if err_code == 0 {
            Ok(Self::Ok(T::multi_decode_or_handle_err(input, h)?))
        } else {
            let err_msg = if input.has_next() {
                input.next_value(h)?
            } else {
                // temporary fix, until a problem involving missing error messages in the protocol gets fixed
                // can be removed after the protocol is patched
                // error messages should not normally be missing
                BoxedBytes::empty()
            };
            Ok(Self::Err(AsyncCallError { err_code, err_msg }))
        }
    }
}

impl<T> TopEncodeMulti for AsyncCallResult<T>
where
    T: TopEncodeMulti,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        match self {
            AsyncCallResult::Ok(result) => {
                0u32.multi_encode_or_handle_err(output, h)?;
                result.multi_encode_or_handle_err(output, h)?;
            },
            AsyncCallResult::Err(error_message) => {
                error_message
                    .err_code
                    .multi_encode_or_handle_err(output, h)?;
                error_message
                    .err_msg
                    .multi_encode_or_handle_err(output, h)?;
            },
        }
        Ok(())
    }
}

impl<T: TypeAbi> TypeAbi for AsyncCallResult<T> {
    fn type_name() -> TypeName {
        let mut repr = TypeName::from("AsyncCallResult<");
        repr.push_str(T::type_name().as_str());
        repr.push('>');
        repr
    }
}
