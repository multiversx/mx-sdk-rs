use crate::{TypeAbi, TypeAbiFrom, TypeName};

/// Pure ABI counterpart of `FungiblePayment<M>`.
///
/// Provides a stable, framework-agnostic type representation for a fungible token payment
/// (token identifier + non-zero amount, no nonce).
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct FungiblePaymentAbi;

impl TypeAbiFrom<Self> for FungiblePaymentAbi {}

impl TypeAbi for FungiblePaymentAbi {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        TypeName::from("FungiblePayment")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("FungiblePaymentAbi")
    }
}
