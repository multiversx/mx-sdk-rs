use multiversx_sc_scenario::{
    api::StaticApi,
    multiversx_sc::{
        tuple_util::NestedTupleFlatten,
        types::{
            FunctionCall, RHListExec, Tx, TxBaseWithEnv, TxFromSpecified, TxGas, TxPayment,
            TxToSpecified,
        },
    },
    scenario::tx_to_step::TxToStep,
    scenario_model::{ScCallStep, TxResponse},
    ScenarioTxEnvData,
};

use crate::Interactor;

use super::{InteractorEnvExec, InteractorExecStep, InteractorPrepareAsync};

impl<'w, From, To, Payment, Gas, RH> InteractorPrepareAsync
    for Tx<InteractorEnvExec<'w>, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>
where
    From: TxFromSpecified<InteractorEnvExec<'w>>,
    To: TxToSpecified<InteractorEnvExec<'w>>,
    Payment: TxPayment<InteractorEnvExec<'w>>,
    Gas: TxGas<InteractorEnvExec<'w>>,
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

impl<'w, RH> InteractorExecStep<'w, ScCallStep, RH>
where
    RH: RHListExec<TxResponse, InteractorEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub async fn run(mut self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        self.step_wrapper
            .env
            .world
            .sc_call(&mut self.step_wrapper.step)
            .await;
        self.step_wrapper.process_result()
    }
}

impl Interactor {
    pub async fn chain_call<From, To, Payment, Gas, RH, F>(&mut self, f: F) -> &mut Self
    where
        From: TxFromSpecified<ScenarioTxEnvData>,
        To: TxToSpecified<ScenarioTxEnvData>,
        Payment: TxPayment<ScenarioTxEnvData>,
        Gas: TxGas<ScenarioTxEnvData>,
        RH: RHListExec<TxResponse, ScenarioTxEnvData, ListReturns = ()>,
        F: FnOnce(
            TxBaseWithEnv<ScenarioTxEnvData>,
        )
            -> Tx<ScenarioTxEnvData, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>,
    {
        let env = self.new_env_data();
        let tx_base = TxBaseWithEnv::new_with_env(env);
        let tx = f(tx_base);

        let mut step_wrapper = tx.tx_to_step();
        self.sc_call(&mut step_wrapper.step).await;
        step_wrapper.process_result();

        self
    }
}
