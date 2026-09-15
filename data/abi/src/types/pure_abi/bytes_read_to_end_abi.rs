use crate::{TypeAbi, TypeAbiFrom, TypeName};

/// Pure ABI counterpart of `ManagedBufferReadToEnd<M>`.
///
/// `ManagedBufferReadToEnd` streams the remaining bytes directly off the live top-level input,
/// a behavior tied to an active `ManagedTypeApi` - but on the wire it is still just the
/// `bytes-read-to-end` ABI type, so it has a stable, framework-agnostic marker like any other
/// pure ABI type, even though there is no meaningful "unmanaged" value for it to carry.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct BytesReadToEndAbi;

impl TypeAbiFrom<Self> for BytesReadToEndAbi {}

impl TypeAbi for BytesReadToEndAbi {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        TypeName::from("bytes-read-to-end")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("BytesReadToEndAbi")
    }
}
