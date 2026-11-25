use multiversx_sc_scenario::{
    ScenarioTxEnvData,
    imports::{NotPayable, ScCallStep, TxToSpecified, UpgradeCall},
    multiversx_sc::{
        tuple_util::NestedTupleFlatten,
        types::{
            Code, DeployCall, RHListExec, Tx, TxBaseWithEnv, TxCodeValue, TxFromSpecified, TxGas,
            TxPayment,
        },
    },
    scenario::tx_to_step::{StepWrapper, TxToStep, address_annotated, code_annotated},
    scenario_model::{ScDeployStep, TxResponse},
};
use multiversx_sdk::{
    chain_core::types::GasResult, data::transaction::Transaction, gateway::GatewayAsyncService,
};

use crate::{DispatchError, InteractorBase, InteractorSimulateGasAsync};

use super::{
    InteractorDispatchAsync, InteractorEnvExec, InteractorExecStep, InteractorIntoSdkTransaction,
    InteractorPrepareAsync, InteractorRunAsync,
};

#[allow(clippy::type_complexity)]
async fn dispatch_async_upgrade<'w, GatewayProxy, From, To, Gas, CodeValue, RH>(
    tx: Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        To,
        NotPayable,
        Gas,
        UpgradeCall<InteractorEnvExec<'w, GatewayProxy>, Code<CodeValue>>,
        RH,
    >,
) -> Result<<RH::ListReturns as NestedTupleFlatten>::Unpacked, DispatchError>
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
    step_wrapper
        .env
        .world
        .sc_call_dispatch(&mut step_wrapper.step)
        .await?;
    Ok(step_wrapper.process_result())
}

#[allow(clippy::type_complexity)]
async fn simulate_gas_async_upgrade<'w, GatewayProxy, From, To, Gas, CodeValue, RH>(
    tx: Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        To,
        NotPayable,
        Gas,
        UpgradeCall<InteractorEnvExec<'w, GatewayProxy>, Code<CodeValue>>,
        RH,
    >,
) -> GasResult
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
{
    let step_wrapper = tx.tx_to_step();
    step_wrapper
        .env
        .world
        .sc_call_simulate(&step_wrapper.step)
        .await
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

    async fn run(self) -> Self::Result {
        dispatch_async_upgrade(self)
            .await
            .unwrap_or_else(|err| panic!("SC upgrade: {err}"))
    }
}

impl<'w, GatewayProxy, From, To, Gas, CodeValue, RH> InteractorDispatchAsync
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

    fn dispatch(self) -> impl std::future::Future<Output = Result<Self::Result, DispatchError>> {
        dispatch_async_upgrade(self)
    }
}

impl<'w, GatewayProxy, From, To, Gas, CodeValue, RH> InteractorIntoSdkTransaction
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
    fn into_sdk_transaction(self) -> Transaction {
        let step_wrapper = self.tx_to_step();
        step_wrapper
            .env
            .world
            .tx_call_to_blockchain_tx(&step_wrapper.step.tx)
    }
}

impl<'w, GatewayProxy, From, To, CodeValue, RH> InteractorSimulateGasAsync
    for Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        To,
        NotPayable,
        (),
        UpgradeCall<InteractorEnvExec<'w, GatewayProxy>, Code<CodeValue>>,
        RH,
    >
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
{
    fn simulate_gas(self) -> impl std::future::Future<Output = GasResult> {
        simulate_gas_async_upgrade(self)
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
