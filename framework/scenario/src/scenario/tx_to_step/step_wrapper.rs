use multiversx_sc::{
    tuple_util::NestedTupleFlatten,
    types::{RHListExec, TxEnv},
};

use crate::scenario_model::{ScCallStep, ScDeployStep, ScQueryStep, TxResponse};

pub struct StepWrapper<Env, Step, RH> {
    pub env: Env,
    pub step: Step,
    pub result_handler: RH,
}

impl<Env, Step, RH> StepWrapper<Env, Step, RH>
where
    Env: TxEnv,
    Step: StepWithResponse,
    RH: RHListExec<TxResponse, Env>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub fn process_result(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        let response = self.step.into_response();
        let tuple_result = self.result_handler.list_process_result(&response);
        tuple_result.flatten_unpack()
    }
}

pub trait StepWithResponse {
    fn into_response(self) -> TxResponse;
}

impl StepWithResponse for ScCallStep {
    fn into_response(self) -> TxResponse {
        self.response.expect("SC call step did not return result")
    }
}

impl StepWithResponse for ScDeployStep {
    fn into_response(self) -> TxResponse {
        self.response.expect("SC deploy step did not return result")
    }
}

impl StepWithResponse for ScQueryStep {
    fn into_response(self) -> TxResponse {
        self.response.expect("SC query step did not return result")
    }
}
