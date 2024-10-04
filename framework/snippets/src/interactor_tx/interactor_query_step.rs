use multiversx_sc_scenario::{
    multiversx_sc::{tuple_util::NestedTupleFlatten, types::RHListExec},
    scenario::tx_to_step::StepWrapper,
    scenario_model::{ScQueryStep, TxResponse},
};
use multiversx_sdk::gateway::GatewayAsyncService;

use super::InteractorEnvQuery;

pub struct InteractorQueryStep<'w, GatewayProxy, RH>
where
    GatewayProxy: GatewayAsyncService,
    RH: RHListExec<TxResponse, InteractorEnvQuery<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub(crate) step_wrapper: StepWrapper<InteractorEnvQuery<'w, GatewayProxy>, ScQueryStep, RH>,
}
