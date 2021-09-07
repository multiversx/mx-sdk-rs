mod big_int_api_uncallable;
mod blockchain_api_uncallable;
mod elliptic_curve_api_uncallable;
mod error_api_uncallable;
mod managed_buffer_api_uncallable;
mod managed_type_api_uncallable;
mod send_api_uncallable;
mod storage_api_uncallable;

/// Dummy type with no implementation.
/// Provides context in ABI generators.
#[derive(Clone)]
pub struct UncallableApi;
