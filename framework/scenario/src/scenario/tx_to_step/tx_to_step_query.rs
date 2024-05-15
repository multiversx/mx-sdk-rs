use multiversx_sc::types::{FunctionCall, RHListExec, Tx, TxEnv, TxNoPayment, TxToSpecified};

use crate::scenario_model::{ScQueryStep, TxExpect, TxResponse};

use super::{address_annotated, StepWrapper, TxToQueryStep};

impl<Env, To, Payment, RH> TxToQueryStep<Env, RH>
    for Tx<Env, (), To, Payment, (), FunctionCall<Env::Api>, RH>
where
    Env: TxEnv<RHExpect = TxExpect>,
    To: TxToSpecified<Env>,
    Payment: TxNoPayment<Env>,
    RH: RHListExec<TxResponse, Env>,
{
    type Step = ScQueryStep;

    fn tx_to_query_step(self) -> StepWrapper<Env, Self::Step, RH> {
        let mut step = tx_to_sc_query_step(&self.env, self.to, self.data);
        step.expect = Some(self.result_handler.list_tx_expect());

        StepWrapper {
            env: self.env,
            step,
            result_handler: self.result_handler,
        }
    }
}

pub fn tx_to_sc_query_step<Env, To>(env: &Env, to: To, data: FunctionCall<Env::Api>) -> ScQueryStep
where
    Env: TxEnv,
    To: TxToSpecified<Env>,
{
    let mut step = ScQueryStep::new()
        .to(address_annotated(env, &to))
        .function(data.function_name.to_string().as_str());
    for arg in data.arg_buffer.iter_buffers() {
        step.tx.arguments.push(arg.to_vec().into());
    }

    step
}
