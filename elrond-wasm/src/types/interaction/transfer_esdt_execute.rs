use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::SendApi;
use crate::io::EndpointResult;
use crate::types::{Address, ArgBuffer, BoxedBytes};
use alloc::string::String;
use alloc::vec::Vec;

#[must_use]
pub struct TransferEsdtExecute<SA>
where
	SA: SendApi + 'static,
{
	pub(super) api: SA,
	pub(super) to: Address,
	pub(super) token_name: BoxedBytes,
	pub(super) amount: SA::AmountType,
	pub(super) endpoint_name: BoxedBytes,
	pub(super) arg_buffer: ArgBuffer,
	pub(super) gas_limit: u64,
}

impl<SA> TransferEsdtExecute<SA>
where
	SA: SendApi + 'static,
{
	pub fn with_gas_limit(self, gas_limit: u64) -> Self {
		TransferEsdtExecute { gas_limit, ..self }
	}
}

impl<FA, SA> EndpointResult<FA> for TransferEsdtExecute<SA>
where
	SA: SendApi + 'static,
{
	#[inline]
	fn finish(&self, _api: FA) {
		let result = self.api.direct_esdt_execute(
			&self.to,
			self.token_name.as_slice(),
			&self.amount,
			self.gas_limit,
			self.endpoint_name.as_slice(),
			&self.arg_buffer,
		);
		if let Err(e) = result {
			self.api.signal_error(e);
		}
	}
}

impl<SA> TypeAbi for TransferEsdtExecute<SA>
where
	SA: SendApi + 'static,
{
	fn type_name() -> String {
		"TransferEsdtExecute".into()
	}

	/// No ABI output.
	fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
		Vec::new()
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
