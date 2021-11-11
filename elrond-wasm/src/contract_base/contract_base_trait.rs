use super::{
    BlockchainWrapper, CallValueWrapper, CryptoWrapper, ErrorHelper, ManagedSerializer,
    ManagedTypeHelper, PrintHelper, SendWrapper,
};
use crate::api::VMApi;

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

    /// Gateway into the call value retrieval functionality.
    /// The payment annotations should normally be the ones to handle this,
    /// but the developer is also given direct access to the API.
    fn call_value(&self) -> CallValueWrapper<Self::Api> {
        CallValueWrapper::new(self.raw_vm_api())
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
    #[inline]
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
    #[inline]
    fn serializer(&self) -> ManagedSerializer<Self::Api> {
        ManagedSerializer::new(self.raw_vm_api())
    }

    #[inline]
    fn error(&self) -> ErrorHelper<Self::Api> {
        ErrorHelper::new_instance(self.raw_vm_api())
    }

    #[inline]
    fn print(&self) -> PrintHelper<Self::Api> {
        PrintHelper::new(self.raw_vm_api())
    }
}
