use multiversx_sc::{
    tuple_util::NestedTupleFlatten,
    types::{
        heap::H256, Code, FunctionCall, ManagedAddress, ManagedBuffer, NotPayable, RHListExec, Tx,
        TxBaseWithEnv, TxEnv, TxEnvMockDeployAddress, TxEnvWithTxHash, TxFromSpecified, TxGas,
        TxPayment, TxToSpecified, UpgradeCall,
    },
};

use crate::{
    api::StaticApi,
    imports::MxscPath,
    scenario::tx_to_step::{address_annotated, TxToStep},
    scenario_model::{SetStateStep, TxExpect, TxResponse},
    ScenarioTxEnv, ScenarioTxRun, ScenarioWorld,
};

use super::ScenarioTxEnvData;

/// Environment for executing transactions.
pub struct ScenarioEnvExec<'w> {
    pub world: &'w mut ScenarioWorld,
    pub data: ScenarioTxEnvData,
}

impl<'w> TxEnv for ScenarioEnvExec<'w> {
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

impl<'w> TxEnvMockDeployAddress for ScenarioEnvExec<'w> {
    fn mock_deploy_new_address<From, NA>(&mut self, from: &From, new_address: NA)
    where
        From: TxFromSpecified<Self>,
        NA: multiversx_sc::types::AnnotatedValue<Self, ManagedAddress<Self::Api>>,
    {
        let from_value = address_annotated(self, from);
        let sender_nonce = self
            .world
            .get_state()
            .accounts
            .get(&from_value.to_vm_address())
            .expect("sender does not exist")
            .nonce;
        let new_address_value = address_annotated(self, &new_address);

        self.world.set_state_step(SetStateStep::new().new_address(
            from_value,
            sender_nonce,
            new_address_value,
        ));
    }
}

impl<'w> ScenarioTxEnv for ScenarioEnvExec<'w> {
    fn env_data(&self) -> &ScenarioTxEnvData {
        &self.data
    }
}

impl<'w, From, To, Payment, Gas, RH> ScenarioTxRun
    for Tx<ScenarioEnvExec<'w>, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>
where
    From: TxFromSpecified<ScenarioEnvExec<'w>>,
    To: TxToSpecified<ScenarioEnvExec<'w>>,
    Payment: TxPayment<ScenarioEnvExec<'w>>,
    Gas: TxGas<ScenarioEnvExec<'w>>,
    RH: RHListExec<TxResponse, ScenarioEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run(self) -> Self::Returns {
        let mut step_wrapper = self.tx_to_step();
        step_wrapper.step.explicit_tx_hash = core::mem::take(&mut step_wrapper.env.data.tx_hash);
        step_wrapper.env.world.sc_call(&mut step_wrapper.step);
        step_wrapper.process_result()
    }
}

impl<'w, From, To, RH> ScenarioTxRun
    for Tx<
        ScenarioEnvExec<'w>,
        From,
        To,
        NotPayable,
        (),
        UpgradeCall<ScenarioEnvExec<'w>, Code<MxscPath<'w>>>,
        RH,
    >
where
    From: TxFromSpecified<ScenarioEnvExec<'w>>,
    To: TxToSpecified<ScenarioEnvExec<'w>>,
    RH: RHListExec<TxResponse, ScenarioEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run(self) -> Self::Returns {
        let mut step_wrapper = self.tx_to_step();
        step_wrapper.step.explicit_tx_hash = core::mem::take(&mut step_wrapper.env.data.tx_hash);
        step_wrapper.env.world.sc_call(&mut step_wrapper.step);
        step_wrapper.process_result()
    }
}

impl<'w> TxEnvWithTxHash for ScenarioEnvExec<'w> {
    fn set_tx_hash(&mut self, tx_hash: H256) {
        assert!(self.data.tx_hash.is_none(), "tx hash set twice");
        self.data.tx_hash = Some(tx_hash);
    }
}

impl ScenarioWorld {
    pub fn tx(&mut self) -> TxBaseWithEnv<ScenarioEnvExec<'_>> {
        let data = self.new_env_data();
        let env = ScenarioEnvExec { world: self, data };
        Tx::new_with_env(env)
    }

    pub fn chain_call<From, To, Payment, Gas, RH, F>(&mut self, f: F) -> &mut Self
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
        self.sc_call(&mut step_wrapper.step);
        step_wrapper.process_result();
        self
    }
}
