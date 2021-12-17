use super::{
    BlockchainApi, CallTypeApi, CallValueApi, CryptoApi, EndpointArgumentApi, EndpointFinishApi,
    ErrorApi, LogApi, ManagedTypeApi, ManagedTypeErrorApi, PrintApi, SendApi, StorageReadApi,
    StorageWriteApi,
};

// TODO: cleanup
pub trait VMApi:
    BlockchainApi
    + CallValueApi
    + CryptoApi
    + EndpointArgumentApi
    + EndpointFinishApi
    // + ErrorApi
    + LogApi
    + ManagedTypeErrorApi
    + CallTypeApi
    + SendApi
    + StorageReadApi
    + StorageWriteApi
    + PrintApi
    + Clone
{
}
