use crate::scenario::tx_to_step::TxToQueryStep;
use crate::ScenarioEnvQuery;
use crate::{
    imports::StaticApi,
    scenario::{run_vm::ScenarioVMRunner, tx_to_step::TxToStep},
    scenario_model::TxResponse,
    ScenarioEnvExec,
};
use multiversx_sc::{
    tuple_util::NestedTupleFlatten,
    types::{
        Code, DeployCall, FunctionCall, RHListExec, Tx, TxCodeValue, TxFromSpecified, TxNoPayment,
        TxPayment, TxToSpecified,
    },
};

pub trait ScenarioTxWhitebox {
    type Returns;
    fn whitebox<ContractObj, F: FnOnce(ContractObj)>(
        self,
        contract_obj: ContractObj,
        f: F,
    ) -> Self::Returns;
}

pub struct ContractObj<A>
where
    A: multiversx_sc::api::VMApi,
{
    _phantom: core::marker::PhantomData<A>,
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

    fn whitebox<ContractObj, F: FnOnce(ContractObj)>(
        self,
        contract_obj: ContractObj,
        f: F,
    ) -> Self::Returns {
        let step_wrapper = self.tx_to_step();

        let mut scenario_vm_runner = ScenarioVMRunner::new();
        let (_new_addr, _tx_result) =
            scenario_vm_runner.perform_sc_deploy_lambda(&step_wrapper.step, || f(contract_obj));

        step_wrapper.process_result()
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

    fn whitebox<ContractObj, F: FnOnce(ContractObj)>(
        self,
        contract_obj: ContractObj,
        f: F,
    ) -> Self::Returns {
        let step_wrapper = self.tx_to_step();
        let _tx_result = self
            .env
            .world
            .get_mut_debugger_backend()
            .vm_runner
            .perform_sc_call_lambda(&step_wrapper.step, || f(contract_obj));

        step_wrapper.process_result()
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

    fn whitebox<ContractObj, F: FnOnce(ContractObj)>(
        self,
        contract_obj: ContractObj,
        f: F,
    ) -> Self::Returns {
        let step_wrapper = self.tx_to_query_step();

        let mut scenario_vm_runner = ScenarioVMRunner::new();
        let _tx_result =
            scenario_vm_runner.perform_sc_query_lambda(&step_wrapper.step, || f(contract_obj));
        step_wrapper.process_result()
    }
}
