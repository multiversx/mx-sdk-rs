use super::{
    BlockchainApi, CallTypeApi, CallValueApi, CryptoApi, EndpointArgumentApi, EndpointFinishApi,
    ErrorApi, LogApi, ManagedTypeApi, PrintApi, SendApi, StorageMapperApi, StorageReadApi,
    StorageReadApiImpl, StorageWriteApi,
};

pub trait VMApi:
    ManagedTypeApi
    + BlockchainApi
    + CallValueApi
    + CryptoApi
    + EndpointArgumentApi
    + EndpointFinishApi
    + ErrorApi
    + LogApi
    + SendApi
    + StorageReadApi
    + StorageWriteApi
    + PrintApi
    + CallTypeApi
    + StorageMapperApi
    + Clone // TODO: remove
    + PartialEq // for helping derive PartialEq for managed types
    + Eq
    + Send
    + Sync
{
    /// Slightly hacky way of overriding the constructor for external view contracts.
    /// 
    /// Only required for the tests, in production the meta crate makes sure to replace it.
    /// 
    /// TODO: find a more robust and maybe extendable solution.
    fn external_view_init_override() -> bool {
        false
    }
    fn init_static() {
        Self::storage_read_api_impl().storage_read_api_init();
    }
}
