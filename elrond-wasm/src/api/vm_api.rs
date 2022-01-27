use crate::abi::EndpointLocationAbi;

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
{
    fn has_location(location: EndpointLocationAbi) -> bool {
        location == EndpointLocationAbi::MainContract
    }

    fn init_static() {
        Self::storage_read_api_impl().storage_read_api_init()
    }
}
