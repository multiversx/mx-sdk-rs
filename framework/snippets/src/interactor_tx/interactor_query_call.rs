use multiversx_sc_scenario::{
    api::StaticApi,
    multiversx_sc::{
        tuple_util::NestedTupleFlatten,
        types::{FunctionCall, RHListExec, Tx, TxBaseWithEnv, TxNoPayment, TxToSpecified},
    },
    scenario::tx_to_step::TxToQueryStep,
    scenario_model::TxResponse,
    ScenarioTxEnvData,
};

use crate::Interactor;

use super::{InteractorEnvQuery, InteractorPrepareAsync, InteractorQueryStep};

impl<'w, To, Payment, RH> InteractorPrepareAsync
    for Tx<InteractorEnvQuery<'w>, (), To, Payment, (), FunctionCall<StaticApi>, RH>
where
    To: TxToSpecified<InteractorEnvQuery<'w>>,
    Payment: TxNoPayment<InteractorEnvQuery<'w>>,
    RH: RHListExec<TxResponse, InteractorEnvQuery<'w>>,
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
    RH: RHListExec<TxResponse, InteractorEnvQuery<'w>>,
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
    pub async fn chain_query<To, Payment, RH, F>(&mut self, f: F) -> &mut Self
    where
        To: TxToSpecified<ScenarioTxEnvData>,
        Payment: TxNoPayment<ScenarioTxEnvData>,
        RH: RHListExec<TxResponse, ScenarioTxEnvData, ListReturns = ()>,
        F: FnOnce(
            TxBaseWithEnv<ScenarioTxEnvData>,
        )
            -> Tx<ScenarioTxEnvData, (), To, Payment, (), FunctionCall<StaticApi>, RH>,
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
