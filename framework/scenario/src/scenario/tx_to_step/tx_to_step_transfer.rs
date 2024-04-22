use multiversx_sc::types::{Tx, TxEnv, TxFromSpecified, TxGas, TxPayment, TxToSpecified};

use crate::scenario_model::TransferStep;

use super::{address_annotated, gas_annotated, StepWrapper, TxToStep};

impl<Env, From, To, Payment, Gas> TxToStep<Env, ()> for Tx<Env, From, To, Payment, Gas, (), ()>
where
    Env: TxEnv,
    From: TxFromSpecified<Env>,
    To: TxToSpecified<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
{
    type Step = TransferStep;

    fn tx_to_step(self) -> StepWrapper<Env, Self::Step, ()> {
        let step = tx_to_transfer_step(&self.env, self.from, self.to, self.payment, self.gas);

        StepWrapper {
            env: self.env,
            step,
            result_handler: self.result_handler,
        }
    }
}

pub fn tx_to_transfer_step<Env, From, To, Payment, Gas>(
    env: &Env,
    from: From,
    to: To,
    payment: Payment,
    gas: Gas,
) -> TransferStep
where
    Env: TxEnv,
    From: TxFromSpecified<Env>,
    To: TxToSpecified<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
{
    let mut step = TransferStep::new()
        .from(address_annotated(env, &from))
        .to(address_annotated(env, &to));

    step.tx.gas_limit = gas_annotated(env, gas);

    let full_payment_data = payment.into_full_payment_data(env);
    if let Some(annotated_egld_payment) = full_payment_data.egld {
        step.tx.egld_value = annotated_egld_payment.into();
    }

    step
}
