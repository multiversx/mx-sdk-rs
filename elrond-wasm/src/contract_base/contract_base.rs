// use super::{
//     BlockchainApi, CallValueApi, CryptoApi, EndpointArgumentApi, EndpointFinishApi, ErrorApi,
//     LogApi, ManagedSerializer, ManagedTypeApi, ManagedTypeHelper, ProxyObjApi, SendApi,
//     StorageReadApi, StorageWriteApi,
// };
use super::{
    BlockchainWrapper, CryptoWrapper, ManagedSerializer, ManagedTypeHelper, ProxyObjApi,
    SendWrapper,
};
use crate::{
    api::VMApi,
    types::{Address, ManagedAddress},
};

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
pub trait ContractBase: Sized {
    type Api: VMApi;

    /// Grants direct access to the underlying VM API.
    /// Avoid using it directly.
    fn raw_vm_api(&self) -> Self::Api;

    /// Gateway into the lower-level storage functionality.
    /// Storage related annotations make use of this.
    /// Using it directly is not recommended.
    fn get_storage_raw(&self) -> Self::Api {
        self.raw_vm_api()
    }

    /// Gateway into the call value retrieval functionality.
    /// The payment annotations should normally be the ones to handle this,
    /// but the developer is also given direct access to the API.
    fn call_value(&self) -> Self::Api {
        self.raw_vm_api()
    }

    /// Gateway to the functionality related to sending transactions from the current contract.
    #[inline]
    fn send(&self) -> SendWrapper<Self::Api> {
        SendWrapper::new(self.raw_vm_api())
    }

    /// Managed types API. Required to create new instances of managed types.
    #[inline]
    fn type_manager(&self) -> Self::Api {
        self.raw_vm_api()
    }

    /// Helps create new instances of managed types
    fn types(&self) -> ManagedTypeHelper<Self::Api> {
        ManagedTypeHelper::new(self.raw_vm_api())
    }

    /// Gateway blockchain info related to the current transaction and to accounts.
    #[inline]
    fn blockchain(&self) -> BlockchainWrapper<Self::Api> {
        BlockchainWrapper::new(self.raw_vm_api())
    }

    /// Stateless crypto functions provided by the Arwen VM.
    #[inline]
    fn crypto(&self) -> CryptoWrapper<Self::Api> {
        CryptoWrapper::new(self.raw_vm_api())
    }

    /// Component that provides contract developers access
    /// to highly optimized manual serialization and deserialization.
    fn serializer(&self) -> ManagedSerializer<Self::Api> {
        ManagedSerializer::new(self.raw_vm_api())
    }

    /// Currently for some auto-generated code involving callbacks.
    /// Please avoid using it directly.
    /// TODO: find a way to hide this API.
    fn error_api(&self) -> Self::Api {
        self.raw_vm_api()
    }
}
