use multiversx_sc_scenario::{
    api::StaticApi,
    multiversx_sc::{
        tuple_util::NestedTupleFlatten,
        types::{FunctionCall, RHListExec, Tx, TxBaseWithEnv, TxToSpecified},
    },
    scenario::tx_to_step::TxToQueryStep,
    scenario_model::TxResponse,
    ScenarioTxEnvData,
};

use crate::Interactor;

use super::{InteractorPrepareAsync, InteractorQueryEnv, InteractorQueryStep};

impl<'w, To, RH> InteractorPrepareAsync
    for Tx<InteractorQueryEnv<'w>, (), To, (), (), FunctionCall<StaticApi>, RH>
where
    To: TxToSpecified<InteractorQueryEnv<'w>>,
    RH: RHListExec<TxResponse, InteractorQueryEnv<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Exec = InteractorQueryStep<'w, RH>;

    fn prepare_async(self) -> Self::Exec {
        InteractorQueryStep {
            step_wrapper: self.tx_to_query_step(),
        }
    }
}

impl<'w, RH> InteractorQueryStep<'w, RH>
where
    RH: RHListExec<TxResponse, InteractorQueryEnv<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub async fn run(mut self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        self.step_wrapper
            .env
            .world
            .sc_query(&mut self.step_wrapper.step)
            .await;
        self.step_wrapper.process_result()
    }
}

impl Interactor {
    pub async fn chain_query<To, RH, F>(&mut self, f: F) -> &mut Self
    where
        To: TxToSpecified<ScenarioTxEnvData>,
        RH: RHListExec<TxResponse, ScenarioTxEnvData, ListReturns = ()>,
        F: FnOnce(
            TxBaseWithEnv<ScenarioTxEnvData>,
        ) -> Tx<ScenarioTxEnvData, (), To, (), (), FunctionCall<StaticApi>, RH>,
    {
        let env = self.new_env_data();
        let tx_base = TxBaseWithEnv::new_with_env(env);
        let tx = f(tx_base);

        let mut step_wrapper = tx.tx_to_query_step();
        self.sc_query(&mut step_wrapper.step).await;
        step_wrapper.process_result();
        self
    }
}
