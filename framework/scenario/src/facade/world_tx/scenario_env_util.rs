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
    RHListScenario, ScenarioEnvExec, ScenarioWorld,
};

pub(super) fn address_annotated<Env, Addr>(env: &Env, from: Addr) -> AddressValue
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

pub(super) fn code_annotated<Env, CodeValue>(env: &Env, code: Code<CodeValue>) -> BytesValue
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

pub(super) fn tx_to_sc_call_step<Env, From, To, Payment, Gas>(
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

pub(super) fn tx_to_sc_deploy_step<Env, From, Payment, Gas, CodeValue>(
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

pub(super) fn process_result<Env, RH>(
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
