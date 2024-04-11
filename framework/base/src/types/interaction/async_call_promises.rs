use crate::{
    api::CallTypeApi,
    contract_base::SendRawWrapper,
    types::{BigUint, CallbackClosure, ManagedAddress, ManagedBuffer},
};

use super::FunctionCall;

/// Will be renamed to `AsyncCall` and `AsyncCall` to `AsyncCallLegacy` when the promises end up on the mainnet.
#[must_use]
pub struct AsyncCallPromises<SA>
where
    SA: CallTypeApi + 'static,
{
    pub(crate) to: ManagedAddress<SA>,
    pub(crate) egld_payment: BigUint<SA>,
    pub(crate) function_call: FunctionCall<SA>,
    pub(crate) explicit_gas_limit: u64,
    pub(crate) extra_gas_for_callback: u64,
    pub(crate) callback_call: Option<CallbackClosure<SA>>,
}

#[allow(clippy::return_self_not_must_use)]
impl<SA> AsyncCallPromises<SA>
where
    SA: CallTypeApi,
{
    pub fn with_callback(self, callback_call: CallbackClosure<SA>) -> Self {
        AsyncCallPromises {
            callback_call: Some(callback_call),
            ..self
        }
    }

    #[inline]
    pub fn with_extra_gas_for_callback(mut self, gas_limit: u64) -> Self {
        self.extra_gas_for_callback = gas_limit;
        self
    }

    pub fn register_promise(self) {
        use crate::{api::const_handles, types::ManagedType};

        let mut cb_closure_args_serialized =
            ManagedBuffer::<SA>::from_raw_handle(const_handles::MBUF_TEMPORARY_1);
        let callback_name;
        if let Some(callback_call) = self.callback_call {
            callback_name = callback_call.callback_name;
            callback_call
                .closure_args
                .serialize_overwrite(&mut cb_closure_args_serialized);
        } else {
            callback_name = "";
            cb_closure_args_serialized.overwrite(&[]);
        }

        SendRawWrapper::<SA>::new().create_async_call_raw(
            &self.to,
            &self.egld_payment,
            &self.function_call.function_name,
            &self.function_call.arg_buffer,
            callback_name,
            callback_name,
            self.explicit_gas_limit,
            self.extra_gas_for_callback,
            &cb_closure_args_serialized,
        )
    }
}
