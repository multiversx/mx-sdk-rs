use multiversx_sc_scenario::{
    multiversx_sc::{tuple_util::NestedTupleFlatten, types::RHListExec},
    scenario::tx_to_step::StepWrapper,
    scenario_model::TxResponse,
};

use super::InteractorExecEnv;

pub struct InteractorExecStep<'w, Step, RH>
where
    RH: RHListExec<TxResponse, InteractorExecEnv<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub(crate) step_wrapper: StepWrapper<InteractorExecEnv<'w>, Step, RH>,
}
