use crate::{
    abi::TypeAbiFrom,
    codec::{
        multi_types::MultiValue3, DecodeErrorHandler, EncodeErrorHandler, MultiValueConstLength,
        TopDecodeMulti, TopDecodeMultiInput, TopEncodeMulti, TopEncodeMultiOutput,
    },
    types::{ManagedVecRef, NonZeroBigUint, TokenId},
};

use crate::{
    abi::{TypeAbi, TypeName},
    api::ManagedTypeApi,
    types::{BigUint, EsdtTokenIdentifier, ManagedVecItem, Payment},
};

/// Thin wrapper around Payment, which has different I/O behaviour:
/// - as input, is built from 3 arguments instead of 1: token identifier, nonce, value
/// - as output, it becomes 3 results instead of 1: token identifier, nonce, value
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PaymentMultiValue<M: ManagedTypeApi> {
    obj: Payment<M>,
}

impl<M: ManagedTypeApi> From<Payment<M>> for PaymentMultiValue<M> {
    #[inline]
    fn from(obj: Payment<M>) -> Self {
        PaymentMultiValue { obj }
    }
}

impl<M: ManagedTypeApi> PaymentMultiValue<M> {
    pub fn into_inner(self) -> Payment<M> {
        self.obj
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for PaymentMultiValue<M> {
    type PAYLOAD = <Payment<M> as ManagedVecItem>::PAYLOAD;
    const SKIPS_RESERIALIZATION: bool = Payment::<M>::SKIPS_RESERIALIZATION;
    type Ref<'a> = ManagedVecRef<'a, Self>;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        Payment::read_from_payload(payload).into()
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        ManagedVecRef::new(Self::read_from_payload(payload))
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        self.obj.save_to_payload(payload);
    }
}

impl<M> TopEncodeMulti for PaymentMultiValue<M>
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

impl<M> TopDecodeMulti for PaymentMultiValue<M>
where
    M: ManagedTypeApi,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let token_identifier = TokenId::multi_decode_or_handle_err(input, h)?;
        let token_nonce = u64::multi_decode_or_handle_err(input, h)?;
        let amount = NonZeroBigUint::multi_decode_or_handle_err(input, h)?;
        Ok(Payment::new(token_identifier, token_nonce, amount).into())
    }
}

impl<M> MultiValueConstLength for PaymentMultiValue<M>
where
    M: ManagedTypeApi,
{
    const MULTI_VALUE_CONST_LEN: usize = 3;
}

impl<M> TypeAbiFrom<Self> for PaymentMultiValue<M> where M: ManagedTypeApi {}

impl<M> TypeAbi for PaymentMultiValue<M>
where
    M: ManagedTypeApi,
{
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        MultiValue3::<EsdtTokenIdentifier<M>, u64, BigUint<M>>::type_name()
    }

    fn type_name_rust() -> TypeName {
        "PaymentMultiValue<$API>".into()
    }

    fn is_variadic() -> bool {
        true
    }
}
