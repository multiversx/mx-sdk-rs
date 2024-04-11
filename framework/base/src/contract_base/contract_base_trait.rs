use super::{
    BlockchainWrapper, CallValueWrapper, CryptoWrapper, ErrorHelper, ManagedSerializer,
    SendRawWrapper, SendWrapper, StorageRawWrapper,
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

    /// Gateway into the call value retrieval functionality.
    /// The payment annotations should normally be the ones to handle this,
    /// but the developer is also given direct access to the API.
    fn call_value(&self) -> CallValueWrapper<Self::Api> {
        CallValueWrapper::new()
    }

    /// Gateway to the functionality related to sending transactions from the current contract.
    #[inline]
    fn send(&self) -> SendWrapper<Self::Api> {
        SendWrapper::new()
    }

    /// Low-level functionality related to sending transactions from the current contract.
    ///
    /// For almost all cases contracts should instead use `self.send()` and `ContractCall`.
    #[inline]
    fn send_raw(&self) -> SendRawWrapper<Self::Api> {
        SendRawWrapper::new()
    }

    /// Gateway blockchain info related to the current transaction and to accounts.
    #[inline]
    fn blockchain(&self) -> BlockchainWrapper<Self::Api> {
        BlockchainWrapper::<Self::Api>::new()
    }

    /// Stateless crypto functions provided by the Arwen VM.
    #[inline]
    fn crypto(&self) -> CryptoWrapper<Self::Api> {
        CryptoWrapper::new()
    }

    /// Component that provides contract developers access
    /// to highly optimized manual serialization and deserialization.
    #[inline]
    fn serializer(&self) -> ManagedSerializer<Self::Api> {
        ManagedSerializer::new()
    }

    #[inline]
    fn error(&self) -> ErrorHelper<Self::Api> {
        ErrorHelper::new()
    }

    #[inline]
    fn storage_raw(&self) -> StorageRawWrapper<Self::Api> {
        StorageRawWrapper::new()
    }
}
