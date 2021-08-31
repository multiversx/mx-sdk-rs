use elrond_codec::{EncodeError, TopEncode};

use crate::{
    api::{ErrorApi, ManagedTypeApi},
    err_msg,
    types::{BoxedBytes, ManagedBuffer, ManagedType},
};

pub fn serialize_log_data<T, EA>(data: T, api: EA) -> BoxedBytes
where
    T: TopEncode,
    EA: ErrorApi + ManagedTypeApi + Clone + 'static,
{
    let mut result = BoxedBytes::empty();
    data.top_encode_or_exit(&mut result, api, serialize_log_data_exit);
    result
}

#[inline(always)]
fn serialize_log_data_exit<EA>(api: EA, encode_err: EncodeError) -> !
where
    EA: ErrorApi + ManagedTypeApi + 'static,
{
    let mut message_buffer =
        ManagedBuffer::new_from_bytes(api.clone(), err_msg::LOG_DATA_ENCODE_ERROR);
    message_buffer.append_bytes(encode_err.message_bytes());
    api.signal_error_from_buffer(message_buffer.get_raw_handle())
}
