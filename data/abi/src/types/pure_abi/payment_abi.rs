use crate::{
    AbiType, AbiTypeFrom, HasUnmanaged, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName,
};

/// Pure ABI counterpart of `Payment<M>`.
///
/// Provides a stable, framework-agnostic type representation for a token payment
/// (token identifier + nonce + non-zero amount).
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct PaymentAbi;

impl AbiTypeFrom<Self> for PaymentAbi {}

impl AbiType for PaymentAbi {
    fn type_name() -> TypeName {
        TypeName::from("Payment")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbiFrom<Self> for PaymentAbi {}

impl TypeAbi for PaymentAbi {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        TypeName::from("PaymentAbi")
    }
}

#[cfg(feature = "num-bigint")]
impl HasUnmanaged for PaymentAbi {
    type Unmanaged = (
        multiversx_chain_core::types::BoxedBytes,
        u64,
        crate::codec::num_bigint::BigUint,
    );
}
