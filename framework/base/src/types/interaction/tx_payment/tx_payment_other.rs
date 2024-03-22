use crate::{
    contract_base::SendRawWrapper,
    types::{
        BigUint, Egld, EgldOrEsdtTokenPayment, EgldOrMultiEsdtPayment, ManagedAddress, ManagedVec,
    },
};

use super::{AnnotatedEgldPayment, FullPaymentData, FunctionCall, TxEgldValue, TxEnv, TxPayment};

impl<Env> TxPayment<Env> for EgldOrEsdtTokenPayment<Env::Api>
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
        self.map_egld_or_esdt(
            (to, fc),
            |(to, fc), amount| Egld(amount).perform_transfer_execute(env, to, gas_limit, fc),
            |(to, fc), esdt_payment| esdt_payment.perform_transfer_execute(env, to, gas_limit, fc),
        )
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        self.map_egld_or_esdt(
            (),
            |(), amount| TxPayment::<Env>::into_full_payment_data(Egld(amount), env),
            |(), esdt_payment| TxPayment::<Env>::into_full_payment_data(esdt_payment, env),
        )
    }
}

impl<Env> TxPayment<Env> for EgldOrMultiEsdtPayment<Env::Api>
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
        match self {
            EgldOrMultiEsdtPayment::Egld(egld_amount) => {
                Egld(egld_amount).perform_transfer_execute(env, to, gas_limit, fc)
            },
            EgldOrMultiEsdtPayment::MultiEsdt(multi_esdt_payment) => {
                multi_esdt_payment.perform_transfer_execute(env, to, gas_limit, fc)
            },
        }
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        match self {
            EgldOrMultiEsdtPayment::Egld(egld_amount) => {
                TxPayment::<Env>::into_full_payment_data(Egld(egld_amount), env)
            },
            EgldOrMultiEsdtPayment::MultiEsdt(multi_esdt_payment) => {
                TxPayment::<Env>::into_full_payment_data(multi_esdt_payment, env)
            },
        }
    }
}
