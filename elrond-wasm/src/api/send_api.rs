use super::BigUintApi;
use crate::types::{Address, ArgBuffer, BoxedBytes, CodeMetadata, TokenIdentifier};

const DIRECT_ESDT_GAS_LIMIT: u64 = 0;

/// API that groups methods that either send EGLD or ESDT, or that call other contracts.
pub trait SendApi<BigUint>: Sized
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
	#[inline]
	fn direct_esdt(&self, to: &Address, token: &[u8], amount: &BigUint, data: &[u8]) {
		self.direct_esdt_explicit_gas_limit(to, token, amount, DIRECT_ESDT_GAS_LIMIT, data);
	}

	/// Lower-level version of `direct_esdt`, in which the contract can specify a gas limit.
	/// The gas limit should be 0 for regulat ESDT transfers.
	fn direct_esdt_explicit_gas_limit(
		&self,
		to: &Address,
		token: &[u8],
		amount: &BigUint,
		gas_limit: u64,
		data: &[u8],
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

	/// Sends an asynchronous call to another contract.
	/// Calling this method immediately terminates tx execution.
	/// Using it directly is generally discouraged.
	///
	/// The data is expected to be of the form `functionName@<arg1-hex>@<arg2-hex>@...`.
	/// Use a `HexCallDataSerializer` to prepare this field.
	fn async_call_raw(&self, to: &Address, amount: &BigUint, data: &[u8]);

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
}
