use crate::{AbiType, AbiTypeFrom, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName};

/// Pure ABI counterpart of `FungiblePayment<M>`.
///
/// Provides a stable, framework-agnostic type representation for a fungible token payment
/// (token identifier + non-zero amount, no nonce).
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct FungiblePaymentAbi;

impl AbiTypeFrom<Self> for FungiblePaymentAbi {}

impl AbiType for FungiblePaymentAbi {
    fn type_name() -> TypeName {
        TypeName::from("FungiblePayment")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbiFrom<Self> for FungiblePaymentAbi {}

impl TypeAbi for FungiblePaymentAbi {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        TypeName::from("FungiblePaymentAbi")
    }
}
