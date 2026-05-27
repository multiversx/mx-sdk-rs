/// Indicates that 2 types have the same encoding, so they can be used interchangeably in proxies.
///
/// Only relevant for serializable types.
pub trait TypeAbiFrom<T: ?Sized> {}
