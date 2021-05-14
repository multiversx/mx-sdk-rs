use super::{
	BigIntApi, BigUintApi, BlockchainApi, CallValueApi, CryptoApi, EndpointArgumentApi,
	EndpointFinishApi, ErrorApi, LogApi, ProxyObjApi, SendApi, StorageReadApi, StorageWriteApi,
};
use crate::{
	storage,
	types::{Address, BoxedBytes, EsdtLocalRole, Vec},
};

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
pub trait ContractBase: Sized {
	type BigUint: BigUintApi + 'static;

	type BigInt: BigIntApi + 'static;

	/// Abstracts the lower-level storage functionality.
	type Storage: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static;

	/// Abstracts the call value handling at the beginning of a function call.
	type CallValue: CallValueApi<AmountType = Self::BigUint> + ErrorApi + Clone + 'static;

	/// Abstracts the sending of EGLD & ESDT transactions, as well as async calls.
	type SendApi: SendApi<
			AmountType = Self::BigUint,
			ProxyBigInt = Self::BigInt,
			ProxyStorage = Self::Storage,
		> + Clone
		+ 'static;

	type BlockchainApi: BlockchainApi<BalanceType = Self::BigUint> + Clone + 'static;

	type CryptoApi: CryptoApi + Clone + 'static;

	type LogApi: LogApi + ErrorApi + Clone + 'static;

	type ErrorApi: ErrorApi + Clone + 'static;

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

	/// Gateway into the lower-level event log functionality.
	/// Gets called in auto-generated
	/// Using it directly is not recommended.
	/// TODO: consider moving to `ContractPrivateApi`.
	fn log_api_raw(&self) -> Self::LogApi;

	/// Currently for some auto-generated code involving callbacks.
	/// Please avoid using it directly.
	/// TODO: find a way to hide this API.
	fn error_api(&self) -> Self::ErrorApi;

	/// Retrieves validator rewards, as set by the protocol.
	/// TODO: move to the storage API, once BigUint gets refactored
	#[inline]
	fn storage_load_cumulated_validator_reward(&self) -> Self::BigUint {
		storage::storage_get(
			self.get_storage_raw(),
			storage::protected_keys::ELROND_REWARD_KEY,
		)
	}

	/// Retrieves local roles for the token, by reading protected storage.
	#[inline]
	fn storage_load_esdt_local_roles(&self, token_id: &[u8]) -> Vec<EsdtLocalRole> {
		let mut roles = Vec::new();

		let key = [storage::protected_keys::ELROND_ESDT_LOCAL_ROLES_KEY, token_id].concat();
		let raw_storage = storage::storage_get::<Self::Storage, BoxedBytes>(
			self.get_storage_raw(),
			&key,
		);
		let raw_storage_bytes = raw_storage.as_slice();
		let mut current_index = 0;

		while current_index < raw_storage_bytes.len() {
			// first character before each role is a \n, so we skip it
			current_index += 1;

			// next is the length of the role as string
			let role_len = raw_storage_bytes[current_index];
			current_index += 1;

			// next is role's ASCII string representation
			let end_index = current_index + role_len as usize;
			let role_name = &raw_storage_bytes[current_index..end_index];
			current_index = end_index;

			let esdt_local_role = EsdtLocalRole::from(role_name);
			roles.push(esdt_local_role);
		}

		roles
	}

	fn proxy<P: ProxyObjApi<SendApi = Self::SendApi>>(&self, address: Address) -> P {
		P::new_proxy_obj(self.send(), address)
	}
}

pub trait ContractPrivateApi {
	type ArgumentApi: EndpointArgumentApi + Clone + 'static;

	type FinishApi: EndpointFinishApi + ErrorApi + Clone + 'static;

	fn argument_api(&self) -> Self::ArgumentApi;

	fn finish_api(&self) -> Self::FinishApi;
}
