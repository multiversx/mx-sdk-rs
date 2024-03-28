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
    scenario_env_util::*,
    scenario_model::{
        AddressValue, BytesValue, ScCallStep, ScDeployStep, TransferStep, TxResponse,
    },
    ScenarioTxEnv, ScenarioTxEnvData, ScenarioTxRun, ScenarioWorld,
};

use crate::{Interactor, InteractorEnvExec, InteractorPrepareAsync};

pub struct InteractorTransferStep<'w> {
    world: &'w mut Interactor,
    step: TransferStep,
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
        let mut sc_call_step =
            tx_to_transfer_step(&self.env, self.from, self.to, self.payment, self.gas);
        InteractorTransferStep {
            world: self.env.world,
            step: sc_call_step,
        }
    }
}

impl<'w> InteractorTransferStep<'w> {
    pub async fn run(self) {
        self.world.transfer(self.step).await;
    }
}
