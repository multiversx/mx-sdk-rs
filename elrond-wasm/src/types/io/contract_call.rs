use crate::api::{BigUintApi, ESDT_TRANSFER_STRING};
use crate::hex_call_data::HexCallDataSerializer;
use crate::io::AsyncCallArg;
use crate::types::{Address, AsyncCall, SCError};
use crate::TokenIdentifier;

/// Represents metadata for calling another contract.
/// Can transform into either an async call, transfer call or other types of calls.
#[must_use]
pub struct ContractCall<BigUint: BigUintApi> {
	to: Address,
	egld_payment: BigUint,
	pub hex_data: HexCallDataSerializer, // TODO: make private and find a better way to serialize
}

impl<BigUint: BigUintApi> ContractCall<BigUint> {
	pub fn new_egld(to: Address, egld_payment: BigUint, endpoint_name: &[u8]) -> Self {
		ContractCall {
			to,
			egld_payment,
			hex_data: HexCallDataSerializer::new(endpoint_name),
		}
	}

	pub fn new_esdt(
		to: Address,
		esdt_token_name: &[u8],
		esdt_payment: &BigUint,
		endpoint_name: &[u8],
	) -> Self {
		let mut hex_data = HexCallDataSerializer::new(ESDT_TRANSFER_STRING);
		hex_data.push_argument_bytes(esdt_token_name);
		hex_data.push_argument_bytes(esdt_payment.to_bytes_be().as_slice());
		hex_data.push_argument_bytes(endpoint_name);
		ContractCall {
			to,
			egld_payment: BigUint::zero(),
			hex_data,
		}
	}

	pub fn new(
		to: Address,
		token: TokenIdentifier,
		payment: BigUint,
		endpoint_name: &[u8],
	) -> Self {
		if token.is_egld() {
			Self::new_egld(to, payment, endpoint_name)
		} else {
			Self::new_esdt(to, token.as_slice(), &payment, endpoint_name)
		}
	}

	pub fn push_argument_raw_bytes(&mut self, bytes: &[u8]) {
		self.hex_data.push_argument_bytes(bytes);
	}

	pub fn push_callback_argument_raw_bytes(&mut self, bytes: &[u8]) {
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

	pub fn async_call(self) -> AsyncCall<BigUint> {
		AsyncCall {
			to: self.to,
			egld_payment: self.egld_payment,
			hex_data: self.hex_data,
			callback_data: HexCallDataSerializer::new(&[]),
		}
	}
}
