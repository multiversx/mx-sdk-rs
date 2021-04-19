use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::{BigUintApi, ErrorApi, SendApi};
use crate::io::EndpointResult;
use crate::types::{Address, ArgBuffer, BoxedBytes};
use alloc::string::String;
use alloc::vec::Vec;

#[must_use]
pub struct TransferEsdtExecute<BigUint: BigUintApi> {
	pub(super) to: Address,
	pub(super) token_name: BoxedBytes,
	pub(super) amount: BigUint,
	pub(super) endpoint_name: BoxedBytes,
	pub(super) arg_buffer: ArgBuffer,
	pub(super) gas_limit: u64,
}

impl<BigUint> TransferEsdtExecute<BigUint>
where
	BigUint: BigUintApi + 'static,
{
	pub fn with_gas_limit(self, gas_limit: u64) -> Self {
		TransferEsdtExecute { gas_limit, ..self }
	}
}

impl<FA, BigUint> EndpointResult<FA> for TransferEsdtExecute<BigUint>
where
	BigUint: BigUintApi + 'static,
	FA: SendApi<BigUint> + ErrorApi + Clone + 'static,
{
	#[inline]
	fn finish(&self, api: FA) {
		let result = api.direct_esdt_execute(
			&self.to,
			self.token_name.as_slice(),
			&self.amount,
			self.gas_limit,
			self.endpoint_name.as_slice(),
			&self.arg_buffer,
		);
		if let Err(e) = result {
			api.signal_error(e);
		}
	}
}

impl<BigUint: BigUintApi> TypeAbi for TransferEsdtExecute<BigUint> {
	fn type_name() -> String {
		"TransferEsdtExecute".into()
	}

	/// No ABI output.
	fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
		Vec::new()
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
