use alloc::string::String;
use elrond_codec::{
    DecodeErrorHandler, EncodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput, TopEncodeMulti,
    TopEncodeMultiOutput,
};

use crate::{
    abi::TypeAbi,
    api::ManagedTypeApi,
    types::{BigUint, EsdtTokenPayment, ManagedVecItem, TokenIdentifier},
};

/// Thin wrapper around EsdtTokenPayment, which has different I/O behaviour:
/// - as input, is built from 3 arguments instead of 1: token identifier, nonce, value
/// - as output, it becomes 3 results instead of 1: token identifier, nonce, value
#[derive(Clone, PartialEq, Debug)]
pub struct EsdtTokenPaymentMultiArg<M: ManagedTypeApi> {
    obj: EsdtTokenPayment<M>,
}

impl<M: ManagedTypeApi> From<EsdtTokenPayment<M>> for EsdtTokenPaymentMultiArg<M> {
    #[inline]
    fn from(obj: EsdtTokenPayment<M>) -> Self {
        EsdtTokenPaymentMultiArg { obj }
    }
}

impl<M: ManagedTypeApi> EsdtTokenPaymentMultiArg<M> {
    pub fn into_esdt_token_payment(self) -> EsdtTokenPayment<M> {
        self.obj
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for EsdtTokenPaymentMultiArg<M> {
    const PAYLOAD_SIZE: usize = EsdtTokenPayment::<M>::PAYLOAD_SIZE;
    const SKIPS_RESERIALIZATION: bool = EsdtTokenPayment::<M>::SKIPS_RESERIALIZATION;
    type Ref<'a> = Self;

    #[inline]
    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        EsdtTokenPayment::from_byte_reader(reader).into()
    }

    #[inline]
    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }

    #[inline]
    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        self.obj.to_byte_writer(writer)
    }
}

impl<M> TopEncodeMulti for EsdtTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    type DecodeAs = Self;

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

impl<M> TopDecodeMulti for EsdtTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
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

impl<M> TypeAbi for EsdtTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    fn type_name() -> String {
        crate::types::MultiArg3::<TokenIdentifier<M>, u64, BigUint<M>>::type_name()
    }

    fn is_variadic() -> bool {
        true
    }
}
