#![no_std]

mod internal_mod_a;
mod internal_mod_b;
mod internal_mod_c;
mod internal_mod_d;
mod internal_mod_init;
mod ongoing_operation_mod_example;
mod only_admin_derived_mod;
mod only_admin_mod;
mod only_owner_derived_mod;
mod only_owner_mod;

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
    + only_owner_mod::OnlyOwnerTestModule
    + only_owner_derived_mod::OnlyOwnerDerivedTestModule
    + only_admin_mod::OnlyAdminTestModule
    + only_admin_derived_mod::OnlyAdminDerivedTestModule
    + ongoing_operation_mod_example::OngoingOperationModExample
    + elrond_wasm_modules::claim_developer_rewards::ClaimDeveloperRewardsModule
    + elrond_wasm_modules::dns::DnsModule
    + elrond_wasm_modules::esdt::EsdtModule
    + elrond_wasm_modules::features::FeaturesModule
    + elrond_wasm_modules::governance::GovernanceModule
    + elrond_wasm_modules::governance::governance_configurable::GovernanceConfigurablePropertiesModule
    + elrond_wasm_modules::governance::governance_events::GovernanceEventsModule
    + elrond_wasm_modules::pause::PauseModule
    + elrond_wasm_modules::staking::StakingModule
    + elrond_wasm_modules::token_merge::TokenMergeModule
    + elrond_wasm_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + elrond_wasm_modules::only_admin::OnlyAdminModule
    + elrond_wasm_modules::ongoing_operation::OngoingOperationModule
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
