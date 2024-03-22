use crate::{
    contract_base::SendRawWrapper,
    types::{BigUint, ManagedAddress, ManagedVec, MultiEsdtPayment},
};

use super::{AnnotatedEgldPayment, FullPaymentData, FunctionCall, TxEgldValue, TxEnv, TxPayment};

impl<Env> TxPayment<Env> for &MultiEsdtPayment<Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self) -> bool {
        self.is_empty()
    }

    fn perform_transfer_execute(
        self,
        _env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        let _ = SendRawWrapper::<Env::Api>::new().multi_esdt_transfer_execute(
            to,
            self,
            gas_limit,
            &fc.function_name,
            &fc.arg_buffer,
        );
    }

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            egld: None,
            multi_esdt: self.clone(),
        }
    }
}

impl<Env> TxPayment<Env> for MultiEsdtPayment<Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self) -> bool {
        self.is_empty()
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        (&self).perform_transfer_execute(env, to, gas_limit, fc);
    }

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            egld: None,
            multi_esdt: self,
        }
    }
}
