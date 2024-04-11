use crate::codec::{TopEncode, TopEncodeMulti};

use crate::{
    api::{ErrorApi, LogApi, LogApiImpl, ManagedTypeApi},
    contract_base::ExitCodecErrorHandler,
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
    T: TopEncodeMulti,
{
    let Ok(()) = topic.multi_encode_or_handle_err(
        accumulator,
        ExitCodecErrorHandler::<A>::from(err_msg::LOG_TOPIC_ENCODE_ERROR),
    );
}

pub fn serialize_log_data<T, A>(data: T) -> ManagedBuffer<A>
where
    T: TopEncode,
    A: ErrorApi + ManagedTypeApi,
{
    let mut data_buffer = ManagedBuffer::new();
    let Ok(()) = data.top_encode_or_handle_err(
        &mut data_buffer,
        ExitCodecErrorHandler::<A>::from(err_msg::LOG_DATA_ENCODE_ERROR),
    );
    data_buffer
}

pub fn write_log<A>(topics: &ManagedVec<A, ManagedBuffer<A>>, data: &ManagedBuffer<A>)
where
    A: LogApi + ManagedTypeApi,
{
    A::log_api_impl().managed_write_log(topics.get_handle(), data.get_handle());
}
