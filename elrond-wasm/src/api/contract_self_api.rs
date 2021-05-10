use super::{
	BigIntApi, BigUintApi, BlockchainApi, CallValueApi, CryptoApi, ErrorApi, SendApi,
	StorageReadApi, StorageWriteApi,
};
use crate::storage;

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
pub trait ContractSelfApi<BigInt, BigUint>: Sized
where
	BigInt: BigIntApi<BigUint> + 'static,
	BigUint: BigUintApi + 'static,
{
	/// Abstracts the lower-level storage functionality.
	type Storage: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static;

	/// Abstracts the call value handling at the beginning of a function call.
	type CallValue: CallValueApi<BigUint> + ErrorApi + Clone + 'static;

	/// Abstracts the sending of EGLD & ESDT transactions, as well as async calls.
	type SendApi: SendApi<BigUint> + Clone + 'static;

	type BlockchainApi: BlockchainApi<BigUint> + Clone + 'static;

	type CryptoApi: CryptoApi<BigInt, BigUint> + Clone + 'static;

	/// Gateway into the lower-level storage functionality.
	/// Storage related annotations make use of this.
	/// Using it directly is not recommended.
	fn get_storage_raw(&self) -> Self::Storage;

	/// Gateway into the call value retrieval functionality.
	/// The payment annotations should normally be the ones to handle this,
	/// but the developer is also given direct access to the API.
	fn call_value(&self) -> Self::CallValue;

	/// Gateway to the functionality related to sending transactions from the current contract.
	fn send(&self) -> Self::SendApi;

	/// Gateway blockchain info related to the current transaction and to accounts.
	fn blockchain(&self) -> Self::BlockchainApi;

	/// Stateless crypto functions provided by the Arwen VM.
	fn crypto(&self) -> Self::CryptoApi;

	/// Retrieves validator rewards, as set by the protocol.
	/// TODO: move to the storage API, once BigUint gets refactored
	#[inline]
	fn storage_load_cumulated_validator_reward(&self) -> BigUint {
		storage::storage_get(
			self.get_storage_raw(),
			storage::protected_keys::ELROND_REWARD_KEY,
		)
	}
}
