use super::{ErrorApi, ManagedTypeApi, SendApi, StorageReadApi, StorageWriteApi};

/// Provided for convenience.
/// Designed to be used in any types that send tokens or calls.
pub trait CallTypeApi<'a>: SendApi<'a> + ManagedTypeApi<'a> + ErrorApi {}

/// Provided for convenience.
/// Designed to be used in storage mappers.
pub trait StorageMapperApi<'a>:
    StorageReadApi + StorageWriteApi + ManagedTypeApi<'a> + ErrorApi
{
}
