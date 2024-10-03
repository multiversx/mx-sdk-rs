use multiversx_sc_scenario::{
    imports::{NotPayable, ScCallStep, TxToSpecified, UpgradeCall},
    multiversx_sc::{
        tuple_util::NestedTupleFlatten,
        types::{
            Code, DeployCall, RHListExec, Tx, TxBaseWithEnv, TxCodeValue, TxFromSpecified, TxGas,
            TxPayment,
        },
    },
    scenario::tx_to_step::{address_annotated, code_annotated, StepWrapper, TxToStep},
    scenario_model::{ScDeployStep, TxResponse},
    ScenarioTxEnvData,
};

use crate::InteractorBase;

use super::{InteractorEnvExec, InteractorExecStep, InteractorPrepareAsync};

impl<'w, From, To, Gas, CodeValue, RH> InteractorPrepareAsync
    for Tx<
        InteractorEnvExec<'w>,
        From,
        To,
        NotPayable,
        Gas,
        UpgradeCall<InteractorEnvExec<'w>, Code<CodeValue>>,
        RH,
    >
where
    From: TxFromSpecified<InteractorEnvExec<'w>>,
    To: TxToSpecified<InteractorEnvExec<'w>>,
    Gas: TxGas<InteractorEnvExec<'w>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Exec = InteractorExecStep<'w, ScCallStep, RH>;

    fn prepare_async(self) -> Self::Exec {
        InteractorExecStep {
            step_wrapper: self.tx_to_step(),
        }
    }
}

impl<'w, From, To, Gas, RH, CodeValue> TxToStep<InteractorEnvExec<'w>, RH>
    for Tx<
        InteractorEnvExec<'w>,
        From,
        To,
        NotPayable,
        Gas,
        UpgradeCall<InteractorEnvExec<'w>, Code<CodeValue>>,
        RH,
    >
where
    From: TxFromSpecified<InteractorEnvExec<'w>>,
    To: TxToSpecified<InteractorEnvExec<'w>>,
    Gas: TxGas<InteractorEnvExec<'w>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Step = ScCallStep;

    fn tx_to_step(self) -> StepWrapper<InteractorEnvExec<'w>, Self::Step, RH> {
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

pub fn tx_to_sc_call_upgrade_step<'a, 'w: 'a, From, To, Gas, CodeValue>(
    env: &'a InteractorEnvExec<'w>,
    from: From,
    to: To,
    gas: Gas,
    data: UpgradeCall<InteractorEnvExec<'w>, Code<CodeValue>>,
) -> ScCallStep
where
    From: TxFromSpecified<InteractorEnvExec<'w>>,
    To: TxToSpecified<InteractorEnvExec<'w>>,
    Gas: TxGas<InteractorEnvExec<'w>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w>>,
{
    let mut step = ScCallStep::new()
        .from(address_annotated(env, &from))
        .to(address_annotated(env, &to))
        .gas_limit(gas.gas_value(env))
        .function("upgradeContract")
        .argument(code_annotated(env, data.code_source))
        .argument(data.code_metadata.to_byte_array().to_vec());

    for arg in data.arg_buffer.iter_buffers() {
        step.tx.arguments.push(arg.to_vec().into());
    }

    step
}
