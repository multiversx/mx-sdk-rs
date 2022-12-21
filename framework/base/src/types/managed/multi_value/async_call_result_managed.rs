use crate::{
    abi::{TypeAbi, TypeName},
    api::ManagedTypeApi,
    codec::{
        DecodeErrorHandler, EncodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput,
        TopEncodeMulti, TopEncodeMultiOutput,
    },
    types::ManagedBuffer,
};

const SAME_SHARD_SUCCESS_CODE: u32 = 0;
const CROSS_SHARD_SUCCESS_CODE: u32 = 0x00006f6b; // "ok"

pub struct ManagedAsyncCallError<M>
where
    M: ManagedTypeApi,
{
    pub err_code: u32,
    pub err_msg: ManagedBuffer<M>,
}

pub enum ManagedAsyncCallResult<M, T>
where
    M: ManagedTypeApi,
{
    Ok(T),
    Err(ManagedAsyncCallError<M>),
}

impl<M, T> ManagedAsyncCallResult<M, T>
where
    M: ManagedTypeApi,
{
    #[inline]
    pub fn is_ok(&self) -> bool {
        matches!(self, ManagedAsyncCallResult::Ok(_))
    }

    #[inline]
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

impl<M, T> TopDecodeMulti for ManagedAsyncCallResult<M, T>
where
    M: ManagedTypeApi,
    T: TopDecodeMulti,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let err_code: u32 = input.next_value(h)?;
        if err_code == SAME_SHARD_SUCCESS_CODE || err_code == CROSS_SHARD_SUCCESS_CODE {
            Ok(Self::Ok(T::multi_decode_or_handle_err(input, h)?))
        } else {
            let err_msg = if input.has_next() {
                input.next_value(h)?
            } else {
                // temporary fix, until a problem involving missing error messages in the protocol gets fixed
                // can be removed after the protocol is patched
                // error messages should not normally be missing
                ManagedBuffer::new()
            };
            Ok(Self::Err(ManagedAsyncCallError { err_code, err_msg }))
        }
    }
}

impl<M, T> TopEncodeMulti for ManagedAsyncCallResult<M, T>
where
    M: ManagedTypeApi,
    T: TopEncodeMulti,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        match self {
            ManagedAsyncCallResult::Ok(result) => {
                0u32.multi_encode_or_handle_err(output, h)?;
                result.multi_encode_or_handle_err(output, h)?;
            },
            ManagedAsyncCallResult::Err(error_message) => {
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

impl<M, T> TypeAbi for ManagedAsyncCallResult<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi,
{
    fn type_name() -> TypeName {
        let mut repr = TypeName::from("AsyncCallResult<");
        repr.push_str(T::type_name().as_str());
        repr.push('>');
        repr
    }
}
