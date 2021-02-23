use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::{BigUintApi, ErrorApi, SendApi};
use crate::io::EndpointResult;
use crate::types::{Address, ArgBuffer, BoxedBytes, TokenIdentifier};
use alloc::string::String;
use alloc::vec::Vec;

#[must_use]
pub struct TransferExecute<BigUint: BigUintApi> {
	pub(super) to: Address,
	pub(super) token: TokenIdentifier,
	pub(super) amount: BigUint,
	pub(super) endpoint_name: BoxedBytes,
	pub(super) arg_buffer: ArgBuffer,
	pub(super) gas_limit: u64,
}

impl<BigUint> TransferExecute<BigUint>
where
	BigUint: BigUintApi + 'static,
{
	pub fn with_gas_limit(self, gas_limit: u64) -> Self {
		TransferExecute { gas_limit, ..self }
	}
}

impl<FA, BigUint> EndpointResult<FA> for TransferExecute<BigUint>
where
	BigUint: BigUintApi + 'static,
	FA: SendApi<BigUint> + ErrorApi + Clone + 'static,
{
	#[inline]
	fn finish(&self, api: FA) {
		if self.token.is_egld() {
			api.direct_egld_execute(
				&self.to,
				&self.amount,
				self.gas_limit,
				self.endpoint_name.as_slice(),
				&self.arg_buffer,
			);
		} else {
			api.direct_esdt_execute(
				&self.to,
				self.token.as_slice(),
				&self.amount,
				self.gas_limit,
				self.endpoint_name.as_slice(),
				&self.arg_buffer,
			);
		}
	}
}

impl<BigUint: BigUintApi> TypeAbi for TransferExecute<BigUint> {
	fn type_name() -> String {
		"TransferExecute".into()
	}

	/// No ABI output.
	fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
		Vec::new()
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
