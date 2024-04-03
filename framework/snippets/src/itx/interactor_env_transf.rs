use std::path::PathBuf;

use multiversx_sc_scenario::{
    api::StaticApi,
    multiversx_sc::{
        tuple_util::NestedTupleFlatten,
        types::{
            AnnotatedValue, Code, DeployCall, FunctionCall, ManagedAddress, ManagedBuffer, Tx,
            TxBaseWithEnv, TxCodeSource, TxCodeSourceSpecified, TxCodeValue, TxEnv,
            TxFromSpecified, TxGas, TxPayment, TxToSpecified,
        },
    },
    scenario::tx_to_step::{StepWrapper, TxToStep},
    scenario_model::{
        AddressValue, BytesValue, ScCallStep, ScDeployStep, TransferStep, TxResponse,
    },
    ScenarioTxEnv, ScenarioTxEnvData, ScenarioTxRun, ScenarioWorld,
};

use crate::{Interactor, InteractorEnvExec, InteractorPrepareAsync};

pub struct InteractorTransferStep<'w> {
    step_wrapper: StepWrapper<InteractorEnvExec<'w>, TransferStep, ()>,
}

impl<'w, From, To, Payment, Gas> InteractorPrepareAsync
    for Tx<InteractorEnvExec<'w>, From, To, Payment, Gas, (), ()>
where
    From: TxFromSpecified<InteractorEnvExec<'w>>,
    To: TxToSpecified<InteractorEnvExec<'w>>,
    Payment: TxPayment<InteractorEnvExec<'w>>,
    Gas: TxGas<InteractorEnvExec<'w>>,
{
    type Exec = InteractorTransferStep<'w>;

    fn prepare_async(self) -> Self::Exec {
        InteractorTransferStep {
            step_wrapper: self.tx_to_step(),
        }
    }
}

impl<'w> InteractorTransferStep<'w> {
    pub async fn run(mut self) {
        self.step_wrapper
            .env
            .world
            .transfer(self.step_wrapper.step)
            .await;
    }
}
