use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::SendApi;
use crate::hex_call_data::HexCallDataSerializer;
use crate::io::EndpointResult;
use crate::types::{Address, CallbackCall};
use alloc::string::String;
use alloc::vec::Vec;

#[must_use]
pub struct AsyncCall<SA>
where
    SA: SendApi + 'static,
{
    pub(crate) api: SA,
    pub(crate) to: Address,
    pub(crate) egld_payment: SA::AmountType,
    pub(crate) hex_data: HexCallDataSerializer,
    pub(crate) callback_data: HexCallDataSerializer,
}

impl<SA> AsyncCall<SA>
where
    SA: SendApi + 'static,
{
    pub fn with_callback(self, callback: CallbackCall) -> Self {
        AsyncCall {
            callback_data: callback.closure_data,
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
        self.api
            .storage_store_tx_hash_key(self.callback_data.as_slice());

        // last, send the async call, which will kill the execution
        self.api
            .async_call_raw(&self.to, &self.egld_payment, self.hex_data.as_slice());
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
