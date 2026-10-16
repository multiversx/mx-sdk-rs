use super::ManagedTypeApi;

/// Marker for [`ManagedTypeApi`] implementations that require no live VM connection.
///
/// Managed types instantiated with such an API can be treated as being effectively their own
/// "unmanaged" representation (see [`HasUnmanaged`](crate::types::HasUnmanaged)), since there is
/// no VM to disconnect from. Only implemented by `StaticApi`, in `multiversx-sc-scenario`.
pub trait UnmanagedApi: ManagedTypeApi {}
