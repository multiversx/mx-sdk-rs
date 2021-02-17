use super::{BigUintApi, ErrorApi};
use crate::hex_call_data::HexCallDataSerializer;
use crate::io::AsyncCallArg;
use crate::types::{Address, ArgBuffer, BoxedBytes, CodeMetadata, TokenIdentifier};

pub const ESDT_TRANSFER_STRING: &[u8] = b"ESDTTransfer";

pub trait CallbackContainer<BigUint>
where
	BigUint: BigUintApi + 'static,
{
	type SendApi: SendApi<BigUint> + Clone + 'static;

	fn send_api(&self) -> Self::SendApi;
}

/// API that groups methods that either send EGLD or ESDT, or that call other contracts.
pub trait SendApi<BigUint>: ErrorApi + Sized
where
	BigUint: BigUintApi + 'static,
{
	/// Sends EGLD to a given address, directly.
	/// Used especially for sending EGLD to regular accounts.
	fn direct_egld(&self, to: &Address, amount: &BigUint, data: &[u8]);

	/// Sends an ESDT token to a given address, directly.
	/// Used especially for sending ESDT to regular accounts.
	///
	/// Unlike sending ESDT via async call, this method can be called multiple times per transaction.
	fn direct_esdt(&self, to: &Address, token: &[u8], amount: &BigUint, data: &[u8]) {
		self.direct_esdt_execute(to, token, amount, 0, data, &ArgBuffer::new());
	}

	/// Lower-level version of `direct_esdt`, in which the contract can specify a gas limit.
	/// The gas limit should be 0 for regulat ESDT transfers.
	fn direct_esdt_execute(
		&self,
		to: &Address,
		token: &[u8],
		amount: &BigUint,
		gas_limit: u64,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	);

	/// Sends either EGLD or an ESDT token to the target address,
	/// depending on what token identifier was specified.
	fn direct(&self, to: &Address, token: &TokenIdentifier, amount: &BigUint, data: &[u8]) {
		if token.is_egld() {
			self.direct_egld(to, amount, data);
		} else {
			self.direct_esdt(to, token.as_slice(), amount, data);
		}
	}

	/// Performs a simple ESDT transfer, but via async call.
	/// This is the preferred way to send ESDT.
	fn direct_esdt_via_async_call(
		&self,
		to: &Address,
		esdt_token_name: &[u8],
		amount: &BigUint,
		data: &[u8],
	) -> ! {
		let mut serializer = HexCallDataSerializer::new(ESDT_TRANSFER_STRING);
		serializer.push_argument_bytes(esdt_token_name);
		serializer.push_argument_bytes(amount.to_bytes_be().as_slice());
		if !data.is_empty() {
			serializer.push_argument_bytes(data);
		}
		self.async_call_raw(&to, &BigUint::zero(), serializer.as_slice())
	}

	/// Sends either EGLD or an ESDT token to the target address,
	/// depending on what token identifier was specified.
	/// In case of ESDT it performs an async call.
	fn direct_via_async_call(
		&self,
		to: &Address,
		token: &TokenIdentifier,
		amount: &BigUint,
		data: &[u8],
	) {
		if token.is_egld() {
			self.direct_egld(to, amount, data);
		} else {
			self.direct_esdt_via_async_call(to, token.as_slice(), amount, data);
		}
	}

	/// Sends an asynchronous call to another contract.
	/// Calling this method immediately terminates tx execution.
	/// Using it directly is generally discouraged.
	///
	/// The data is expected to be of the form `functionName@<arg1-hex>@<arg2-hex>@...`.
	/// Use a `HexCallDataSerializer` to prepare this field.
	fn async_call_raw(&self, to: &Address, amount: &BigUint, data: &[u8]) -> !;

	/// Sends an asynchronous call to another contract.
	/// Calling this method immediately terminates tx execution.
	///
	/// Even though it only accepts 1 argument, this is not restrictive, since the argument can be a MultiArg.
	/// For instance, to pass 3 arguments, pass a `MultiArg3<Type1, Type2, Type3>`.
	/// To pass a variable number of arguments, pass VarArgs.
	fn async_call_egld<A: AsyncCallArg>(
		&self,
		to: &Address,
		payment: &BigUint,
		endpoint_name: &[u8],
		arg: A,
	) -> ! {
		let mut serializer = HexCallDataSerializer::new(endpoint_name);
		// TODO: fast exit is also required here
		if let Err(serialization_err) = arg.push_async_arg(&mut serializer) {
			self.signal_error(serialization_err.as_bytes());
		}
		self.async_call_raw(&to, &payment, serializer.as_slice())
	}

	/// Sends an asynchronous call to another contract, by first invoking `ESDTTransfer`.
	/// This allows to first pass an ESDT token to the endpoint.
	/// Cannot send EGLD using this method.
	/// Calling this method immediately terminates tx execution.
	///
	/// Even though it only accepts 1 argument, this is not restrictive, since the argument can be a MultiArg.
	/// For instance, to pass 3 arguments, pass a `MultiArg3<Type1, Type2, Type3>`.
	/// To pass a variable number of arguments, pass VarArgs.
	fn async_call_esdt<A: AsyncCallArg>(
		&self,
		to: &Address,
		esdt_payment: &BigUint,
		esdt_token_name: &[u8],
		endpoint_name: &[u8],
		arg: A,
	) -> ! {
		let mut serializer = HexCallDataSerializer::new(ESDT_TRANSFER_STRING);
		serializer.push_argument_bytes(esdt_token_name);
		serializer.push_argument_bytes(esdt_payment.to_bytes_be().as_slice());
		serializer.push_argument_bytes(endpoint_name);
		// TODO: fast exit is also required here
		if let Err(serialization_err) = arg.push_async_arg(&mut serializer) {
			self.signal_error(serialization_err.as_bytes());
		}
		self.async_call_raw(&to, &BigUint::zero(), serializer.as_slice())
	}

	/// Sends an asynchronous call to another contract, with either EGLD or ESDT value.
	/// The `token` argument decides which one it will be.
	/// Calling this method immediately terminates tx execution.
	///
	/// Even though it only accepts 1 argument, this is not restrictive, since the argument can be a MultiArg.
	/// For instance, to pass 3 arguments, pass a `MultiArg3<Type1, Type2, Type3>`.
	/// To pass a variable number of arguments, pass VarArgs.
	fn async_call<A: AsyncCallArg>(
		&self,
		to: &Address,
		token: &TokenIdentifier,
		payment: &BigUint,
		endpoint_name: &[u8],
		arg: A,
	) -> ! {
		if token.is_egld() {
			self.async_call_egld(to, payment, endpoint_name, arg)
		} else {
			self.async_call_esdt(to, payment, token.as_slice(), endpoint_name, arg);
		}
	}

	/// Deploys a new contract in the same shard.
	/// Unlike `async_call_raw`, the deployment is synchronous and tx execution continues afterwards.
	/// Also unlike `async_call_raw`, it uses an argument buffer to pass arguments
	fn deploy_contract(
		&self,
		gas: u64,
		amount: &BigUint,
		code: &BoxedBytes,
		code_metadata: CodeMetadata,
		arg_buffer: &ArgBuffer,
	) -> Address;

	fn execute_on_dest_context(
		&self,
		gas: u64,
		address: &Address,
		value: &BigUint,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	);

	fn execute_on_dest_context_by_caller(
		&self,
		gas: u64,
		address: &Address,
		value: &BigUint,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	);

	fn execute_on_same_context(
		&self,
		gas: u64,
		address: &Address,
		value: &BigUint,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	);

	/// Used to store data between async call and callback.
	fn storage_store_tx_hash_key(&self, data: &[u8]);

	/// Used to store data between async call and callback.
	fn storage_load_tx_hash_key(&self) -> BoxedBytes;
}
