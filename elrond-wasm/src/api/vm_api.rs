use super::{
    BlockchainApi, CallTypeApi, CallValueApi, CryptoApi, EndpointArgumentApi, EndpointFinishApi,
    ErrorApi, LogApi, PrintApi, SendApi, StorageReadApi, StorageWriteApi,
};

pub trait VMApi:
    BlockchainApi
    + CallValueApi
    + CryptoApi
    + EndpointArgumentApi
    + EndpointFinishApi
    + ErrorApi
    + LogApi
    + CallTypeApi
    + SendApi
    + StorageReadApi
    + StorageWriteApi
    + PrintApi
    + Clone
{
}
