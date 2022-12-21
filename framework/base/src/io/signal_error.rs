use super::ArgId;
use crate::{
    api::{ErrorApi, ErrorApiImpl, ManagedTypeApi},
    codec::DecodeError,
    err_msg,
    types::{ManagedBuffer, ManagedType},
};

pub fn signal_arg_de_error<EA>(arg_id: ArgId, decode_err: DecodeError) -> !
where
    EA: ManagedTypeApi + ErrorApi,
{
    let mut message_buffer = ManagedBuffer::<EA>::new_from_bytes(err_msg::ARG_DECODE_ERROR_1);
    message_buffer.append_bytes(arg_id.as_bytes());
    message_buffer.append_bytes(err_msg::ARG_DECODE_ERROR_2);
    message_buffer.append_bytes(decode_err.message_bytes());
    EA::error_api_impl().signal_error_from_buffer(message_buffer.get_handle())
}
