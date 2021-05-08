use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::SendApi;
use crate::io::EndpointResult;
use crate::types::{Address, BoxedBytes, TokenIdentifier};
use alloc::string::String;
use alloc::vec::Vec;

pub struct SendToken<SA>
where
	SA: SendApi + 'static,
{
	pub api: SA,
	pub to: Address,
	pub token: TokenIdentifier,
	pub amount: SA::AmountType,
	pub data: BoxedBytes,
}

impl<FA, SA> EndpointResult<FA> for SendToken<SA>
where
	SA: SendApi + 'static,
{
	#[inline]
	fn finish(&self, _api: FA) {
		self.api
			.direct_via_async_call(&self.to, &self.token, &self.amount, self.data.as_slice());
	}
}

impl<SA> TypeAbi for SendToken<SA>
where
	SA: SendApi + 'static,
{
	fn type_name() -> String {
		"SendToken".into()
	}

	/// No ABI output.
	fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
		Vec::new()
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
