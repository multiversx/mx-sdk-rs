use multiversx_sc::{
    abi::{
        BigFloatAbi, BigIntAbi, BigUintAbi, BytesReadToEndAbi, CountedVariadicAbi, DecimalAbi,
        DecimalSignedAbi, EgldOrEsdtTokenIdentifierAbi, EllipticCurveAbi, EsdtTokenIdentifierAbi,
        FungiblePaymentAbi, ListAbi, NonZeroBigUintAbi, PaymentAbi, StringAbi, TokenIdAbi,
        VariadicAbi,
    },
    codec::{MultiValueConstLength, multi_types::MultiValueVec, num_bigint},
    typenum::Unsigned,
    types::{
        ConstDecimals, EgldOrEsdtTokenIdentifier, EllipticCurve, EsdtTokenIdentifier,
        FungiblePayment, ManagedDecimal, ManagedDecimalSigned, NonZeroBigUint, NumDecimals,
        Payment, TokenId,
    },
};

use crate::{api::StaticApi, facade::result_handlers::HasUnmanaged};

// Plain, non-managed unmanaged counterparts, when one is available.

impl HasUnmanaged for BigFloatAbi {
    type Unmanaged = f64;
}

impl HasUnmanaged for BigIntAbi {
    type Unmanaged = num_bigint::BigInt;
}

impl HasUnmanaged for BigUintAbi {
    type Unmanaged = num_bigint::BigUint;
}

impl HasUnmanaged for BytesReadToEndAbi {
    type Unmanaged = Vec<u8>;
}

impl HasUnmanaged for StringAbi {
    type Unmanaged = String;
}

// No plain unmanaged counterpart exists for these: fall back to the managed type,
// instantiated with `StaticApi`.

macro_rules! has_unmanaged_static {
    ($ty:ident, $managed_ty:ident) => {
        impl HasUnmanaged for $ty {
            type Unmanaged = $managed_ty<StaticApi>;
        }
    };
}

has_unmanaged_static!(EgldOrEsdtTokenIdentifierAbi, EgldOrEsdtTokenIdentifier);
has_unmanaged_static!(EllipticCurveAbi, EllipticCurve);
has_unmanaged_static!(EsdtTokenIdentifierAbi, EsdtTokenIdentifier);
has_unmanaged_static!(FungiblePaymentAbi, FungiblePayment);
has_unmanaged_static!(NonZeroBigUintAbi, NonZeroBigUint);
has_unmanaged_static!(PaymentAbi, Payment);
has_unmanaged_static!(TokenIdAbi, TokenId);

impl HasUnmanaged for DecimalAbi<NumDecimals> {
    type Unmanaged = ManagedDecimal<StaticApi, NumDecimals>;
}

impl<DECIMALS: Unsigned> HasUnmanaged for DecimalAbi<ConstDecimals<DECIMALS>> {
    type Unmanaged = ManagedDecimal<StaticApi, ConstDecimals<DECIMALS>>;
}

impl HasUnmanaged for DecimalSignedAbi<NumDecimals> {
    type Unmanaged = ManagedDecimalSigned<StaticApi, NumDecimals>;
}

impl<DECIMALS: Unsigned> HasUnmanaged for DecimalSignedAbi<ConstDecimals<DECIMALS>> {
    type Unmanaged = ManagedDecimalSigned<StaticApi, ConstDecimals<DECIMALS>>;
}

// Container ABI markers: resolve in terms of their item's unmanaged counterpart.

impl<T: HasUnmanaged> HasUnmanaged for ListAbi<T> {
    type Unmanaged = Vec<T::Unmanaged>;
}

impl<T: HasUnmanaged> HasUnmanaged for VariadicAbi<T> {
    type Unmanaged = MultiValueVec<T::Unmanaged>;
}

impl<T: HasUnmanaged + MultiValueConstLength> HasUnmanaged for CountedVariadicAbi<T> {
    type Unmanaged = MultiValueVec<T::Unmanaged>;
}
