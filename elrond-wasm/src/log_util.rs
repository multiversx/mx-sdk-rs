use elrond_codec::{EncodeError, TopEncode};

use crate::{api::ErrorApi, types::BoxedBytes};

pub fn serialize_log_data<T, EA>(data: T, api: EA) -> BoxedBytes
where
    T: TopEncode,
    EA: ErrorApi + Clone + 'static,
{
    let mut result = BoxedBytes::empty();
    data.top_encode_or_exit(&mut result, api, serialize_log_data_exit);
    result
}

#[inline(always)]
fn serialize_log_data_exit<EA>(api: EA, encode_err: EncodeError) -> !
where
    EA: ErrorApi + 'static,
{
    api.signal_error(encode_err.message_bytes())
}
