#![no_std]

mod internal_mod_a;
mod internal_mod_b;
mod internal_mod_c;

elrond_wasm::imports!();

#[cfg(feature = "elrond-wasm-module-features-default")]
pub use elrond_wasm_module_features_default as features;
#[cfg(feature = "elrond-wasm-module-features-wasm")]
pub use elrond_wasm_module_features_wasm as features;

#[cfg(feature = "elrond-wasm-module-pause-default")]
pub use elrond_wasm_module_pause_default as pause;
#[cfg(feature = "elrond-wasm-module-pause-wasm")]
pub use elrond_wasm_module_pause_wasm as pause;

use features::feature_guard;

/// Contract that tests that using modules works correctly.
/// Also provides testing for the most common modules:
/// - FeaturesModule
/// - PauseModule
#[elrond_wasm_derive::contract(UseModuleImpl)]
pub trait UseModule:
	internal_mod_a::InternalModuleA
	+ internal_mod_b::InternalModuleB
	+ internal_mod_c::InternalModuleC
	+ features::FeaturesModule
	+ pause::PauseModule
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
