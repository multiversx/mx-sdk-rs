use crate::{
    api::{CallTypeApi, StorageWriteApi},
    contract_base::SendRawWrapper,
    types::{BigUint, CallbackClosure, ManagedAddress},
};

use super::FunctionCall;

#[must_use]
pub struct AsyncCall<'a, SA>
where
    SA: CallTypeApi<'a> + 'static,
{
    pub(crate) to: ManagedAddress<'a, SA>,
    pub(crate) egld_payment: BigUint<'a, SA>,
    pub(crate) function_call: FunctionCall<'a, SA>,
    pub(crate) callback_call: Option<CallbackClosure<'a, SA>>,
}

#[allow(clippy::return_self_not_must_use)]
impl<'a, SA> AsyncCall<'a, SA>
where
    SA: CallTypeApi<'a>,
{
    pub fn with_callback(self, callback_call: CallbackClosure<'a, SA>) -> Self {
        AsyncCall {
            callback_call: Some(callback_call),
            ..self
        }
    }
}

impl<'a, SA> AsyncCall<'a, SA>
where
    SA: CallTypeApi<'a>,
{
    pub fn call_and_exit_ignore_callback(&self) -> ! {
        SendRawWrapper::<'a, SA>::new().async_call_raw(
            &self.to,
            &self.egld_payment,
            &self.function_call.function_name,
            &self.function_call.arg_buffer,
        )
    }
}

impl<'a, SA> AsyncCall<'a, SA>
where
    SA: CallTypeApi<'a> + StorageWriteApi,
{
    pub fn call_and_exit(&self) -> ! {
        // first, save the callback closure
        if let Some(callback_call) = &self.callback_call {
            callback_call.save_to_storage::<'a, SA>();
        }

        // last, send the async call, which will kill the execution
        self.call_and_exit_ignore_callback()
    }
}
