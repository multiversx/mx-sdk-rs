use super::{
	BigIntApi, BigUintApi, BlockchainApi, CallValueApi, CryptoApi, EndpointArgumentApi,
	EndpointFinishApi, ErrorApi, LogApi, SendApi, StorageReadApi, StorageWriteApi,
};
use crate::{
	storage,
	types::{Address, TokenIdentifier},
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
	type CallValue: CallValueApi + ErrorApi + Clone + 'static;

	/// Abstracts the sending of EGLD & ESDT transactions, as well as async calls.
	type SendApi: SendApi + Clone + 'static;

	type BlockchainApi: BlockchainApi + Clone + 'static;

	type CryptoApi: CryptoApi + Clone + 'static;

	type LogApi: LogApi + ErrorApi + Clone + 'static;

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

	/// Retrieves validator rewards, as set by the protocol.
	/// TODO: move to the storage API, once BigUint gets refactored
	#[inline]
	fn storage_load_cumulated_validator_reward(&self) -> Self::BigUint {
		storage::storage_get(
			self.get_storage_raw(),
			storage::protected_keys::ELROND_REWARD_KEY,
		)
	}
}

pub trait ContractPrivateApi {
	type ArgumentApi: EndpointArgumentApi + Clone + 'static;

	type FinishApi: EndpointFinishApi + ErrorApi + Clone + 'static;

	fn argument_api(&self) -> Self::ArgumentApi;

	fn finish_api(&self) -> Self::FinishApi;
}

pub trait ProxyObjApi {
	type BigUint: BigUintApi + 'static;

	type BigInt: BigIntApi + 'static;

	type PaymentType: BigUintApi + 'static;

	type ProxySendApi: SendApi + Clone + 'static;

	// type ContractCall<R>;

	// fn new_proxy_obj(api: Self::ProxySendApi, address: Address) -> Self;

	fn with_token_transfer(self, token: TokenIdentifier, payment: Self::PaymentType) -> Self;

	fn into_fields(
		self,
	) -> (
		Self::ProxySendApi,
		Address,
		TokenIdentifier,
		Self::PaymentType,
	);

	// fn get_address(&self) -> Address;

	// fn get_token(&self) -> TokenIdentifier;

	// fn get_payment(&self) -> Self::BigUint;
}
