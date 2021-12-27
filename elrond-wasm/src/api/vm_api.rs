use super::{
    BlockchainApi, CallTypeApi, CallValueApi, CryptoApi, EndpointArgumentApi, EndpointFinishApi,
    ErrorApi, LogApi, PrintApi, SendApi, StorageMapperApi, StorageReadApi, StorageWriteApi,
};

pub trait VMApi:
    BlockchainApi
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
}
