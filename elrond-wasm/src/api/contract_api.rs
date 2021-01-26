use super::{
	BigIntApi, BigUintApi, CallValueApi, CryptoApi, ErrorApi, StorageReadApi, StorageWriteApi,
};
use crate::storage;
use crate::types::{Address, ArgBuffer, BoxedBytes, CodeMetadata, H256};
use alloc::boxed::Box;

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
pub trait ContractHookApi<BigInt, BigUint>: Sized + CryptoApi
where
	BigInt: BigIntApi<BigUint> + 'static,
	BigUint: BigUintApi + 'static,
{
	/// Abstracts the lower-level storage functionality.
	type Storage: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static;

	/// Abstracts the call value handling at the beginning of a function call.
	type CallValue: CallValueApi<BigUint> + ErrorApi + Clone + 'static;

	/// Gateway into the lower-level storage functionality.
	/// Storage related annotations make use of this.
	/// Using it directly is not recommended.
	fn get_storage_raw(&self) -> Self::Storage;

	/// Gateway into the call value retrieval functionality.
	/// The payment annotations should normally be the ones to handle this,
	/// but the developer is also given direct access to the API.
	fn call_value(&self) -> Self::CallValue;

	fn get_sc_address(&self) -> Address;

	fn get_owner_address(&self) -> Address;

	fn get_caller(&self) -> Address;

	fn get_balance(&self, address: &Address) -> BigUint;

	fn get_sc_balance(&self) -> BigUint {
		self.get_balance(&self.get_sc_address())
	}

	#[inline]
	fn storage_load_cumulated_validator_reward(&self) -> BigUint {
		storage::storage_get(
			self.get_storage_raw(),
			storage::protected_keys::ELROND_REWARD_KEY,
		)
	}

	fn send_tx(&self, to: &Address, amount: &BigUint, data: &[u8]);

	fn async_call(&self, to: &Address, amount: &BigUint, data: &[u8]);

	fn deploy_contract(
		&self,
		gas: u64,
		amount: &BigUint,
		code: &BoxedBytes,
		code_metadata: CodeMetadata,
		arg_buffer: &ArgBuffer,
	) -> Address;

	fn get_tx_hash(&self) -> H256;

	fn get_gas_left(&self) -> u64;

	fn get_block_timestamp(&self) -> u64;

	fn get_block_nonce(&self) -> u64;

	fn get_block_round(&self) -> u64;

	fn get_block_epoch(&self) -> u64;

	fn get_block_random_seed(&self) -> Box<[u8; 48]>;

	fn get_prev_block_timestamp(&self) -> u64;

	fn get_prev_block_nonce(&self) -> u64;

	fn get_prev_block_round(&self) -> u64;

	fn get_prev_block_epoch(&self) -> u64;

	fn get_prev_block_random_seed(&self) -> Box<[u8; 48]>;

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
