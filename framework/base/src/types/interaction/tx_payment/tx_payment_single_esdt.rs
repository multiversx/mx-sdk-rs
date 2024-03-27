use crate::{
    contract_base::SendRawWrapper,
    types::{
        BigUint, EsdtTokenPayment, ManagedAddress, ManagedVec, MultiEsdtPayment, TxFrom,
        TxToSpecified,
    },
};

use super::{AnnotatedEgldPayment, FullPaymentData, FunctionCall, TxEgldValue, TxEnv, TxPayment};

impl<Env> TxPayment<Env> for EsdtTokenPayment<Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.amount == 0u32
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        self.as_refs()
            .perform_transfer_execute(env, to, gas_limit, fc);
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
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        self.as_refs().with_normalized(env, from, to, fc, f)
    }

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            egld: None,
            multi_esdt: MultiEsdtPayment::from_single_item(self),
        }
    }
}
