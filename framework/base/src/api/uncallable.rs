use crate::codec::TryStaticCast;

mod big_float_api_uncallable;
mod big_int_api_uncallable;
mod blockchain_api_uncallable;
mod call_value_api_uncallable;
mod crypto_api_uncallable;
mod elliptic_curve_api_uncallable;
mod endpoint_arg_api_uncallable;
mod endpoint_finish_api_uncallable;
mod error_api_uncallable;
mod log_api_uncallable;
mod managed_buffer_api_uncallable;
mod managed_map_api_uncallable;
mod managed_type_api_uncallable;
mod print_api_uncallable;
mod send_api_uncallable;
mod static_var_api_uncallable;
mod storage_api_uncallable;
mod vm_api_uncallable;

/// Dummy type with no implementation.
/// Provides context in ABI generators.
#[derive(Clone)]
pub struct UncallableApi;

impl TryStaticCast for UncallableApi {}
