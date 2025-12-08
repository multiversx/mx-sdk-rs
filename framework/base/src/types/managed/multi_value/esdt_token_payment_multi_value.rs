use crate::{
    abi::TypeAbiFrom,
    codec::{
        multi_types::MultiValue3, DecodeErrorHandler, EncodeErrorHandler, MultiValueConstLength,
        TopDecodeMulti, TopDecodeMultiInput, TopEncodeMulti, TopEncodeMultiOutput,
    },
    types::Ref,
};

use crate::{
    abi::{TypeAbi, TypeName},
    api::ManagedTypeApi,
    types::{BigUint, EsdtTokenIdentifier, EsdtTokenPayment, ManagedVecItem},
};

/// Thin wrapper around EsdtTokenPayment, which has different I/O behaviour:
/// - as input, is built from 3 arguments instead of 1: token identifier, nonce, value
/// - as output, it becomes 3 results instead of 1: token identifier, nonce, value
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EsdtTokenPaymentMultiValue<M: ManagedTypeApi> {
    obj: EsdtTokenPayment<M>,
}

#[deprecated(
    since = "0.29.3",
    note = "Alias kept for backwards compatibility. Replace with `EsdtTokenPaymentMultiValue`"
)]
pub type EsdtTokenPaymentMultiArg<M> = EsdtTokenPaymentMultiValue<M>;

impl<M: ManagedTypeApi> From<EsdtTokenPayment<M>> for EsdtTokenPaymentMultiValue<M> {
    #[inline]
    fn from(obj: EsdtTokenPayment<M>) -> Self {
        EsdtTokenPaymentMultiValue { obj }
    }
}

impl<M: ManagedTypeApi> EsdtTokenPaymentMultiValue<M> {
    pub fn into_inner(self) -> EsdtTokenPayment<M> {
        self.obj
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for EsdtTokenPaymentMultiValue<M> {
    type PAYLOAD = <EsdtTokenPayment<M> as ManagedVecItem>::PAYLOAD;
    const SKIPS_RESERIALIZATION: bool = EsdtTokenPayment::<M>::SKIPS_RESERIALIZATION;
    type Ref<'a> = Ref<'a, Self>;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        EsdtTokenPayment::read_from_payload(payload).into()
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        Ref::new(Self::read_from_payload(payload))
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        self.obj.save_to_payload(payload);
    }
}

impl<M> TopEncodeMulti for EsdtTokenPaymentMultiValue<M>
where
    M: ManagedTypeApi,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        output.push_single_value(&self.obj.token_identifier, h)?;
        output.push_single_value(&self.obj.token_nonce, h)?;
        output.push_single_value(&self.obj.amount, h)?;
        Ok(())
    }
}

impl<M> TopDecodeMulti for EsdtTokenPaymentMultiValue<M>
where
    M: ManagedTypeApi,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let token_identifier = EsdtTokenIdentifier::multi_decode_or_handle_err(input, h)?;
        let token_nonce = u64::multi_decode_or_handle_err(input, h)?;
        let amount = BigUint::multi_decode_or_handle_err(input, h)?;
        Ok(EsdtTokenPayment::new(token_identifier, token_nonce, amount).into())
    }
}

impl<M> MultiValueConstLength for EsdtTokenPaymentMultiValue<M>
where
    M: ManagedTypeApi,
{
    const MULTI_VALUE_CONST_LEN: usize = 3;
}

impl<M> TypeAbiFrom<Self> for EsdtTokenPaymentMultiValue<M> where M: ManagedTypeApi {}

impl<M> TypeAbi for EsdtTokenPaymentMultiValue<M>
where
    M: ManagedTypeApi,
{
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        MultiValue3::<EsdtTokenIdentifier<M>, u64, BigUint<M>>::type_name()
    }

    fn type_name_rust() -> TypeName {
        "EsdtTokenPaymentMultiValue<$API>".into()
    }

    fn is_variadic() -> bool {
        true
    }
}
