use crate::api::{BigUintApi, SendApi};
use crate::types::{Address, ArgBuffer, BoxedBytes, CodeMetadata};

/// Using max u64 to represent maximum possible gas,
/// so that the value zero is not reserved and can be specified explicitly.
/// Leaving the gas limit unspecified will replace it with `api.get_gas_left()`.
const UNSPECIFIED_GAS_LIMIT: u64 = u64::MAX;

#[must_use]
pub struct ContractDeploy<SA>
where
	SA: SendApi + 'static,
{
	api: SA,
	code: BoxedBytes,
	code_metadata: CodeMetadata,
	payment_amount: SA::AmountType,
	explicit_gas_limit: u64,
	pub arg_buffer: ArgBuffer, // TODO: make private and find a better way to serialize
}

/// Syntactical sugar to help macros to generate code easier.
/// Unlike calling `ContractDeploy::<SA>::new`, here types can be inferred from the context.
pub fn new_contract_deploy<SA>(
	api: SA,
	payment_amount: SA::AmountType,
) -> ContractDeploy<SA>
where
	SA: SendApi + 'static,
{
	let mut contract_deploy = ContractDeploy::<SA>::new(api);
	contract_deploy.payment_amount = payment_amount;

	contract_deploy
}

impl<SA> ContractDeploy<SA>
where
	SA: SendApi + 'static,
{
	pub fn new(api: SA) -> Self {
		ContractDeploy {
			api,
			code: BoxedBytes::empty(),
			code_metadata: CodeMetadata::DEFAULT,
			payment_amount: SA::AmountType::zero(),
			explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
			arg_buffer: ArgBuffer::new(),
		}
	}

	pub fn with_code(mut self, code: BoxedBytes, code_metadata: CodeMetadata) -> Self {
		self.code = code;
		self.code_metadata = code_metadata;
		self
	}

	pub fn with_egld_transfer(mut self, payment_amount: SA::AmountType) -> Self {
		self.payment_amount = payment_amount;
		self
	}

	pub fn with_gas_limit(mut self, gas_limit: u64) -> Self {
		self.explicit_gas_limit = gas_limit;
		self
	}

	pub fn get_mut_arg_buffer(&mut self) -> &mut ArgBuffer {
		&mut self.arg_buffer
	}

	/// Provided for cases where we build the contract deploy by hand.
	pub fn push_argument_raw_bytes(&mut self, bytes: &[u8]) {
		self.arg_buffer.push_argument_bytes(bytes);
	}

	fn resolve_gas_limit(&self) -> u64 {
		if self.explicit_gas_limit == UNSPECIFIED_GAS_LIMIT {
			self.api.get_gas_left()
		} else {
			self.explicit_gas_limit
		}
	}
}

impl<SA> ContractDeploy<SA>
where
	SA: SendApi + 'static,
{
	/// Executes immediately, synchronously, and returns Some(Address) of the deployed contract.  
	/// Will return None if the deploy fails.  
	pub fn execute(self) -> Option<Address> {
		let address = self.api.deploy_contract(
			self.resolve_gas_limit(),
			&self.payment_amount,
			&self.code,
			self.code_metadata,
			&self.arg_buffer,
		);

		if address.is_zero() {
			None
		} else {
			Some(address)
		}
	}
}
