use super::ArgId;
use crate::{
    api::{ErrorApi, ManagedTypeApi},
    err_msg,
    types::{ManagedBuffer, ManagedType},
};
use elrond_codec::DecodeError;

pub fn signal_arg_de_error<EA>(api: EA, arg_id: ArgId, decode_err: DecodeError) -> !
where
    EA: ManagedTypeApi + ErrorApi,
{
    let mut message_buffer = ManagedBuffer::<EA>::new_from_bytes(err_msg::ARG_DECODE_ERROR_1);
    message_buffer.append_bytes(arg_id.as_bytes());
    message_buffer.append_bytes(err_msg::ARG_DECODE_ERROR_2);
    message_buffer.append_bytes(decode_err.message_bytes());
    api.signal_error_from_buffer(message_buffer.get_raw_handle())
}
