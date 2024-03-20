use std::path::PathBuf;

use multiversx_sc::{
    tuple_util::NestedTupleFlatten,
    types::{
        AnnotatedValue, FunctionCall, ManagedAddress, Tx, TxBaseWithEnv, TxEnv, TxFromSpecified,
        TxGas, TxPayment, TxToSpecified,
    },
};

use crate::{
    api::StaticApi, scenario_model::TxResponse, RHListScenario, ScenarioTxEnv, ScenarioTxEnvData,
    ScenarioTxRun, ScenarioWorld,
};

use super::scenario_env_util::*;

pub struct ScenarioEnvQuery<'w> {
    pub world: &'w mut ScenarioWorld,
    pub data: ScenarioTxEnvData,
}

impl<'w> TxEnv for ScenarioEnvQuery<'w> {
    type Api = StaticApi;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api> {
        panic!("Explicit sender address expected")
    }

    fn default_gas(&self) -> u64 {
        self.data.default_gas()
    }
}

impl<'w> ScenarioTxEnv for ScenarioEnvQuery<'w> {
    fn env_data(&self) -> &ScenarioTxEnvData {
        &self.data
    }
}

impl<'w, To, RH> ScenarioTxRun
    for Tx<ScenarioEnvQuery<'w>, (), To, (), (), FunctionCall<StaticApi>, RH>
where
    To: TxToSpecified<ScenarioEnvQuery<'w>>,
    RH: RHListScenario<ScenarioEnvQuery<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run(self) -> Self::Returns {
        let mut step = tx_to_sc_query_step(&self.env, self.to, self.data);
        self.env.world.sc_query(&mut step);
        process_result(step.response, self.result_handler)
    }
}

impl ScenarioWorld {
    pub fn query(&mut self) -> TxBaseWithEnv<ScenarioEnvQuery<'_>> {
        let data = self.new_env_data();
        let env = ScenarioEnvQuery { world: self, data };
        Tx::new_with_env(env)
    }

    pub fn chain_query<To, RH, F>(&mut self, f: F) -> &mut Self
    where
        To: TxToSpecified<ScenarioTxEnvData>,
        RH: RHListScenario<ScenarioTxEnvData, ListReturns = ()>,
        F: FnOnce(
            TxBaseWithEnv<ScenarioTxEnvData>,
        ) -> Tx<ScenarioTxEnvData, (), To, (), (), FunctionCall<StaticApi>, RH>,
    {
        let env = self.new_env_data();
        let tx_base = TxBaseWithEnv::new_with_env(env);
        let tx = f(tx_base);
        let mut step = tx_to_sc_query_step(&tx.env, tx.to, tx.data);
        self.sc_query(&mut step);
        process_result(step.response, tx.result_handler);
        self
    }
}
