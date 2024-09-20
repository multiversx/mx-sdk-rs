use crate::debug_executor::contract_instance_wrapped_execution;
use crate::scenario::tx_to_step::TxToQueryStep;
use crate::{
    imports::StaticApi, scenario::tx_to_step::TxToStep, scenario_model::TxResponse, ScenarioEnvExec,
};
use crate::{DebugApi, ScenarioEnvQuery};
use multiversx_chain_vm::tx_mock::TxFunctionName;
use multiversx_sc::contract_base::ContractBase;
use multiversx_sc::{
    tuple_util::NestedTupleFlatten,
    types::{
        Code, DeployCall, FunctionCall, RHListExec, Tx, TxCodeValue, TxFromSpecified, TxNoPayment,
        TxPayment, TxToSpecified,
    },
};

pub trait ScenarioTxWhitebox {
    type Returns;

    /// Runs a lambda function in the name of a smart contract, with the configured transaction context.
    fn whitebox<ContractObj, F>(self, contract_obj: fn() -> ContractObj, f: F) -> Self::Returns
    where
        ContractObj: ContractBase<Api = DebugApi> + 'static,
        F: FnOnce(ContractObj);
}

impl<'w, From, Payment, CodeValue, RH> ScenarioTxWhitebox
    for Tx<
        ScenarioEnvExec<'w>,
        From,
        (),
        Payment,
        (),
        DeployCall<ScenarioEnvExec<'w>, Code<CodeValue>>,
        RH,
    >
where
    From: TxFromSpecified<ScenarioEnvExec<'w>>,
    Payment: TxNoPayment<ScenarioEnvExec<'w>>,
    CodeValue: TxCodeValue<ScenarioEnvExec<'w>>,
    RH: RHListExec<TxResponse, ScenarioEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn whitebox<ContractObj, F>(
        self,
        contract_obj_builder: fn() -> ContractObj,
        f: F,
    ) -> Self::Returns
    where
        ContractObj: ContractBase<Api = DebugApi> + 'static,
        F: FnOnce(ContractObj),
    {
        let contract_obj = contract_obj_builder();

        let mut step_wrapper = self.tx_to_step();
        let (new_address, tx_result) = step_wrapper
            .env
            .world
            .get_mut_debugger_backend()
            .vm_runner
            .perform_sc_deploy_lambda(&step_wrapper.step, || {
                contract_instance_wrapped_execution(true, || {
                    f(contract_obj);
                    Ok(())
                });
            });

        let mut response = TxResponse::from_tx_result(tx_result);
        response.new_deployed_address = Some(new_address);
        step_wrapper.step.save_response(response);
        step_wrapper.process_result()
    }
}

impl<'w, From, To, Payment, RH> ScenarioTxWhitebox
    for Tx<ScenarioEnvExec<'w>, From, To, Payment, (), (), RH>
where
    From: TxFromSpecified<ScenarioEnvExec<'w>>,
    To: TxToSpecified<ScenarioEnvExec<'w>>,
    Payment: TxPayment<ScenarioEnvExec<'w>>,
    RH: RHListExec<TxResponse, ScenarioEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn whitebox<ContractObj, F>(
        self,
        contract_obj_builder: fn() -> ContractObj,
        f: F,
    ) -> Self::Returns
    where
        ContractObj: ContractBase<Api = DebugApi> + 'static,
        F: FnOnce(ContractObj),
    {
        self.raw_call(TxFunctionName::WHITEBOX_CALL.as_str())
            .whitebox(contract_obj_builder, f)
    }
}

impl<'w, From, To, Payment, RH> ScenarioTxWhitebox
    for Tx<ScenarioEnvExec<'w>, From, To, Payment, (), FunctionCall<StaticApi>, RH>
where
    From: TxFromSpecified<ScenarioEnvExec<'w>>,
    To: TxToSpecified<ScenarioEnvExec<'w>>,
    Payment: TxPayment<ScenarioEnvExec<'w>>,
    RH: RHListExec<TxResponse, ScenarioEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn whitebox<ContractObj, F>(
        self,
        contract_obj_builder: fn() -> ContractObj,
        f: F,
    ) -> Self::Returns
    where
        ContractObj: ContractBase<Api = DebugApi> + 'static,
        F: FnOnce(ContractObj),
    {
        let contract_obj = contract_obj_builder();

        let mut step_wrapper = self.tx_to_step();

        // no endpoint is called per se, but if it is empty, the VM thinks it is a simple transfer of value
        if step_wrapper.step.tx.function.is_empty() {
            step_wrapper.step.tx.function = TxFunctionName::WHITEBOX_CALL.to_string();
        }

        let tx_result = step_wrapper
            .env
            .world
            .get_mut_debugger_backend()
            .vm_runner
            .perform_sc_call_lambda(&step_wrapper.step, || {
                contract_instance_wrapped_execution(true, || {
                    f(contract_obj);
                    Ok(())
                });
            });

        let response = TxResponse::from_tx_result(tx_result);
        step_wrapper.step.save_response(response);
        step_wrapper.process_result()
    }
}

impl<'w, To, Payment, RH> ScenarioTxWhitebox
    for Tx<ScenarioEnvQuery<'w>, (), To, Payment, (), (), RH>
where
    To: TxToSpecified<ScenarioEnvQuery<'w>>,
    Payment: TxNoPayment<ScenarioEnvQuery<'w>>,
    RH: RHListExec<TxResponse, ScenarioEnvQuery<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn whitebox<ContractObj, F>(
        self,
        contract_obj_builder: fn() -> ContractObj,
        f: F,
    ) -> Self::Returns
    where
        ContractObj: ContractBase<Api = DebugApi> + 'static,
        F: FnOnce(ContractObj),
    {
        self.raw_call(TxFunctionName::WHITEBOX_CALL.as_str())
            .whitebox(contract_obj_builder, f)
    }
}

impl<'w, To, Payment, RH> ScenarioTxWhitebox
    for Tx<ScenarioEnvQuery<'w>, (), To, Payment, (), FunctionCall<StaticApi>, RH>
where
    To: TxToSpecified<ScenarioEnvQuery<'w>>,
    Payment: TxNoPayment<ScenarioEnvQuery<'w>>,
    RH: RHListExec<TxResponse, ScenarioEnvQuery<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn whitebox<ContractObj, F>(
        self,
        contract_obj_builder: fn() -> ContractObj,
        f: F,
    ) -> Self::Returns
    where
        ContractObj: ContractBase<Api = DebugApi> + 'static,
        F: FnOnce(ContractObj),
    {
        let contract_obj = contract_obj_builder();

        let mut step_wrapper = self.tx_to_query_step();

        // no endpoint is called per se, but if it is empty, the VM thinks it is a simple transfer of value
        if step_wrapper.step.tx.function.is_empty() {
            step_wrapper.step.tx.function = TxFunctionName::WHITEBOX_CALL.to_string();
        }

        let tx_result = step_wrapper
            .env
            .world
            .get_mut_debugger_backend()
            .vm_runner
            .perform_sc_query_lambda(&step_wrapper.step, || {
                contract_instance_wrapped_execution(true, || {
                    f(contract_obj);
                    Ok(())
                });
            });

        let response = TxResponse::from_tx_result(tx_result);
        step_wrapper.step.save_response(response);
        step_wrapper.process_result()
    }
}
