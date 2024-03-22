use crate::{
    contract_base::SendRawWrapper,
    types::{BigUint, EsdtTokenPayment, ManagedAddress, ManagedVec, MultiEsdtPayment},
};

use super::{AnnotatedEgldPayment, FullPaymentData, FunctionCall, TxEgldValue, TxEnv, TxPayment};

impl<Env> TxPayment<Env> for EsdtTokenPayment<Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self) -> bool {
        self.amount == 0u32
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        MultiEsdtPayment::from_single_item(self).perform_transfer_execute(env, to, gas_limit, fc);
    }

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            egld: None,
            multi_esdt: MultiEsdtPayment::from_single_item(self),
        }
    }
}
