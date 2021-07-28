use super::ArgId;
use crate::api::ErrorApi;
use crate::err_msg;
use crate::types::BoxedBytes;
use elrond_codec::DecodeError;

pub fn signal_arg_de_error<EA: ErrorApi>(api: &EA, arg_id: ArgId, de_err: DecodeError) -> ! {
    let decode_err_message = BoxedBytes::from_concat(
        &[
            err_msg::ARG_DECODE_ERROR_1,
            arg_id.as_bytes(),
            err_msg::ARG_DECODE_ERROR_2,
            de_err.message_bytes(),
        ][..],
    );
    api.signal_error(decode_err_message.as_slice())
}
