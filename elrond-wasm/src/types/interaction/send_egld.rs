use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::SendApi;
use crate::io::EndpointResult;
use crate::types::{Address, BoxedBytes};
use alloc::string::String;
use alloc::vec::Vec;

pub struct SendEgld<SA>
where
	SA: SendApi + 'static,
{
	pub api: SA,
	pub to: Address,
	pub amount: SA::AmountType,
	pub data: BoxedBytes,
}

impl<FA, SA> EndpointResult<FA> for SendEgld<SA>
where
	SA: SendApi + 'static,
{
	#[inline]
	fn finish(&self, _api: FA) {
		self.api
			.direct_egld(&self.to, &self.amount, self.data.as_slice());
	}
}

impl<SA> TypeAbi for SendEgld<SA>
where
	SA: SendApi + 'static,
{
	fn type_name() -> String {
		"SendEgld".into()
	}

	/// No ABI output.
	fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
		Vec::new()
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
