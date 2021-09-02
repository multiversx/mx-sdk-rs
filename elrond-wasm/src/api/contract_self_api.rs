use super::{
    BlockchainApi, CallValueApi, CryptoApi, EndpointArgumentApi, EndpointFinishApi, ErrorApi,
    LogApi, ManagedSerializer, ManagedTypeApi, ManagedTypeHelper, ProxyObjApi, SendApi,
    StorageReadApi, StorageWriteApi,
};
use crate::types::{Address, ManagedAddress};

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
pub trait ContractBase: Sized {
    type TypeManager: ManagedTypeApi + ErrorApi + 'static;

    /// Abstracts the lower-level storage functionality.
    type Storage: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static;

    /// Abstracts the call value handling at the beginning of a function call.
    type CallValue: CallValueApi<TypeManager = Self::TypeManager> + ErrorApi + Clone + 'static;

    /// Abstracts the sending of EGLD & ESDT transactions, as well as async calls.
    type SendApi: SendApi<ProxyTypeManager = Self::TypeManager, ProxyStorage = Self::Storage>
        + 'static;

    type BlockchainApi: BlockchainApi<Storage = Self::Storage, TypeManager = Self::TypeManager>
        + Clone
        + 'static;

    type CryptoApi: CryptoApi + Clone + 'static;

    type LogApi: LogApi + ErrorApi + ManagedTypeApi + Clone + 'static;

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

    /// Managed types API. Required to create new instances of managed types.
    fn type_manager(&self) -> Self::TypeManager;

    /// Helps create new instances of managed types
    fn types(&self) -> ManagedTypeHelper<Self::TypeManager> {
        ManagedTypeHelper::new(self.type_manager())
    }

    /// Gateway blockchain info related to the current transaction and to accounts.
    fn blockchain(&self) -> Self::BlockchainApi;

    /// Stateless crypto functions provided by the Arwen VM.
    fn crypto(&self) -> Self::CryptoApi;

    /// Component that provides contract developers access
    /// to highly optimized manual serialization and deserialization.
    fn serializer(&self) -> ManagedSerializer<Self::TypeManager> {
        ManagedSerializer::new(self.type_manager())
    }

    /// Gateway into the lower-level event log functionality.
    /// Gets called in auto-generated
    /// Using it directly is not recommended.
    /// TODO: consider moving to `ContractPrivateApi`.
    fn log_api_raw(&self) -> Self::LogApi;

    /// Currently for some auto-generated code involving callbacks.
    /// Please avoid using it directly.
    /// TODO: find a way to hide this API.
    fn error_api(&self) -> Self::ErrorApi;
}

pub trait ContractPrivateApi {
    type ArgumentApi: ManagedTypeApi + EndpointArgumentApi + Clone + 'static;

    type CallbackClosureArgumentApi: ManagedTypeApi + ErrorApi + Clone + 'static;

    type FinishApi: ManagedTypeApi + EndpointFinishApi + ErrorApi + Clone + 'static;

    fn argument_api(&self) -> Self::ArgumentApi;

    fn callback_closure_arg_api(&self) -> Self::CallbackClosureArgumentApi;

    fn finish_api(&self) -> Self::FinishApi;
}
