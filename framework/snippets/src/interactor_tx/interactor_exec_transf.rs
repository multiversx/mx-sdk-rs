use multiversx_sc_scenario::{
    multiversx_sc::types::{Tx, TxFromSpecified, TxGas, TxPayment, TxToSpecified},
    scenario::tx_to_step::TxToStep,
    scenario_model::TransferStep,
};

use super::{InteractorExecEnv, InteractorExecStep, InteractorPrepareAsync};

impl<'w, From, To, Payment, Gas> InteractorPrepareAsync
    for Tx<InteractorExecEnv<'w>, From, To, Payment, Gas, (), ()>
where
    From: TxFromSpecified<InteractorExecEnv<'w>>,
    To: TxToSpecified<InteractorExecEnv<'w>>,
    Payment: TxPayment<InteractorExecEnv<'w>>,
    Gas: TxGas<InteractorExecEnv<'w>>,
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
