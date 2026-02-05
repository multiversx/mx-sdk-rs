use crate::{
    contract_base::{SendRawWrapper, TransferExecuteFailed},
    types::{
        BigUint, ManagedAddress, MultiTransfer, MultiTransferMarkerArg, PaymentVec, TxFrom,
        TxToSpecified,
    },
};

use super::{FullPaymentData, FunctionCall, TxEnv, TxPayment};

impl<Env, P> TxPayment<Env> for MultiTransfer<P>
where
    Env: TxEnv,
    P: MultiTransferMarkerArg + AsRef<PaymentVec<Env::Api>>,
{
    fn is_no_payment(&self, _env: &Env) -> bool {
        let pv = self.0.as_ref();
        pv.is_empty()
    }

    fn perform_transfer_execute_fallible(
        self,
        _env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        let pv = self.0.as_ref();
        SendRawWrapper::<Env::Api>::new().multi_egld_or_esdt_transfer_execute_fallible(
            to,
            pv.as_multi_egld_or_esdt_payment(),
            gas_limit,
            &fc.function_name,
            &fc.arg_buffer,
        )
    }

    fn perform_transfer_execute_legacy(
        self,
        _env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        let pv = self.0.as_ref();
        SendRawWrapper::<Env::Api>::new().multi_egld_or_esdt_transfer_execute(
            to,
            pv.as_multi_egld_or_esdt_payment(),
            gas_limit,
            &fc.function_name,
            &fc.arg_buffer,
        );
    }

    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R,
    {
        let pv = self.0.as_ref();
        to.with_address_ref(env, |to_addr| {
            let fc_conv =
                fc.convert_to_multi_transfer_esdt_call(to_addr, pv.as_multi_egld_or_esdt_payment());
            f(&from.resolve_address(env), &*BigUint::zero_ref(), fc_conv)
        })
    }

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        let pv = self.0.as_ref();
        FullPaymentData {
            egld: None,
            multi_esdt: pv.clone(),
        }
    }
}
