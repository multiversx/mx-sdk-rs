use crate::{TokenIdentifier, abi::{OutputAbi, TypeAbi, TypeDescriptionContainer}};
use crate::api::{BigUintApi, ErrorApi, SendApi, ESDT_TRANSFER_STRING};
use crate::hex_call_data::HexCallDataSerializer;
use crate::io::AsyncCallArg;
use crate::io::EndpointResult;
use crate::types::{Address, SCError};
use alloc::string::String;
use alloc::vec::Vec;

pub struct AsyncCall<BigUint: BigUintApi> {
	to: Address,
	egld_payment: BigUint,
	hex_data: HexCallDataSerializer,
}

impl<BigUint: BigUintApi> AsyncCall<BigUint> {
	pub fn new(to: Address, egld_payment: BigUint, endpoint_name: &[u8]) -> Self {
		AsyncCall {
			to,
			egld_payment,
			hex_data: HexCallDataSerializer::new(endpoint_name),
		}
	}

	pub fn with_esdt(
		to: Address,
		esdt_token_name: &[u8],
		esdt_payment: &BigUint,
		endpoint_name: &[u8],
	) -> Self {
		let mut hex_data = HexCallDataSerializer::new(ESDT_TRANSFER_STRING);
		hex_data.push_argument_bytes(esdt_token_name);
		hex_data.push_argument_bytes(esdt_payment.to_bytes_be().as_slice());
		hex_data.push_argument_bytes(endpoint_name);
		AsyncCall {
			to,
			egld_payment: BigUint::zero(),
			hex_data,
		}
	}

	pub fn with_token_payment(
		to: Address,
		token: TokenIdentifier,
		payment: BigUint,
		endpoint_name: &[u8],
	) -> Self {
		if token.is_egld() {
			Self::new(to, payment, endpoint_name)
		} else {
			Self::with_esdt(to, token.as_slice(), &payment, endpoint_name)
		}
	}

	pub fn push_argument_raw_bytes(&mut self, bytes: &[u8]) {
		self.hex_data.push_argument_bytes(bytes);
	}

	pub fn push_argument_or_exit<A: AsyncCallArg, ExitCtx: Clone>(
		&mut self,
		arg: A,
		c: ExitCtx,
		exit: fn(ExitCtx, SCError) -> !,
	) {
		// TODO: propagate fast exit down the serializer
		// TODO: also expose an EncodingError instead of the SCError
		if let Err(serialization_err) = arg.push_async_arg(&mut self.hex_data) {
			exit(c, serialization_err);
		}
	}
}

impl<FA, BigUint> EndpointResult<FA> for AsyncCall<BigUint>
where
	BigUint: BigUintApi + 'static,
	FA: SendApi<BigUint> + ErrorApi + Clone + 'static,
{
	#[inline]
	fn finish(&self, api: FA) {
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
