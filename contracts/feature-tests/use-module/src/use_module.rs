#![no_std]

mod contract_base_full_path_mod;
mod contract_base_mod;
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
pub mod token_merge_mod_impl;

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
    ContractBase
    + contract_base_full_path_mod::ContractBaseFullPathTestModule
    + contract_base_mod::ContractBaseTestModule
    + internal_mod_a::InternalModuleA
    + internal_mod_b::InternalModuleB
    + internal_mod_c::InternalModuleC
    + internal_mod_init::InternalModuleInit
    + only_owner_mod::OnlyOwnerTestModule
    + only_owner_derived_mod::OnlyOwnerDerivedTestModule
    + only_admin_mod::OnlyAdminTestModule
    + only_admin_derived_mod::OnlyAdminDerivedTestModule
    + ongoing_operation_mod_example::OngoingOperationModExample
    + token_merge_mod_impl::TokenMergeModImpl
    + mx_sc_modules::claim_developer_rewards::ClaimDeveloperRewardsModule
    + mx_sc_modules::dns::DnsModule
    + mx_sc_modules::esdt::EsdtModule
    + mx_sc_modules::features::FeaturesModule
    + mx_sc_modules::governance::GovernanceModule
    + mx_sc_modules::governance::governance_configurable::GovernanceConfigurablePropertiesModule
    + mx_sc_modules::governance::governance_events::GovernanceEventsModule
    + mx_sc_modules::pause::PauseModule
    + mx_sc_modules::staking::StakingModule
    + mx_sc_modules::token_merge::TokenMergeModule
    + mx_sc_modules::token_merge::merged_token_setup::MergedTokenSetupModule
    + mx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + mx_sc_modules::only_admin::OnlyAdminModule
    + mx_sc_modules::ongoing_operation::OngoingOperationModule
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
