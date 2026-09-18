use crate::{TypeAbi, TypeAbiFrom, TypeName};

/// ABI marker for a raw byte buffer (e.g. `ManagedBuffer`, `BoxedBytes`) — distinct from
/// `ListAbi<u8>`, which is what a genuine top-encoded list of individual bytes (`Vec<u8>`, ...)
/// resolves to.
pub struct BytesAbi;

impl TypeAbiFrom<Self> for BytesAbi {}
impl TypeAbiFrom<&[u8]> for BytesAbi {}
impl TypeAbiFrom<&str> for BytesAbi {}
impl<const N: usize> TypeAbiFrom<&[u8; N]> for BytesAbi {}

impl TypeAbi for BytesAbi {
    type Abi = Self;

    fn type_name() -> TypeName {
        "bytes".into()
    }

    fn type_name_rust() -> TypeName {
        "BytesAbi".into()
    }
}
