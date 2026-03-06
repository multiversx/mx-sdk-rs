use crate::{
    contract_base::TransferExecuteFailed,
    types::{
        BigUint, ManagedAddress, Payment, PaymentVec, TxFrom, TxPaymentCompose, TxToSpecified,
    },
};

use super::{FunctionCall, ScenarioPayments, TxEnv, TxPayment};

impl<Env> TxPayment<Env> for Payment<Env::Api>
where
    Env: TxEnv,
{
    #[inline]
    fn is_no_payment(&self, env: &Env) -> bool {
        (&self).is_no_payment(env)
    }

    #[inline]
    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        self.as_refs()
            .perform_transfer_execute_fallible(env, to, gas_limit, fc)
    }

    #[inline]
    fn perform_transfer_execute_legacy(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        self.as_refs()
            .perform_transfer_execute_legacy(env, to, gas_limit, fc)
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
        self.map_egld_or_esdt(
            (to, fc, f),
            |(to, fc, f), egld_payment| egld_payment.with_normalized(env, from, to, fc, f),
            |(to, fc, f), esdt_payment| esdt_payment.with_normalized(env, from, to, fc, f),
        )
    }

    fn into_scenario_payments(self, env: &Env) -> ScenarioPayments<Env::Api> {
        self.map_egld_or_esdt(
            (),
            |(), egld_payment| TxPayment::<Env>::into_scenario_payments(egld_payment, env),
            |(), esdt_payment| TxPayment::<Env>::into_scenario_payments(esdt_payment, env),
        )
    }
}

impl<Env> TxPaymentCompose<Env, Payment<Env::Api>> for Payment<Env::Api>
where
    Env: TxEnv,
{
    type Output = PaymentVec<Env::Api>;

    fn compose(self, rhs: Payment<Env::Api>) -> Self::Output {
        let mut payments = PaymentVec::new();
        payments.push(self);
        payments.push(rhs);
        payments
    }
}

impl<Env> TxPaymentCompose<Env, Payment<Env::Api>> for PaymentVec<Env::Api>
where
    Env: TxEnv,
{
    type Output = PaymentVec<Env::Api>;

    fn compose(mut self, rhs: Payment<Env::Api>) -> Self::Output {
        self.push(rhs);
        self
    }
}
