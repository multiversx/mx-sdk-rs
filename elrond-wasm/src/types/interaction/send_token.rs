use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::{BigUintApi, EndpointFinishApi, ErrorApi, SendApi};
use crate::io::EndpointResult;
use crate::types::{Address, BoxedBytes, TokenIdentifier};
use alloc::string::String;
use alloc::vec::Vec;

pub struct SendToken<BigUint: BigUintApi> {
	pub to: Address,
	pub token: TokenIdentifier,
	pub amount: BigUint,
	pub data: BoxedBytes,
}

impl<FA, BigUint> EndpointResult<FA> for SendToken<BigUint>
where
	BigUint: BigUintApi + 'static,
	FA: EndpointFinishApi + SendApi<BigUint> + ErrorApi + Clone + 'static,
{
	#[inline]
	fn finish(&self, api: FA) {
		api.direct_via_async_call(&self.to, &self.token, &self.amount, self.data.as_slice());
	}
}

impl<BigUint: BigUintApi> TypeAbi for SendToken<BigUint> {
	fn type_name() -> String {
		"SendToken".into()
	}

	/// No ABI output.
	fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
		Vec::new()
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
