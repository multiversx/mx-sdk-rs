use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::{BigUintApi, ErrorApi, SendApi};
use crate::hex_call_data::HexCallDataSerializer;
use crate::io::EndpointResult;
use crate::types::{Address, CallbackCall};
use alloc::string::String;
use alloc::vec::Vec;

#[must_use]
pub struct AsyncCall<BigUint: BigUintApi> {
	pub(crate) to: Address,
	pub(crate) egld_payment: BigUint,
	pub(crate) hex_data: HexCallDataSerializer,
	pub(crate) callback_data: HexCallDataSerializer,
}

impl<BigUint: BigUintApi> AsyncCall<BigUint> {
	pub fn with_callback(self, callback: CallbackCall) -> Self {
		AsyncCall {
			callback_data: callback.closure_data,
			..self
		}
	}
}

impl<FA, BigUint> EndpointResult<FA> for AsyncCall<BigUint>
where
	BigUint: BigUintApi + 'static,
	FA: SendApi<AmountType = BigUint> + ErrorApi + Clone + 'static,
{
	#[inline]
	fn finish(&self, api: FA) {
		// first, save the callback closure
		api.storage_store_tx_hash_key(self.callback_data.as_slice());

		// last, send the async call, which will kill the execution
		api.async_call_raw(&self.to, &self.egld_payment, self.hex_data.as_slice());
	}
}

impl<BigUint: BigUintApi> TypeAbi for AsyncCall<BigUint> {
	fn type_name() -> String {
		"AsyncCall".into()
	}

	/// No ABI output.
	fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
		Vec::new()
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
