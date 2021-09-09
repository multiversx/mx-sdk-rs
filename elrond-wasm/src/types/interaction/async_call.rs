use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::{ManagedTypeApi, SendApi};
use crate::io::EndpointResult;
use crate::types::{BigUint, CallbackCall, ManagedAddress, ManagedArgBuffer, ManagedBuffer};
use alloc::string::String;
use alloc::vec::Vec;

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
    pub(crate) callback_call: Option<CallbackCall<SA>>,
}

impl<SA> AsyncCall<SA>
where
    SA: SendApi + 'static,
{
    pub fn with_callback(self, callback_call: CallbackCall<SA>) -> Self {
        AsyncCall {
            callback_call: Some(callback_call),
            ..self
        }
    }
}

impl<SA> EndpointResult for AsyncCall<SA>
where
    SA: SendApi + 'static,
{
    type DecodeAs = ();

    #[inline]
    fn finish<FA>(&self, _api: FA) {
        // first, save the callback closure
        if let Some(callback_call) = &self.callback_call {
            let hex_cd_ser = callback_call.serialize_hex_call_data();
            self.api
                .storage_store_tx_hash_key(&ManagedBuffer::new_from_bytes(
                    self.api.clone(),
                    hex_cd_ser.as_slice(),
                ));
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
