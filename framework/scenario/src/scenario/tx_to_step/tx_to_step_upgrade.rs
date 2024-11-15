use multiversx_sc::types::{
    Code, NotPayable, RHListExec, Tx, TxCodeValue, TxEnv, TxFromSpecified, TxGas, TxToSpecified,
    UpgradeCall,
};

use crate::{
    imports::ScCallStep,
    scenario_model::{TxExpect, TxResponse},
};

use super::{address_annotated, code_annotated, gas_annotated, StepWrapper, TxToStep};

impl<Env, From, To, Gas, RH, CodeValue> TxToStep<Env, RH>
    for Tx<Env, From, To, NotPayable, Gas, UpgradeCall<Env, Code<CodeValue>>, RH>
where
    Env: TxEnv<RHExpect = TxExpect>,
    From: TxFromSpecified<Env>,
    To: TxToSpecified<Env>,
    Gas: TxGas<Env>,
    CodeValue: TxCodeValue<Env>,
    RH: RHListExec<TxResponse, Env>,
{
    type Step = ScCallStep;

    fn tx_to_step(self) -> StepWrapper<Env, Self::Step, RH> {
        let mut step =
            tx_to_sc_call_upgrade_step(&self.env, self.from, self.to, self.gas, self.data);
        step.expect = Some(self.result_handler.list_tx_expect());

        StepWrapper {
            env: self.env,
            step,
            result_handler: self.result_handler,
        }
    }
}

pub fn tx_to_sc_call_upgrade_step<'a, 'w: 'a, Env, From, To, Gas, CodeValue>(
    env: &Env,
    from: From,
    to: To,
    gas: Gas,
    data: UpgradeCall<Env, Code<CodeValue>>,
) -> ScCallStep
where
    Env: TxEnv,
    From: TxFromSpecified<Env>,
    To: TxToSpecified<Env>,
    Gas: TxGas<Env>,
    CodeValue: TxCodeValue<Env>,
{
    let mut step = ScCallStep::new()
        .from(address_annotated(env, &from))
        .to(address_annotated(env, &to))
        .function("upgradeContract")
        .argument(code_annotated(env, data.code_source))
        .argument(data.code_metadata.to_byte_array().to_vec());

    step.tx.gas_limit = gas_annotated(env, gas);

    for arg in data.arg_buffer.iter_buffers() {
        step.tx.arguments.push(arg.to_vec().into());
    }

    step
}
