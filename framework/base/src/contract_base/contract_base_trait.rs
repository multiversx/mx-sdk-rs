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
pub trait ContractBase<A: VMApi>: Sized {
    /// Gateway into the call value retrieval functionality.
    /// The payment annotations should normally be the ones to handle this,
    /// but the developer is also given direct access to the API.
    fn call_value(&self) -> CallValueWrapper<A> {
        CallValueWrapper::new()
    }

    /// Gateway to the functionality related to sending transactions from the current contract.
    #[inline]
    fn send(&self) -> SendWrapper<A> {
        SendWrapper::new()
    }

    /// Low-level functionality related to sending transactions from the current contract.
    ///
    /// For almost all cases contracts should instead use `self.send()` and `ContractCall`.
    #[inline]
    fn send_raw(&self) -> SendRawWrapper<A> {
        SendRawWrapper::new()
    }

    /// Gateway blockchain info related to the current transaction and to accounts.
    #[inline]
    fn blockchain(&self) -> BlockchainWrapper<A> {
        BlockchainWrapper::<A>::new()
    }

    /// Stateless crypto functions provided by the Arwen VM.
    #[inline]
    fn crypto(&self) -> CryptoWrapper<A> {
        CryptoWrapper::new()
    }

    /// Component that provides contract developers access
    /// to highly optimized manual serialization and deserialization.
    #[inline]
    fn serializer(&self) -> ManagedSerializer<A> {
        ManagedSerializer::new()
    }

    #[inline]
    fn error(&self) -> ErrorHelper<A> {
        ErrorHelper::new()
    }

    #[inline]
    fn storage_raw(&self) -> StorageRawWrapper<A> {
        StorageRawWrapper::new()
    }
}
