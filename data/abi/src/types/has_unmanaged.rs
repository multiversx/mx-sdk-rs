use super::*;

/// A pure ABI type that has a known default concrete Rust implementation.
///
/// Not implemented when the concrete type is feature-gated or unavailable.
/// This trait is optional and only implemented by pure ABI types that have
/// an obvious default "unmanaged" Rust counterpart.
pub trait HasUnmanaged: AbiType {
    /// The default concrete Rust type for this ABI type.
    type Unmanaged: TypeAbi;
}
