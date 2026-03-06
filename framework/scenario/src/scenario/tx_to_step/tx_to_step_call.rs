use multiversx_sc::types::{
    FunctionCall, RHListExec, Tx, TxEnv, TxEnvWithTxHash, TxFromSpecified, TxGas, TxPayment,
    TxToSpecified,
};

use crate::scenario_model::{ScCallStep, TxESDT, TxExpect, TxResponse};

use super::{StepWrapper, TxToStep, address_annotated, gas_annotated};

impl<Env, From, To, Payment, Gas, RH> TxToStep<Env, RH>
    for Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, RH>
where
    Env: TxEnvWithTxHash<RHExpect = TxExpect>,
    From: TxFromSpecified<Env>,
    To: TxToSpecified<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    RH: RHListExec<TxResponse, Env>,
{
    type Step = ScCallStep;

    fn tx_to_step(mut self) -> StepWrapper<Env, Self::Step, RH> {
        let mut step = tx_to_sc_call_step(
            &self.env,
            self.from,
            self.to,
            self.payment,
            self.gas,
            self.data,
        );
        step.tx_id = self.env.take_tx_id();
        step.explicit_tx_hash = self.env.take_tx_hash();
        step.expect = Some(self.result_handler.list_preprocessing());

        StepWrapper {
            env: self.env,
            step,
            result_handler: self.result_handler,
        }
    }
}

pub fn tx_to_sc_call_step<Env, From, To, Payment, Gas>(
    env: &Env,
    from: From,
    to: To,
    payment: Payment,
    gas: Gas,
    data: FunctionCall<Env::Api>,
) -> ScCallStep
where
    Env: TxEnv,
    From: TxFromSpecified<Env>,
    To: TxToSpecified<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
{
    let mut step = ScCallStep::new()
        .from(address_annotated(env, &from))
        .to(address_annotated(env, &to))
        .function(data.function_name.to_string().as_str());
    for arg in data.arg_buffer.iter_buffers() {
        step.tx.arguments.push(arg.to_vec().into());
    }

    step.tx.gas_limit = gas_annotated(env, gas);

    let full_payment_data = payment.into_scenario_payments(env);
    if let Some(annotated_egld_payment) = full_payment_data.egld {
        step.tx.egld_value = annotated_egld_payment.into();
    } else {
        step.tx.esdt_value = full_payment_data
            .multi_esdt
            .iter()
            .map(|item| TxESDT::from(item.clone()))
            .collect();
    }

    step
}
