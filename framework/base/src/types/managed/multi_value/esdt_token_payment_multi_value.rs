use crate::codec::{
    multi_types::MultiValue3, DecodeErrorHandler, EncodeErrorHandler, TopDecodeMulti,
    TopDecodeMultiInput, TopDecodeMultiLength, TopEncodeMulti, TopEncodeMultiOutput,
};

use crate::{
    abi::{TypeAbi, TypeName},
    api::ManagedTypeApi,
    types::{BigUint, EsdtTokenPayment, ManagedVecItem, TokenIdentifier},
};

/// Thin wrapper around EsdtTokenPayment, which has different I/O behaviour:
/// - as input, is built from 3 arguments instead of 1: token identifier, nonce, value
/// - as output, it becomes 3 results instead of 1: token identifier, nonce, value
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EsdtTokenPaymentMultiValue<'a, M: ManagedTypeApi<'a>> {
    obj: EsdtTokenPayment<'a, M>,
}

#[deprecated(
    since = "0.29.3",
    note = "Alias kept for backwards compatibility. Replace with `EsdtTokenPaymentMultiValue`"
)]
pub type EsdtTokenPaymentMultiArg<'a, M> = EsdtTokenPaymentMultiValue<'a, M>;

impl<'a, M: ManagedTypeApi<'a>> From<EsdtTokenPayment<'a, M>> for EsdtTokenPaymentMultiValue<'a, M> {
    #[inline]
    fn from(obj: EsdtTokenPayment<'a, M>) -> Self {
        EsdtTokenPaymentMultiValue { obj }
    }
}

impl<'a, M: ManagedTypeApi<'a>> EsdtTokenPaymentMultiValue<'a, M> {
    pub fn into_esdt_token_payment(self) -> EsdtTokenPayment<'a, M> {
        self.obj
    }
}

impl<'a, M: ManagedTypeApi<'a>> ManagedVecItem for EsdtTokenPaymentMultiValue<'a, M> {
    const PAYLOAD_SIZE: usize = EsdtTokenPayment::<'a, M>::PAYLOAD_SIZE;
    const SKIPS_RESERIALIZATION: bool = EsdtTokenPayment::<'a, M>::SKIPS_RESERIALIZATION;
    type Ref<'b> = Self;

    #[inline]
    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        EsdtTokenPayment::from_byte_reader(reader).into()
    }

    #[inline]
    unsafe fn from_byte_reader_as_borrow<'b, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'b> {
        Self::from_byte_reader(reader)
    }

    #[inline]
    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        self.obj.to_byte_writer(writer)
    }

    fn take_handle_ownership(self) {
        self.obj.take_handle_ownership();
    }
}

impl<'a, M> TopEncodeMulti for EsdtTokenPaymentMultiValue<'a, M>
where
    M: ManagedTypeApi<'a>,
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

impl<'a, M> TopDecodeMulti for EsdtTokenPaymentMultiValue<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let token_identifier = TokenIdentifier::multi_decode_or_handle_err(input, h)?;
        let token_nonce = u64::multi_decode_or_handle_err(input, h)?;
        let amount = BigUint::multi_decode_or_handle_err(input, h)?;
        Ok(EsdtTokenPayment::new(token_identifier, token_nonce, amount).into())
    }
}

impl<'a, M> TopDecodeMultiLength for EsdtTokenPaymentMultiValue<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    const LEN: usize = 3;
}

impl<'a, M> TypeAbi for EsdtTokenPaymentMultiValue<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    fn type_name() -> TypeName {
        MultiValue3::<TokenIdentifier<'a, M>, u64, BigUint<'a, M>>::type_name()
    }

    fn is_variadic() -> bool {
        true
    }
}
