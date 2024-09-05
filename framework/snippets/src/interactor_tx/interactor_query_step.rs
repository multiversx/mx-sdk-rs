use multiversx_sc_scenario::{
    multiversx_sc::{tuple_util::NestedTupleFlatten, types::RHListExec},
    scenario::tx_to_step::StepWrapper,
    scenario_model::{ScQueryStep, TxResponse},
};

use super::InteractorEnvQuery;

pub struct InteractorQueryStep<'w, RH>
where
    RH: RHListExec<TxResponse, InteractorEnvQuery<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub(crate) step_wrapper: StepWrapper<InteractorEnvQuery<'w>, ScQueryStep, RH>,
}
