use crate::{
    api::{CallTypeApi, StorageWriteApi},
    contract_base::SendRawWrapper,
    types::{BigUint, CallbackClosure, EgldPayment, ManagedAddress},
};

use super::{FunctionCall, Tx, TxAsyncCallCallback, TxScEnv};

pub type AsyncCall<Api> = Tx<
    TxScEnv<Api>,
    (),
    ManagedAddress<Api>,
    EgldPayment<Api>,
    (),
    FunctionCall<Api>,
    Option<CallbackClosure<Api>>,
>;

#[allow(clippy::return_self_not_must_use)]
impl<Api> AsyncCall<Api>
where
    Api: CallTypeApi,
{
    pub fn with_callback(mut self, callback_call: CallbackClosure<Api>) -> Self {
        self.result_handler = Some(callback_call);
        self
    }
}

impl<Api> AsyncCall<Api>
where
    Api: CallTypeApi + StorageWriteApi,
{
    pub fn call_and_exit_ignore_callback(self) -> ! {
        self.async_call_and_exit()
    }
}

impl<Api> AsyncCall<Api>
where
    Api: CallTypeApi + StorageWriteApi,
{
    pub fn call_and_exit(self) -> ! {
        self.async_call_and_exit()
    }
}
