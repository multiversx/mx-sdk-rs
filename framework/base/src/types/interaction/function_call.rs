use multiversx_sc_codec::{
    DecodeErrorHandler, EncodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput, TopEncodeMulti,
    TopEncodeMultiOutput,
};

use crate::{
    abi::{TypeAbi, TypeName},
    api::{
        ManagedTypeApi, ESDT_MULTI_TRANSFER_FUNC_NAME, ESDT_NFT_TRANSFER_FUNC_NAME,
        ESDT_TRANSFER_FUNC_NAME,
    },
    formatter::SCLowerHex,
    types::{
        EsdtTokenPayment, ManagedAddress, ManagedBuffer, ManagedBufferBuilder, ManagedVec,
        MultiValueEncoded,
    },
};

use super::ManagedArgBuffer;

/// Encodes a function call on the blockchain, composed of a function name and its encoded arguments.
///
/// Can be used as a multi-argument, to embed a call within a call.
pub struct FunctionCall<'a, Api>
where
    Api: ManagedTypeApi<'a>,
{
    pub function_name: ManagedBuffer<'a, Api>,
    pub arg_buffer: ManagedArgBuffer<'a, Api>,
}

impl<'a, Api> FunctionCall<'a, Api>
where
    Api: ManagedTypeApi<'a>,
{
    /// Initializes a new function call with a function call name.
    ///
    /// The arguments will need to be added afterwards.
    pub fn new<N: Into<ManagedBuffer<'a, Api>>>(function_name: N) -> Self {
        FunctionCall {
            function_name: function_name.into(),
            arg_buffer: ManagedArgBuffer::new(),
        }
    }

    /// Initializes a new empty function call, this means no function name and no arguments.
    pub fn empty() -> Self {
        FunctionCall::new(ManagedBuffer::new())
    }

    /// Empty function calls have empty function names.
    ///
    /// There should be no function call with empty function call but with arguments.
    pub fn is_empty(&self) -> bool {
        self.function_name.is_empty()
    }

    /// Adds an argument of any serializable type.
    ///
    /// Multi-values are accepted. No type checking performed.
    pub fn argument<T: TopEncodeMulti>(mut self, arg: &T) -> Self {
        self.arg_buffer.push_multi_arg(arg);
        self
    }

    pub fn to_call_data_string(&self) -> ManagedBuffer<'a, Api> {
        let mut result = ManagedBufferBuilder::default();
        result.append_managed_buffer(&self.function_name);
        for arg in self.arg_buffer.raw_arg_iter() {
            result.append_bytes(b"@");
            SCLowerHex::fmt(&*arg, &mut result);
        }
        result.into_managed_buffer()
    }
}

impl<'a, Api> TopEncodeMulti for FunctionCall<'a, Api>
where
    Api: ManagedTypeApi<'a>,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        if self.function_name.is_empty() {
            return Ok(());
        }
        output.push_single_value(&self.function_name, h)?;
        for arg in self.arg_buffer.raw_arg_iter() {
            output.push_single_value(&*arg, h)?;
        }

        Ok(())
    }
}

impl<'a, Api> TopDecodeMulti for FunctionCall<'a, Api>
where
    Api: ManagedTypeApi<'a>,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        if !input.has_next() {
            return Ok(FunctionCall::empty());
        }

        let function_name = ManagedBuffer::<Api>::multi_decode_or_handle_err(input, h)?;
        let args =
            MultiValueEncoded::<Api, ManagedBuffer<'a, Api>>::multi_decode_or_handle_err(input, h)?;
        Ok(FunctionCall {
            function_name,
            arg_buffer: args.to_arg_buffer(),
        })
    }
}

impl<'a, Api> TypeAbi for FunctionCall<'a, Api>
where
    Api: ManagedTypeApi<'a>,
{
    fn type_name() -> TypeName {
        crate::abi::type_name_variadic::<ManagedBuffer<'a, Api>>()
    }

    fn is_variadic() -> bool {
        true
    }
}

impl<'a, Api> FunctionCall<'a, Api>
where
    Api: ManagedTypeApi<'a>,
{
    /// Constructs `ESDTTransfer` builtin function call.
    pub(super) fn convert_to_single_transfer_fungible_call(
        self,
        payment: EsdtTokenPayment<Api>,
    ) -> FunctionCall<'a, Api> {
        FunctionCall::new(ESDT_TRANSFER_FUNC_NAME)
            .argument(&payment.token_identifier)
            .argument(&payment.amount)
            .argument(&self)
    }

    /// Constructs `ESDTNFTTransfer` builtin function call.
    ///
    /// `ESDTNFTTransfer` takes 4 arguments:
    /// arg0 - token identifier
    /// arg1 - nonce
    /// arg2 - quantity to transfer
    /// arg3 - destination address
    pub(super) fn convert_to_single_transfer_nft_call(
        self,
        to: &ManagedAddress<Api>,
        payment: EsdtTokenPayment<Api>,
    ) -> FunctionCall<'a, Api> {
        FunctionCall::new(ESDT_NFT_TRANSFER_FUNC_NAME)
            .argument(&payment.token_identifier)
            .argument(&payment.token_nonce)
            .argument(&payment.amount)
            .argument(to)
            .argument(&self)
    }

    /// Constructs `MultiESDTNFTTransfer` builtin function call.
    pub(super) fn convert_to_multi_transfer_esdt_call(
        self,
        to: &ManagedAddress<Api>,
        payments: ManagedVec<Api, EsdtTokenPayment<Api>>,
    ) -> FunctionCall<'a, Api> {
        let mut result = FunctionCall::new(ESDT_MULTI_TRANSFER_FUNC_NAME)
            .argument(&to)
            .argument(&payments.len());

        for payment in payments.into_iter() {
            result = result
                .argument(&payment.token_identifier)
                .argument(&payment.token_nonce)
                .argument(&payment.amount);
        }

        result.argument(&self)
    }
}
