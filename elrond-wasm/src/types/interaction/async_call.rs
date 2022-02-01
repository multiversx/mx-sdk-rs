use core::marker::PhantomData;

use crate::{
    abi::{OutputAbi, TypeAbi, TypeDescriptionContainer},
    api::{CallTypeApi, SendApiImpl, StorageWriteApi},
    io::EndpointResult,
    types::{BigUint, CallbackClosure, ManagedAddress, ManagedArgBuffer, ManagedBuffer},
};
use alloc::{string::String, vec::Vec};

#[must_use]
pub struct AsyncCall<SA>
where
    SA: CallTypeApi + 'static,
{
    pub(crate) _phantom: PhantomData<SA>,
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

impl<SA> EndpointResult for AsyncCall<SA>
where
    SA: CallTypeApi + StorageWriteApi + 'static,
{
    type DecodeAs = ();

    #[inline]
    fn finish<FA>(&self) {
        // first, save the callback closure
        if let Some(callback_call) = &self.callback_call {
            callback_call.save_to_storage::<SA>();
        }

        // last, send the async call, which will kill the execution
        SA::send_api_impl().async_call_raw(
            &self.to,
            &self.egld_payment,
            &self.endpoint_name,
            &self.arg_buffer,
        );
    }
}

impl<SA> TypeAbi for AsyncCall<SA>
where
    SA: CallTypeApi + 'static,
{
    fn type_name() -> String {
        "AsyncCall".into()
    }

    /// No ABI output.
    fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
        Vec::new()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
