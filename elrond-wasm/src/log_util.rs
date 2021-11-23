use elrond_codec::{EncodeError, TopEncode};

use crate::{
    api::{ErrorApi, LogApi, ManagedTypeApi},
    err_msg,
    types::{ManagedBuffer, ManagedType, ManagedVec},
};

pub fn event_topic_accumulator<A>(event_identifier: &[u8]) -> ManagedVec<A, ManagedBuffer<A>>
where
    A: ErrorApi + ManagedTypeApi,
{
    let mut accumulator = ManagedVec::new();
    accumulator.push(ManagedBuffer::new_from_bytes(event_identifier));
    accumulator
}

pub fn serialize_event_topic<A, T>(accumulator: &mut ManagedVec<A, ManagedBuffer<A>>, topic: T)
where
    A: ErrorApi + ManagedTypeApi,
    T: TopEncode,
{
    let mut topic_buffer = ManagedBuffer::new();
    topic.top_encode_or_exit(
        &mut topic_buffer,
        accumulator.type_manager(),
        serialize_log_topic_exit,
    );
    accumulator.push(topic_buffer);
}

#[inline(always)]
fn serialize_log_topic_exit<A>(api: A, encode_err: EncodeError) -> !
where
    A: ErrorApi + ManagedTypeApi + 'static,
{
    let mut message_buffer = ManagedBuffer::<A>::new_from_bytes(err_msg::LOG_TOPIC_ENCODE_ERROR);
    message_buffer.append_bytes(encode_err.message_bytes());
    api.signal_error_from_buffer(message_buffer.get_raw_handle())
}

pub fn serialize_log_data<T, A>(api: A, data: T) -> ManagedBuffer<A>
where
    T: TopEncode,
    A: ErrorApi + ManagedTypeApi + Clone + 'static,
{
    let mut data_buffer = ManagedBuffer::new();
    data.top_encode_or_exit(&mut data_buffer, api, serialize_log_data_exit);
    data_buffer
}

#[inline(always)]
fn serialize_log_data_exit<A>(api: A, encode_err: EncodeError) -> !
where
    A: ErrorApi + ManagedTypeApi + 'static,
{
    let mut message_buffer = ManagedBuffer::<A>::new_from_bytes(err_msg::LOG_DATA_ENCODE_ERROR);
    message_buffer.append_bytes(encode_err.message_bytes());
    api.signal_error_from_buffer(message_buffer.get_raw_handle())
}

pub fn write_log<L, A>(api: L, topics: &ManagedVec<A, ManagedBuffer<A>>, data: &ManagedBuffer<A>)
where
    L: LogApi,
    A: ErrorApi + ManagedTypeApi,
{
    api.managed_write_log(topics.get_raw_handle(), data.get_raw_handle());
}
