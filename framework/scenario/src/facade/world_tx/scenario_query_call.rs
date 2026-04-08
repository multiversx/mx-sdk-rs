use multiversx_sc::{
    tuple_util::NestedTupleFlatten,
    types::{
        FunctionCall, H256, ManagedAddress, ManagedBuffer, RHListExec, Tx, TxBaseWithEnv, TxEnv,
        TxEnvWithTxHash, TxId, TxNoPayment, TxToSpecified,
    },
};

use crate::{
    ScenarioTxEnv, ScenarioTxEnvData, ScenarioTxRun, ScenarioWorld,
    api::StaticApi,
    scenario::tx_to_step::TxToQueryStep,
    scenario_model::{TxExpect, TxResponse},
};

pub struct ScenarioEnvQuery<'w> {
    pub world: &'w mut ScenarioWorld,
    pub data: ScenarioTxEnvData,
}

impl TxEnv for ScenarioEnvQuery<'_> {
    type Api = StaticApi;

    type RHExpect = TxExpect;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api> {
        panic!("Explicit sender address expected")
    }

    fn default_gas_annotation(&self) -> ManagedBuffer<Self::Api> {
        self.data.default_gas_annotation()
    }

    fn default_gas_value(&self) -> u64 {
        self.data.default_gas_value()
    }
}

impl ScenarioTxEnv for ScenarioEnvQuery<'_> {
    fn env_data(&self) -> &ScenarioTxEnvData {
        &self.data
    }
}

impl<'w, To, Payment, RH> ScenarioTxRun
    for Tx<ScenarioEnvQuery<'w>, (), To, Payment, (), FunctionCall<StaticApi>, RH>
where
    To: TxToSpecified<ScenarioEnvQuery<'w>>,
    Payment: TxNoPayment<ScenarioEnvQuery<'w>>,
    RH: RHListExec<TxResponse, ScenarioEnvQuery<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run(self) -> Self::Returns {
        let mut step_wrapper = self.tx_to_query_step();
        step_wrapper.env.world.sc_query(&mut step_wrapper.step);
        step_wrapper.process_result()
    }
}

impl ScenarioWorld {
    pub fn query(&mut self) -> TxBaseWithEnv<ScenarioEnvQuery<'_>> {
        let data = self.new_env_data();
        let env = ScenarioEnvQuery { world: self, data };
        Tx::new_with_env(env)
    }

    pub fn chain_query<To, Payment, RH, F>(&mut self, f: F) -> &mut Self
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
        self.sc_query(&mut step_wrapper.step);
        step_wrapper.process_result();
        self
    }
}

impl TxEnvWithTxHash for ScenarioEnvQuery<'_> {
    fn set_tx_id(&mut self, tx_id: TxId) {
        self.data.set_tx_id(tx_id);
    }

    fn take_tx_id(&mut self) -> Option<TxId> {
        self.data.take_tx_id()
    }

    fn set_tx_hash(&mut self, tx_hash: H256) {
        self.data.set_tx_hash(tx_hash);
    }

    fn take_tx_hash(&mut self) -> Option<H256> {
        self.data.take_tx_hash()
    }
}
