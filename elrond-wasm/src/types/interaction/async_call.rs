use crate::{
    abi::{OutputAbi, TypeAbi, TypeDescriptionContainer},
    api::{ManagedTypeApi, SendApi, StorageWriteApi},
    io::EndpointResult,
    types::{BigUint, CallbackClosure, ManagedAddress, ManagedArgBuffer, ManagedBuffer},
};
use alloc::{string::String, vec::Vec};

#[must_use]
pub struct AsyncCall<SA>
where
    SA: SendApi + ManagedTypeApi + 'static,
{
    pub(crate) api: SA,
    pub(crate) to: ManagedAddress<SA>,
    pub(crate) egld_payment: BigUint<SA>,
    pub(crate) endpoint_name: ManagedBuffer<SA>,
    pub(crate) arg_buffer: ManagedArgBuffer<SA>,
    pub(crate) callback_call: Option<CallbackClosure<SA>>,
}

impl<SA> AsyncCall<SA>
where
    SA: SendApi + 'static,
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
    SA: SendApi + ManagedTypeApi + StorageWriteApi + 'static,
{
    type DecodeAs = ();

    #[inline]
    fn finish<FA>(&self, _api: FA) {
        // first, save the callback closure
        if let Some(callback_call) = &self.callback_call {
            callback_call.save_to_storage(self.api.clone());
        }

        // last, send the async call, which will kill the execution
        self.api.async_call_raw(
            &self.to,
            &self.egld_payment,
            &self.endpoint_name,
            &self.arg_buffer,
        );
    }
}

impl<SA> TypeAbi for AsyncCall<SA>
where
    SA: SendApi + 'static,
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
