use crate::{
    api::{CallTypeApi, StorageWriteApi},
    contract_base::SendRawWrapper,
    types::{BigUint, CallbackClosure, ManagedAddress, ManagedArgBuffer, ManagedBuffer},
};

#[must_use]
pub struct AsyncCall<SA>
where
    SA: CallTypeApi + 'static,
{
    pub(crate) to: ManagedAddress<SA>,
    pub(crate) egld_payment: BigUint<SA>,
    pub(crate) endpoint_name: ManagedBuffer<SA>,
    pub(crate) arg_buffer: ManagedArgBuffer<SA>,
    pub(crate) callback_call: Option<CallbackClosure<SA>>,
}

#[allow(clippy::return_self_not_must_use)]
impl<SA> AsyncCall<SA>
where
    SA: CallTypeApi,
{
    pub fn with_callback(self, callback_call: CallbackClosure<SA>) -> Self {
        AsyncCall {
            callback_call: Some(callback_call),
            ..self
        }
    }
}

impl<SA> AsyncCall<SA>
where
    SA: CallTypeApi,
{
    pub fn call_and_exit_ignore_callback(&self) -> ! {
        SendRawWrapper::<SA>::new().async_call_raw(
            &self.to,
            &self.egld_payment,
            &self.endpoint_name,
            &self.arg_buffer,
        )
    }
}

impl<SA> AsyncCall<SA>
where
    SA: CallTypeApi + StorageWriteApi,
{
    pub fn call_and_exit(&self) -> ! {
        // first, save the callback closure
        if let Some(callback_call) = &self.callback_call {
            callback_call.save_to_storage::<SA>();
        }

        // last, send the async call, which will kill the execution
        self.call_and_exit_ignore_callback()
    }
}
