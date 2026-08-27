use super::*;

/// Indicates that 2 ABI types have the same encoding, so they can be used interchangeably in proxies.
///
/// Self can accept/decode Source in ABI terms.
/// Only relevant for serializable types.
pub trait AbiTypeFrom<Source: AbiType>: AbiType {}

/// Indicates that 2 concrete types have the same encoding, so they can be used interchangeably in proxies.
///
/// Only relevant for serializable types.
/// Kept for backward compatibility. New cross-type edges migrate to `AbiTypeFrom`.
pub trait TypeAbiFrom<T: ?Sized> {}
