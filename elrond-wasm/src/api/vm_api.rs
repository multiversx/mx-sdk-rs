use super::{
    BlockchainApi, CallValueApi, CryptoApi, EndpointArgumentApi, EndpointFinishApi, ErrorApi,
    LogApi, ManagedTypeApi, PrintApi, SendApi, StorageReadApi, StorageWriteApi,
};

pub trait VMApi:
    BlockchainApi
    + CallValueApi
    + CryptoApi
    + EndpointArgumentApi
    + EndpointFinishApi
    + ErrorApi
    + LogApi
    + ManagedTypeApi
    + SendApi
    + StorageReadApi
    + StorageWriteApi
    + PrintApi
    + Clone
{
}
