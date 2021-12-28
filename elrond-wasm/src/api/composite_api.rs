use super::{ErrorApi, ManagedTypeApi, SendApi, StorageReadApi, StorageWriteApi};

/// Provided for convenience.
/// Designed to be used in any types that send tokens or calls.
pub trait CallTypeApi: SendApi + ManagedTypeApi + ErrorApi {}

/// Provided for convenience.
/// Designed to be used in storage mappers.
pub trait StorageMapperApi:
    StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + 'static
{
}
