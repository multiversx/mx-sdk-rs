use crate::codec::{TopEncode, TopEncodeMulti};

use crate::{
    api::{ErrorApi, LogApi, LogApiImpl, ManagedTypeApi},
    contract_base::ExitCodecErrorHandler,
    err_msg,
    types::{ManagedBuffer, ManagedType, ManagedVec},
};

pub fn event_topic_accumulator<'a, A>(event_identifier: &[u8]) -> ManagedVec<'a, A, ManagedBuffer<'a, A>>
where
    A: ErrorApi + ManagedTypeApi<'a>,
{
    let mut accumulator = ManagedVec::new();
    accumulator.push(ManagedBuffer::new_from_bytes(event_identifier));
    accumulator
}

pub fn serialize_event_topic<'a, A, T>(accumulator: &mut ManagedVec<'a, A, ManagedBuffer<'a, A>>, topic: T)
where
    A: ErrorApi + ManagedTypeApi<'a>,
    T: TopEncodeMulti,
{
    let Ok(()) = topic.multi_encode_or_handle_err(
        accumulator,
        ExitCodecErrorHandler::<'a, A>::from(err_msg::LOG_TOPIC_ENCODE_ERROR),
    );
}

pub fn serialize_log_data<'a, T, A>(data: T) -> ManagedBuffer<'a, A>
where
    T: TopEncode,
    A: ErrorApi + ManagedTypeApi<'a>,
{
    let mut data_buffer = ManagedBuffer::new();
    let Ok(()) = data.top_encode_or_handle_err(
        &mut data_buffer,
        ExitCodecErrorHandler::<'a, A>::from(err_msg::LOG_DATA_ENCODE_ERROR),
    );
    data_buffer
}

pub fn write_log<'a, A>(topics: &ManagedVec<'a, A, ManagedBuffer<'a, A>>, data: &ManagedBuffer<'a, A>)
where
    A: LogApi + ManagedTypeApi<'a>,
{
    A::log_api_impl().managed_write_log(topics.get_handle(), data.get_handle());
}
