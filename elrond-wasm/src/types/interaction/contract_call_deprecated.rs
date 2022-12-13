use crate::{
    api::CallTypeApi,
    contract_base::SendRawWrapper,
    types::{BigUint, EgldOrEsdtTokenIdentifier, TokenIdentifier},
};

use super::ContractCall;

impl<SA, OriginalResult> ContractCall<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    #[deprecated(
        since = "0.38.0",
        note = "Replace by `contract_call.with_esdt_transfer((payment_token, payment_nonce, payment_amount))`. 
        The tuple argument will get automatically converted to EsdtTokenPayment."
    )]
    pub fn add_esdt_token_transfer(
        self,
        payment_token: TokenIdentifier<SA>,
        payment_nonce: u64,
        payment_amount: BigUint<SA>,
    ) -> Self {
        self.with_esdt_transfer((payment_token, payment_nonce, payment_amount))
    }

    #[deprecated(
        since = "0.38.0",
        note = "Replace by `contract_call.with_egld_or_single_esdt_transfer((payment_token, payment_nonce, payment_amount))`. "
    )]
    pub fn with_egld_or_single_esdt_token_transfer(
        self,
        payment_token: EgldOrEsdtTokenIdentifier<SA>,
        payment_nonce: u64,
        payment_amount: BigUint<SA>,
    ) -> Self {
        self.with_egld_or_single_esdt_transfer((payment_token, payment_nonce, payment_amount))
    }

    /// Executes immediately, synchronously.
    ///
    /// The result (if any) is ignored.
    ///
    /// Deprecated and will be removed soon. Use `let _: IgnoreValue = contract_call.execute_on_dest_context(...)` instead.
    #[deprecated(
        since = "0.36.1",
        note = "Redundant method, use `let _: IgnoreValue = contract_call.execute_on_dest_context(...)` instead"
    )]
    pub fn execute_on_dest_context_ignore_result(mut self) {
        self = self.convert_to_esdt_transfer_call();
        let _ = SendRawWrapper::<SA>::new().execute_on_dest_context_raw(
            self.resolve_gas_limit(),
            &self.to,
            &self.egld_payment,
            &self.endpoint_name,
            &self.arg_buffer,
        );

        SendRawWrapper::<SA>::new().clean_return_data();
    }
}
