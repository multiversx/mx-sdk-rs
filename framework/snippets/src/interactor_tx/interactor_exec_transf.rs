use multiversx_sc_scenario::{
    multiversx_sc::types::{Tx, TxFromSpecified, TxGas, TxPayment, TxToSpecified},
    scenario::tx_to_step::TxToStep,
    scenario_model::TransferStep,
};

use super::{InteractorEnvExec, InteractorExecStep, InteractorPrepareAsync};

impl<'w, From, To, Payment, Gas> InteractorPrepareAsync
    for Tx<InteractorEnvExec<'w>, From, To, Payment, Gas, (), ()>
where
    From: TxFromSpecified<InteractorEnvExec<'w>>,
    To: TxToSpecified<InteractorEnvExec<'w>>,
    Payment: TxPayment<InteractorEnvExec<'w>>,
    Gas: TxGas<InteractorEnvExec<'w>>,
{
    type Exec = InteractorExecStep<'w, TransferStep, ()>;

    fn prepare_async(self) -> Self::Exec {
        InteractorExecStep {
            step_wrapper: self.tx_to_step(),
        }
    }
}

impl<'w> InteractorExecStep<'w, TransferStep, ()> {
    pub async fn run(self) {
        self.step_wrapper
            .env
            .world
            .transfer(self.step_wrapper.step)
            .await;
    }
}
