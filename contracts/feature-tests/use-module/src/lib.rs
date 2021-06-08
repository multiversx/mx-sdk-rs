#![no_std]

mod internal_mod_a;
mod internal_mod_b;
mod internal_mod_c;

elrond_wasm::imports!();

use elrond_wasm_module_features::feature_guard;

/// Contract that tests that using modules works correctly.
/// Also provides testing for the most common modules:
/// - DnsModule
/// - FeaturesModule
/// - PauseModule
#[elrond_wasm_derive::contract]
pub trait UseModule:
	internal_mod_a::InternalModuleA
	+ internal_mod_b::InternalModuleB
	+ internal_mod_c::InternalModuleC
	+ elrond_wasm_module_dns::DnsModule
	+ elrond_wasm_module_esdt::EsdtModule
	+ elrond_wasm_module_features::FeaturesModule
	+ elrond_wasm_module_pause::PauseModule
{
	#[init]
	fn init(&self) {}

	/// Validates that the "featureName" feature is on.
	/// Uses the `feature_guard!` macro.
	#[endpoint(checkFeatureGuard)]
	fn check_feature_guard(&self) -> SCResult<()> {
		feature_guard!(self, b"featureName", true);
		Ok(())
	}

	#[endpoint(checkPause)]
	fn check_pause(&self) -> SCResult<bool> {
		Ok(self.is_paused())
	}
}
