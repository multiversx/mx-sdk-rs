use super::BigUintApi;
use crate::types::{Address, EsdtTokenData, H256};
use alloc::boxed::Box;

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
pub trait BlockchainApi<BigUint>: Sized
where
	BigUint: BigUintApi + 'static,
{
	fn get_sc_address(&self) -> Address;

	fn get_owner_address(&self) -> Address;

	fn get_shard_of_address(&self, address: &Address) -> u32;

	fn is_smart_contract(&self, address: &Address) -> bool;

	fn get_caller(&self) -> Address;

	fn get_balance(&self, address: &Address) -> BigUint;

	fn get_sc_balance(&self) -> BigUint {
		self.get_balance(&self.get_sc_address())
	}

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

	fn get_current_esdt_nft_nonce(&self, address: &Address, token: &[u8]) -> u64;

	fn get_esdt_balance(&self, address: &Address, token: &[u8], nonce: u64) -> BigUint;

	fn get_esdt_token_data(
		&self,
		address: &Address,
		token: &[u8],
		nonce: u64,
	) -> EsdtTokenData<BigUint>;
}
