use multiversx_chain_scenario_format::serde_raw::ValueSubTree;
use multiversx_sc::{
    tuple_util::NestedTupleFlatten,
    types::{
        AnnotatedValue, Code, DeployCall, FunctionCall, ManagedAddress, ManagedBuffer, RHListSync,
        Tx, TxBaseWithEnv, TxCodeSource, TxCodeSourceSpecified, TxCodeValue, TxEnv,
        TxFromSpecified, TxGas, TxPayment, TxToSpecified,
    },
};

use crate::{
    api::StaticApi,
    scenario_model::{AddressValue, BytesValue, ScCallStep, ScDeployStep, TxResponse},
    ScenarioWorld, WorldRefEnv,
};

use super::{RHListScenario, ScenarioTxEnvData};

impl ScenarioWorld {
    fn new_env_data(&self) -> ScenarioTxEnvData {
        ScenarioTxEnvData {
            context_path: self.current_dir.clone(),
            ..Default::default()
        }
    }

    fn wrap_world_ref<'w>(&'w mut self) -> WorldRefEnv<'w> {
        let data = self.new_env_data();
        WorldRefEnv { world: self, data }
    }

    pub fn tx<'w>(&'w mut self) -> TxBaseWithEnv<WorldRefEnv<'w>> {
        Tx::new_with_env(self.wrap_world_ref())
    }

    pub fn chain_tx<STx, F>(&mut self, f: F) -> &mut Self
    where
        STx: ScenarioTxRunOnWorld<Returns = ()>,
        F: FnOnce(TxBaseWithEnv<ScenarioTxEnvData>) -> STx,
    {
        let env = self.new_env_data();
        let tx_base = TxBaseWithEnv::new_with_env(env);
        let tx = f(tx_base);
        tx.run_on_world(self);
        self
    }
}

pub trait ScenarioTxRunOnWorld {
    type Returns;

    fn run_on_world(self, world: &mut ScenarioWorld) -> Self::Returns;
}

pub trait ScenarioTxRun {
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

fn tx_to_sc_call_step<Env, From, To, Payment, Gas>(
    env: &Env,
    from: From,
    to: To,
    _payment: Payment,
    _gas: Gas,
    data: FunctionCall<Env::Api>,
) -> ScCallStep
where
    Env: TxEnv,
    From: TxFromSpecified<Env>,
    To: TxToSpecified<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
{
    let mut step = ScCallStep::new()
        .from(address_annotated(env, from))
        .to(address_annotated(env, to))
        .function(data.function_name.to_string().as_str());
    for arg in data.arg_buffer.iter_buffers() {
        step.tx.arguments.push(arg.to_vec().into());
    }

    step
}

fn tx_to_sc_deploy_step<Env, From, Payment, Gas, CodeValue>(
    env: &Env,
    from: From,
    _payment: Payment,
    _gas: Gas,
    data: DeployCall<Env, Code<CodeValue>>,
) -> ScDeployStep
where
    Env: TxEnv,
    From: TxFromSpecified<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    CodeValue: TxCodeValue<Env>,
{
    let mut step = ScDeployStep::new()
        .from(address_annotated(env, from))
        .code(code_annotated(env, data.code_source));
    for arg in data.arg_buffer.iter_buffers() {
        step.tx.arguments.push(arg.to_vec().into());
    }

    step
}

fn process_result<Env, RH>(
    response: Option<TxResponse>,
    result_handler: RH,
) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked
where
    Env: TxEnv,
    RH: RHListScenario<Env>,
    RH::ListReturns: NestedTupleFlatten,
{
    let response = response.expect("step did not return result");
    let tuple_result = result_handler.item_scenario_result(&response);
    tuple_result.flatten_unpack()
}

impl<From, To, Payment, Gas, RH> ScenarioTxRunOnWorld
    for Tx<ScenarioTxEnvData, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>
where
    From: TxFromSpecified<ScenarioTxEnvData>,
    To: TxToSpecified<ScenarioTxEnvData>,
    Payment: TxPayment<ScenarioTxEnvData>,
    Gas: TxGas<ScenarioTxEnvData>,
    RH: RHListScenario<ScenarioTxEnvData>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run_on_world(self, world: &mut ScenarioWorld) -> Self::Returns {
        let mut step = tx_to_sc_call_step(
            &self.env,
            self.from,
            self.to,
            self.payment,
            self.gas,
            self.data,
        );
        world.sc_call(&mut step);
        process_result(step.response, self.result_handler)
    }
}

impl<'w, From, To, Payment, Gas, RH> ScenarioTxRun
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
        let mut step = tx_to_sc_call_step(
            &self.env,
            self.from,
            self.to,
            self.payment,
            self.gas,
            self.data,
        );
        self.env.world.sc_call(&mut step);
        process_result(step.response, self.result_handler)
    }
}

impl<From, Payment, Gas, CodeValue, RH> ScenarioTxRunOnWorld
    for Tx<
        ScenarioTxEnvData,
        From,
        (),
        Payment,
        Gas,
        DeployCall<ScenarioTxEnvData, Code<CodeValue>>,
        RH,
    >
where
    From: TxFromSpecified<ScenarioTxEnvData>,
    Payment: TxPayment<ScenarioTxEnvData>,
    Gas: TxGas<ScenarioTxEnvData>,
    CodeValue: TxCodeValue<ScenarioTxEnvData>,
    RH: RHListScenario<ScenarioTxEnvData>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run_on_world(self, world: &mut ScenarioWorld) -> Self::Returns {
        let mut step =
            tx_to_sc_deploy_step(&self.env, self.from, self.payment, self.gas, self.data);
        world.sc_deploy(&mut step);
        process_result(step.response, self.result_handler)
    }
}

impl<'w, From, Payment, Gas, CodeValue, RH> ScenarioTxRun
    for Tx<
        WorldRefEnv<'w>,
        From,
        (),
        Payment,
        Gas,
        DeployCall<WorldRefEnv<'w>, Code<CodeValue>>,
        RH,
    >
where
    From: TxFromSpecified<WorldRefEnv<'w>>,
    Payment: TxPayment<WorldRefEnv<'w>>,
    Gas: TxGas<WorldRefEnv<'w>>,
    CodeValue: TxCodeValue<WorldRefEnv<'w>>,
    RH: RHListScenario<WorldRefEnv<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run(self) -> Self::Returns {
        let mut step =
            tx_to_sc_deploy_step(&self.env, self.from, self.payment, self.gas, self.data);
        self.env.world.sc_deploy(&mut step);
        process_result(step.response, self.result_handler)
    }
}
