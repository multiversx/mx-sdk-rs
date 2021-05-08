use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::SendApi;
use crate::io::EndpointResult;
use crate::types::{Address, BoxedBytes};
use alloc::string::String;
use alloc::vec::Vec;

pub struct SendEsdt<SA>
where
	SA: SendApi + 'static,
{
	pub(super) api: SA,
	pub(super) to: Address,
	pub(super) token_name: BoxedBytes,
	pub(super) amount: SA::AmountType,
	pub(super) data: BoxedBytes,
}

impl<FA, SA> EndpointResult<FA> for SendEsdt<SA>
where
	SA: SendApi + 'static,
{
	#[inline]
	fn finish(&self, _api: FA) {
		self.api.direct_esdt_via_async_call(
			&self.to,
			&self.token_name.as_slice(),
			&self.amount,
			self.data.as_slice(),
		);
	}
}

impl<SA> TypeAbi for SendEsdt<SA>
where
	SA: SendApi + 'static,
{
	fn type_name() -> String {
		"SendEsdt".into()
	}

	/// No ABI output.
	fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
		Vec::new()
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
