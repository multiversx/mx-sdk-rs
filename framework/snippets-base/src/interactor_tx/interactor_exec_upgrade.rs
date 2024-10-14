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
use multiversx_sdk::gateway::GatewayAsyncService;

use crate::InteractorBase;

use super::{InteractorEnvExec, InteractorExecStep, InteractorPrepareAsync, InteractorRunAsync};

#[allow(clippy::type_complexity)]
async fn run_async_upgrade<'w, GatewayProxy, From, To, Gas, CodeValue, RH>(
    tx: Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        To,
        NotPayable,
        Gas,
        UpgradeCall<InteractorEnvExec<'w, GatewayProxy>, Code<CodeValue>>,
        RH,
    >,
) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    let mut step_wrapper = tx.tx_to_step();
    step_wrapper.env.world.sc_call(&mut step_wrapper.step).await;
    step_wrapper.process_result()
}

impl<'w, GatewayProxy, From, To, Gas, CodeValue, RH> InteractorRunAsync
    for Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        To,
        NotPayable,
        Gas,
        UpgradeCall<InteractorEnvExec<'w, GatewayProxy>, Code<CodeValue>>,
        RH,
    >
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Result = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run(self) -> impl std::future::Future<Output = Self::Result> {
        run_async_upgrade(self)
    }
}

impl<'w, GatewayProxy, From, To, Gas, CodeValue, RH> InteractorPrepareAsync
    for Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        To,
        NotPayable,
        Gas,
        UpgradeCall<InteractorEnvExec<'w, GatewayProxy>, Code<CodeValue>>,
        RH,
    >
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Exec = InteractorExecStep<'w, GatewayProxy, ScCallStep, RH>;

    fn prepare_async(self) -> Self::Exec {
        InteractorExecStep {
            step_wrapper: self.tx_to_step(),
        }
    }
}

impl<'w, GatewayProxy, From, To, Gas, RH, CodeValue>
    TxToStep<InteractorEnvExec<'w, GatewayProxy>, RH>
    for Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        To,
        NotPayable,
        Gas,
        UpgradeCall<InteractorEnvExec<'w, GatewayProxy>, Code<CodeValue>>,
        RH,
    >
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Step = ScCallStep;

    fn tx_to_step(self) -> StepWrapper<InteractorEnvExec<'w, GatewayProxy>, Self::Step, RH> {
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

pub fn tx_to_sc_call_upgrade_step<'a, 'w: 'a, GatewayProxy, From, To, Gas, CodeValue>(
    env: &'a InteractorEnvExec<'w, GatewayProxy>,
    from: From,
    to: To,
    gas: Gas,
    data: UpgradeCall<InteractorEnvExec<'w, GatewayProxy>, Code<CodeValue>>,
) -> ScCallStep
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w, GatewayProxy>>,
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
