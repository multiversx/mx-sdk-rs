use crate::{
    abi::{
        BigFloatAbi, BigIntAbi, BigUintAbi, BytesAbi, CountedVariadicAbi, DecimalAbi,
        DecimalSignedAbi, EgldOrEsdtTokenIdentifierAbi, EllipticCurveAbi, EsdtTokenIdentifierAbi,
        FungiblePaymentAbi, ListAbi, NonZeroBigUintAbi, PaymentAbi, StringAbi, TokenIdAbi,
        VariadicAbi,
    },
    api::ManagedTypeApi,
    codec::MultiValueConstLength,
    typenum::Unsigned,
    types::{
        BigFloat, BigInt, BigUint, ConstDecimals, EgldOrEsdtTokenIdentifier, EllipticCurve,
        EsdtTokenIdentifier, FungiblePayment, ManagedBuffer, ManagedDecimal, ManagedDecimalSigned,
        ManagedVec, ManagedVecItem, MultiValueEncoded, MultiValueEncodedCounted, NonZeroBigUint,
        NumDecimals, Payment, TokenId,
    },
};

use crate::types::HasManaged;

// Plain marker types with a real managed counterpart.

impl<M: ManagedTypeApi> HasManaged<M> for BigFloatAbi {
    type Managed = BigFloat<M>;
}

impl<M: ManagedTypeApi> HasManaged<M> for BigIntAbi {
    type Managed = BigInt<M>;
}

impl<M: ManagedTypeApi> HasManaged<M> for BigUintAbi {
    type Managed = BigUint<M>;
}

impl<M: ManagedTypeApi> HasManaged<M> for BytesAbi {
    type Managed = ManagedBuffer<M>;
}

impl<M: ManagedTypeApi> HasManaged<M> for StringAbi {
    type Managed = ManagedBuffer<M>;
}

impl<M: ManagedTypeApi> HasManaged<M> for EgldOrEsdtTokenIdentifierAbi {
    type Managed = EgldOrEsdtTokenIdentifier<M>;
}

impl<M: ManagedTypeApi> HasManaged<M> for EllipticCurveAbi {
    type Managed = EllipticCurve<M>;
}

impl<M: ManagedTypeApi> HasManaged<M> for EsdtTokenIdentifierAbi {
    type Managed = EsdtTokenIdentifier<M>;
}

impl<M: ManagedTypeApi> HasManaged<M> for FungiblePaymentAbi {
    type Managed = FungiblePayment<M>;
}

impl<M: ManagedTypeApi> HasManaged<M> for NonZeroBigUintAbi {
    type Managed = NonZeroBigUint<M>;
}

impl<M: ManagedTypeApi> HasManaged<M> for PaymentAbi {
    type Managed = Payment<M>;
}

impl<M: ManagedTypeApi> HasManaged<M> for TokenIdAbi {
    type Managed = TokenId<M>;
}

impl<M: ManagedTypeApi> HasManaged<M> for DecimalAbi<NumDecimals> {
    type Managed = ManagedDecimal<M, NumDecimals>;
}

impl<M: ManagedTypeApi, DECIMALS: Unsigned> HasManaged<M> for DecimalAbi<ConstDecimals<DECIMALS>> {
    type Managed = ManagedDecimal<M, ConstDecimals<DECIMALS>>;
}

impl<M: ManagedTypeApi> HasManaged<M> for DecimalSignedAbi<NumDecimals> {
    type Managed = ManagedDecimalSigned<M, NumDecimals>;
}

impl<M: ManagedTypeApi, DECIMALS: Unsigned> HasManaged<M>
    for DecimalSignedAbi<ConstDecimals<DECIMALS>>
{
    type Managed = ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>;
}

// Container ABI markers: resolve in terms of their item's managed counterpart.
//
// `ListAbi<T>` can be given a fully generic impl here (unlike `HasUnmanaged`, which
// deliberately skips it): now that `BytesAbi` is its own type rather than an alias for
// `ListAbi<u8>`, there's no coherence conflict between this blanket and a `BytesAbi`-specific
// one.

impl<M: ManagedTypeApi, T> HasManaged<M> for ListAbi<T>
where
    T: HasManaged<M>,
    T::Managed: ManagedVecItem,
{
    type Managed = ManagedVec<M, T::Managed>;
}

impl<M: ManagedTypeApi, T: HasManaged<M>> HasManaged<M> for VariadicAbi<T> {
    type Managed = MultiValueEncoded<M, T::Managed>;
}

impl<M: ManagedTypeApi, T> HasManaged<M> for CountedVariadicAbi<T>
where
    T: HasManaged<M> + MultiValueConstLength,
    T::Managed: MultiValueConstLength,
{
    type Managed = MultiValueEncodedCounted<M, T::Managed>;
}
