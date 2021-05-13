use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::SendApi;
use crate::io::EndpointResult;
use crate::types::{Address, ArgBuffer, BoxedBytes};
use alloc::string::String;
use alloc::vec::Vec;

#[must_use]
pub struct TransferEgldExecute<SA>
where
	SA: SendApi + 'static,
{
	pub(super) api: SA,
	pub(super) to: Address,
	pub(super) egld_payment: SA::AmountType,
	pub(super) endpoint_name: BoxedBytes,
	pub(super) arg_buffer: ArgBuffer,
	pub(super) gas_limit: u64,
}

impl<SA> TransferEgldExecute<SA>
where
	SA: SendApi + 'static,
{
	pub fn with_gas_limit(self, gas_limit: u64) -> Self {
		TransferEgldExecute { gas_limit, ..self }
	}
}

impl<SA> EndpointResult for TransferEgldExecute<SA>
where
	SA: SendApi + 'static,
{
	type DecodeAs = ();

	#[inline]
	fn finish<FA>(&self, _api: FA) {
		let result = self.api.direct_egld_execute(
			&self.to,
			&self.egld_payment,
			self.gas_limit,
			self.endpoint_name.as_slice(),
			&self.arg_buffer,
		);
		if let Err(e) = result {
			self.api.signal_error(e);
		}
	}
}

impl<SA> TypeAbi for TransferEgldExecute<SA>
where
	SA: SendApi + 'static,
{
	fn type_name() -> String {
		"TransferEgldExecute".into()
	}

	/// No ABI output.
	fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
		Vec::new()
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
