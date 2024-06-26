use multiversx_sc_scenario::{
    imports::{NotPayable, ScCallStep, TxToSpecified, UpgradeCall},
    multiversx_sc::{
        tuple_util::NestedTupleFlatten,
        types::{
            Code, DeployCall, RHListExec, Tx, TxBaseWithEnv, TxCodeValue, TxFromSpecified, TxGas,
            TxPayment,
        },
    },
    scenario::tx_to_step::{address_annotated, StepWrapper, TxToStep},
    scenario_model::{ScDeployStep, TxResponse},
    ScenarioTxEnvData,
};

use crate::Interactor;

use super::{InteractorEnvExec, InteractorExecStep, InteractorPrepareAsync};

impl<'w, From, Gas, CodeValue, RH> InteractorPrepareAsync
    for Tx<
        InteractorEnvExec<'w>,
        From,
        (),
        NotPayable,
        Gas,
        UpgradeCall<InteractorEnvExec<'w>, Code<CodeValue>>,
        RH,
    >
where
    From: TxFromSpecified<InteractorEnvExec<'w>>,
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

impl<'w, From, Gas, RH, CodeValue> TxToStep<InteractorEnvExec<'w>, RH>
    for Tx<
        InteractorEnvExec<'w>,
        From,
        (),
        NotPayable,
        Gas,
        UpgradeCall<InteractorEnvExec<'w>, Code<CodeValue>>,
        RH,
    >
where
    From: TxFromSpecified<InteractorEnvExec<'w>>,
    Gas: TxGas<InteractorEnvExec<'w>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Step = ScCallStep;

    fn tx_to_step(self) -> StepWrapper<InteractorEnvExec<'w>, Self::Step, RH> {
        let mut step = tx_to_sc_call_upgrade_step(&self.env, self.from, self.gas, self.data);
        step.expect = Some(self.result_handler.list_tx_expect());

        StepWrapper {
            env: self.env,
            step,
            result_handler: self.result_handler,
        }
    }
}

pub fn tx_to_sc_call_upgrade_step<'a, 'w: 'a, From, Gas, CodeValue>(
    env: &'a InteractorEnvExec<'w>,
    from: From,
    gas: Gas,
    data: UpgradeCall<InteractorEnvExec<'w>, Code<CodeValue>>,
) -> ScCallStep
where
    From: TxFromSpecified<InteractorEnvExec<'w>>,
    Gas: TxGas<InteractorEnvExec<'w>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w>>,
{
    let mut step = ScCallStep::new()
        .from(address_annotated(env, &from))
        .function("upgrade")
        .gas_limit(gas.gas_value(env));
    for arg in data.arg_buffer.iter_buffers() {
        step.tx.arguments.push(arg.to_vec().into());
    }

    step
}
