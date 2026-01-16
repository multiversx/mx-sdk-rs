use multiversx_chain_scenario_format::serde_raw::ValueSubTree;
use multiversx_sc_scenario::{
    api::StaticApi,
    imports::{AnnotatedValue, ManagedBuffer, TxEnv, TxGas, TxGasValue},
    scenario_model::U64Value,
};
use multiversx_sdk::gateway::GatewayAsyncService;

use crate::InteractorEnvExec;

const SIMULATE_GAS_ANNOTATION: &str = "simulate";

/// Indicates that a simulation should be run before launching the actual transaction.
pub struct SimulateGas;

impl<'w, GatewayProxy> AnnotatedValue<InteractorEnvExec<'w, GatewayProxy>, u64> for SimulateGas
where
    GatewayProxy: GatewayAsyncService + 'w,
{
    fn annotation(&self, _env: &InteractorEnvExec<'w, GatewayProxy>) -> ManagedBuffer<StaticApi> {
        SIMULATE_GAS_ANNOTATION.into()
    }

    fn to_value(&self, _env: &InteractorEnvExec<'w, GatewayProxy>) -> u64 {
        0
    }
}

impl<'w, GatewayProxy> TxGasValue<InteractorEnvExec<'w, GatewayProxy>> for SimulateGas where
    GatewayProxy: GatewayAsyncService + 'w
{
}

impl SimulateGas {
    pub fn is_mandos_simulate_gas_marker(gas_limit: &U64Value) -> bool {
        if gas_limit.value != 0 {
            return false;
        }

        if let ValueSubTree::Str(s) = &gas_limit.original {
            s == SIMULATE_GAS_ANNOTATION
        } else {
            false
        }
    }

    /// Add 10% to protect against noise.
    pub fn adjust_simulated_gas(gas: u64) -> u64 {
        gas + gas / 10
    }
}
