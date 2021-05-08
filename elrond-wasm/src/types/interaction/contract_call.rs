use crate::types::{
	Address, ArgBuffer, AsyncCall, BoxedBytes, TokenIdentifier, TransferEgldExecute,
	TransferEsdtExecute, TransferExecute,
};
use crate::{
	api::{BigUintApi, SendApi, ESDT_TRANSFER_STRING},
	BytesArgLoader, DynArg,
};
use crate::{hex_call_data::HexCallDataSerializer, ArgId};
use core::marker::PhantomData;

/// Represents metadata for calling another contract.
/// Can transform into either an async call, transfer call or other types of calls.
#[must_use]
pub struct ContractCall<SA, R>
where
	SA: SendApi + 'static,
{
	api: SA,
	to: Address,
	token: TokenIdentifier,
	payment: SA::AmountType,
	endpoint_name: BoxedBytes,
	pub arg_buffer: ArgBuffer, // TODO: make private and find a better way to serialize
	_return_type: PhantomData<R>,
}

pub fn new_contract_call<SA, R>(
	api: SA,
	to: Address,
	token: TokenIdentifier,
	payment: SA::AmountType,
	endpoint_name: BoxedBytes,
) -> ContractCall<SA, R>
where
	SA: SendApi + 'static,
{
	ContractCall::<SA, R>::new(api, to, token, payment, endpoint_name)
}

impl<SA, R> ContractCall<SA, R>
where
	SA: SendApi + 'static,
{
	pub fn new(
		api: SA,
		to: Address,
		token: TokenIdentifier,
		payment: SA::AmountType,
		endpoint_name: BoxedBytes,
	) -> Self {
		ContractCall {
			api,
			to,
			token,
			payment,
			endpoint_name,
			arg_buffer: ArgBuffer::new(),
			_return_type: PhantomData,
		}
	}

	pub fn get_mut_arg_buffer(&mut self) -> &mut ArgBuffer {
		&mut self.arg_buffer
	}

	/// Provided for cases where we build the contract call by hand.
	pub fn push_argument_raw_bytes(&mut self, bytes: &[u8]) {
		self.arg_buffer.push_argument_bytes(bytes);
	}

	/// If this is an ESDT call, it converts it to a regular call to ESDTTransfer.
	/// Async calls require this step, but not `transfer_esdt_execute`.
	fn convert_to_esdt_transfer_call(self) -> Self {
		if !self.token.is_egld() {
			let mut new_arg_buffer = ArgBuffer::new();
			new_arg_buffer.push_argument_bytes(self.token.as_esdt_identifier());
			new_arg_buffer.push_argument_bytes(self.payment.to_bytes_be().as_slice());
			new_arg_buffer.push_argument_bytes(self.endpoint_name.as_slice());

			ContractCall {
				api: self.api,
				to: self.to,
				token: TokenIdentifier::egld(),
				payment: SA::AmountType::zero(),
				endpoint_name: BoxedBytes::from(ESDT_TRANSFER_STRING),
				arg_buffer: new_arg_buffer.concat(self.arg_buffer),
				_return_type: PhantomData,
			}
		} else {
			self
		}
	}

	pub fn async_call(mut self) -> AsyncCall<SA> {
		self = self.convert_to_esdt_transfer_call();
		AsyncCall {
			api: self.api,
			to: self.to,
			egld_payment: self.payment,
			hex_data: HexCallDataSerializer::from_arg_buffer(
				self.endpoint_name.as_slice(),
				&self.arg_buffer,
			),
			callback_data: HexCallDataSerializer::new(&[]),
		}
	}

	/// Produces an EGLD (or no value) transfer-execute call, no callback.
	/// Will always result in a `transferValueExecute` call.
	pub fn transfer_egld_execute(self) -> TransferEgldExecute<SA::AmountType> {
		TransferEgldExecute {
			to: self.to,
			egld_payment: self.payment,
			endpoint_name: self.endpoint_name,
			arg_buffer: self.arg_buffer,
			gas_limit: 0,
		}
	}

	/// Produces an ESDT transfer-execute call, no callback.
	/// Will always result in a `transferESDTExecute` call.
	pub fn transfer_esdt_execute(self) -> TransferEsdtExecute<SA::AmountType> {
		TransferEsdtExecute {
			to: self.to,
			token_name: self.token.into_boxed_bytes(),
			amount: self.payment,
			endpoint_name: self.endpoint_name,
			arg_buffer: self.arg_buffer,
			gas_limit: 0,
		}
	}

	/// Produces a transfer-execute call, no callback.
	/// Will result in either a `transferValueExecute` or a `transferESDTExecute` call, depending on input.
	pub fn transfer_execute(self) -> TransferExecute<SA::AmountType> {
		TransferExecute {
			to: self.to,
			token: self.token,
			amount: self.payment,
			endpoint_name: self.endpoint_name,
			arg_buffer: self.arg_buffer,
			gas_limit: 0,
		}
	}
}

impl<SA, R> ContractCall<SA, R>
where
	SA: SendApi + 'static,
	R: DynArg,
{
	/// Executes immediately, synchronously, and returns contract call result.
	/// Only works if the target contract is in the same shard.
	pub fn execute_on_dest_context(mut self, gas: u64) -> R {
		self = self.convert_to_esdt_transfer_call();
		let raw_result = self.api.execute_on_dest_context_raw(
			gas,
			&self.to,
			&self.payment,
			self.endpoint_name.as_slice(),
			&self.arg_buffer,
		);

		let mut loader = BytesArgLoader::new(raw_result.as_slice(), self.api);
		R::dyn_load(&mut loader, ArgId::from(&b"sync result"[..]))
	}

	/// Executes immediately, synchronously, and returns contract call result.
	/// Only works if the target contract is in the same shard.
	/// This is a workaround to handle nested sync calls.
	/// Please do not use this method unless there is absolutely no other option.
	/// Will be eliminated after some future Arwen hook redesign.
	/// `range_closure` takes the number of results before, the number of results after,
	/// and is expected to return the start index (inclusive) and end index (exclusive).
	pub fn execute_on_dest_context_custom_range<F>(mut self, gas: u64, range_closure: F) -> R
	where
		F: FnOnce(usize, usize) -> (usize, usize),
	{
		self = self.convert_to_esdt_transfer_call();
		let raw_result = self.api.execute_on_dest_context_raw_custom_result_range(
			gas,
			&self.to,
			&self.payment,
			self.endpoint_name.as_slice(),
			&self.arg_buffer,
			range_closure,
		);

		let mut loader = BytesArgLoader::new(raw_result.as_slice(), self.api);
		R::dyn_load(&mut loader, ArgId::from(&b"sync result"[..]))
	}
}
