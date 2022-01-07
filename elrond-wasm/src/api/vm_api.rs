use crate::abi::EndpointLocationAbi;

use super::{
    BlockchainApi, CallTypeApi, CallValueApi, CryptoApi, EndpointArgumentApi, EndpointFinishApi,
    ErrorApi, LogApi, ManagedTypeApi, PrintApi, SendApi, StorageMapperApi, StorageReadApi,
    StorageWriteApi,
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
{
    fn has_location(location: EndpointLocationAbi) -> bool;
}
