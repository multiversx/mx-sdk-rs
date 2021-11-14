#![no_std]

mod internal_mod_a;
mod internal_mod_b;
mod internal_mod_c;
mod internal_mod_d;
mod internal_mod_init;

elrond_wasm::imports!();

/// Contract that tests that using modules works correctly.
/// Also provides testing for the most common modules:
/// - DnsModule
/// - FeaturesModule
/// - EsdtModule
/// - GovernanceModule
/// - PauseModule
#[elrond_wasm::contract]
pub trait UseModule:
    internal_mod_a::InternalModuleA
    + internal_mod_b::InternalModuleB
    + internal_mod_c::InternalModuleC
    + internal_mod_init::InternalModuleInit
    + elrond_wasm_module_dns::DnsModule
    + elrond_wasm_module_esdt::EsdtModule
    + elrond_wasm_module_features::FeaturesModule
    + elrond_wasm_module_governance::GovernanceModule
    + elrond_wasm_module_governance::governance_configurable::GovernanceConfigurablePropertiesModule
    + elrond_wasm_module_pause::PauseModule
{
    /// Validates that the "featureName" feature is on.
    /// Uses the `feature_guard!` macro.
    #[endpoint(checkFeatureGuard)]
    fn check_feature_guard(&self) {
        self.check_feature_on(b"featureName", true);
    }

    #[endpoint(checkPause)]
    fn check_pause(&self) -> SCResult<bool> {
        Ok(self.is_paused())
    }
}
