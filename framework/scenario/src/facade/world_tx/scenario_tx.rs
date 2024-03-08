use multiversx_chain_scenario_format::serde_raw::ValueSubTree;
use multiversx_sc::{
    tuple_util::NestedTupleFlatten,
    types::{
        AnnotatedValue, Code, DeployCall, FunctionCall, ManagedAddress, ManagedBuffer, RHListSync,
        Tx, TxCodeSource, TxCodeSourceSpecified, TxCodeValue, TxEnv, TxFromSpecified, TxGas,
        TxPayment, TxToSpecified,
    },
};

use crate::{
    api::StaticApi,
    scenario_model::{AddressValue, BytesValue, ScCallStep, ScDeployStep},
    ScenarioWorld, WorldRefEnv,
};

use super::{RHListScenario, ScenarioTxEnv, TxScenarioBase};

impl ScenarioWorld {
    fn new_env_data(&self) -> ScenarioTxEnv {
        ScenarioTxEnv {
            context_path: self.current_dir.clone(),
            ..Default::default()
        }
    }

    fn wrap_world_ref<'w>(&'w mut self) -> WorldRefEnv<'w> {
        let data = self.new_env_data();
        WorldRefEnv { world: self, data }
    }

    pub fn tx<'w>(&'w mut self) -> Tx<WorldRefEnv<'w>, (), (), (), (), (), ()> {
        Tx::new_with_env(self.wrap_world_ref())
    }

    pub fn tx_return<STx, F>(&mut self, f: F) -> STx::Returns
    where
        STx: ScenarioTx,
        F: FnOnce(TxScenarioBase) -> STx,
    {
        let env = self.new_env_data();
        let tx_base = TxScenarioBase::new_with_env(env);
        let tx = f(tx_base);
        tx.run_as_scenario_step(self)
    }

    pub fn chain_tx<STx, F>(&mut self, f: F) -> &mut Self
    where
        STx: ScenarioTx<Returns = ()>,
        F: FnOnce(TxScenarioBase) -> STx,
    {
        self.tx_return(f);
        self
    }
}

pub trait ScenarioTx {
    type Returns;

    fn run_as_scenario_step(self, world: &mut ScenarioWorld) -> Self::Returns;
}

pub trait ScenarioTx2 {
    type Returns;

    fn run(self) -> Self::Returns;
}

fn address_annotated<Env, Addr>(env: &Env, from: Addr) -> AddressValue
where
    Env: TxEnv,
    Addr: AnnotatedValue<Env, ManagedAddress<Env::Api>>,
{
    let annotation = from.annotation(env).to_string();
    AddressValue {
        value: from.into_value(env).to_address(),
        original: ValueSubTree::Str(annotation),
    }
}

fn code_annotated<Env, CodeValue>(env: &Env, code: Code<CodeValue>) -> BytesValue
where
    Env: TxEnv,
    CodeValue: TxCodeValue<Env>,
{
    let annotation = code.0.annotation(env).to_string();
    BytesValue {
        value: code.0.into_value(env).to_vec(),
        original: ValueSubTree::Str(annotation),
    }
}

impl<From, To, Payment, Gas, RH> ScenarioTx
    for Tx<ScenarioTxEnv, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>
where
    From: TxFromSpecified<ScenarioTxEnv>,
    To: TxToSpecified<ScenarioTxEnv>,
    Payment: TxPayment<ScenarioTxEnv>,
    Gas: TxGas<ScenarioTxEnv>,
    RH: RHListScenario<ScenarioTxEnv>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run_as_scenario_step(self, world: &mut ScenarioWorld) -> Self::Returns {
        let mut env = self.env;
        let mut step = ScCallStep::new()
            .from(address_annotated(&env, self.from))
            .to(address_annotated(&env, self.to))
            .function(self.data.function_name.to_string().as_str());
        for arg in self.data.arg_buffer.iter_buffers() {
            step.tx.arguments.push(arg.to_vec().into());
        }

        world.sc_call(&mut step);
        let response = step.response.expect("step did not return result");

        let tuple_result = self.result_handler.item_scenario_result(&response);
        tuple_result.flatten_unpack()
    }
}

impl<'w, From, To, Payment, Gas, RH> ScenarioTx2
    for Tx<WorldRefEnv<'w>, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>
where
    From: TxFromSpecified<WorldRefEnv<'w>>,
    To: TxToSpecified<WorldRefEnv<'w>>,
    Payment: TxPayment<WorldRefEnv<'w>>,
    Gas: TxGas<WorldRefEnv<'w>>,
    RH: RHListScenario<WorldRefEnv<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run(self) -> Self::Returns {
        let mut env = self.env;
        let mut step = ScCallStep::new()
            .from(address_annotated(&env, self.from))
            .to(address_annotated(&env, self.to))
            .function(self.data.function_name.to_string().as_str());
        for arg in self.data.arg_buffer.iter_buffers() {
            step.tx.arguments.push(arg.to_vec().into());
        }

        env.world.sc_call(&mut step);
        let response = step.response.expect("step did not return result");

        let tuple_result = self.result_handler.item_scenario_result(&response);
        tuple_result.flatten_unpack()
    }
}

impl<From, Payment, Gas, CodeValue, RH> ScenarioTx
    for Tx<ScenarioTxEnv, From, (), Payment, Gas, DeployCall<ScenarioTxEnv, Code<CodeValue>>, RH>
where
    From: TxFromSpecified<ScenarioTxEnv>,
    Payment: TxPayment<ScenarioTxEnv>,
    Gas: TxGas<ScenarioTxEnv>,
    CodeValue: TxCodeValue<ScenarioTxEnv>,
    RH: RHListScenario<ScenarioTxEnv>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run_as_scenario_step(self, world: &mut ScenarioWorld) -> Self::Returns {
        let mut env = self.env;
        let mut step = ScDeployStep::new()
            .from(address_annotated(&env, self.from))
            .code(code_annotated(&env, self.data.code_source));
        for arg in self.data.arg_buffer.iter_buffers() {
            step.tx.arguments.push(arg.to_vec().into());
        }

        world.sc_deploy(&mut step);
        let response = step.response.expect("step did not return result");

        let tuple_result = self.result_handler.item_scenario_result(&response);
        tuple_result.flatten_unpack()
    }
}
