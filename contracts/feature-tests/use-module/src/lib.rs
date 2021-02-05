#![no_std]

mod internal_mod_a;
pub use internal_mod_a::*;
mod internal_mod_b;
pub use internal_mod_b::*;

elrond_wasm::imports!();

#[cfg(feature = "elrond-wasm-module-features-default")]
use elrond_wasm_module_features_default::*;
#[cfg(feature = "elrond-wasm-module-features-wasm")]
use elrond_wasm_module_features_wasm::*;

#[cfg(feature = "elrond-wasm-module-pause-default")]
use elrond_wasm_module_pause_default::*;
#[cfg(feature = "elrond-wasm-module-pause-wasm")]
use elrond_wasm_module_pause_wasm::*;

/// Contract that tests that using modules works correctly.
/// Also provides testing for the most common modules:
/// - FeaturesModule
/// - PauseModule
#[elrond_wasm_derive::contract(UseModuleImpl)]
pub trait UseModule {
	#[module(InteralModuleAImpl)]
	fn internal_module_a(&self) -> InteralModuleAImpl<T, BigInt, BigUint>;

	#[module(InteralModuleBImpl)]
	fn internal_module_b(&self) -> InteralModuleBImpl<T, BigInt, BigUint>;

	#[module(FeaturesModuleImpl)]
	fn features_module(&self) -> FeaturesModuleImpl<T, BigInt, BigUint>;

	#[module(PauseModuleImpl)]
	fn pause_module(&self) -> PauseModuleImpl<T, BigInt, BigUint>;

	#[init]
	fn init(&self) {}

	/// Validates that the "featureName" feature is on.
	/// Uses the `feature_guard!` macro.
	#[endpoint(checkFeatureGuard)]
	fn check_feature_guard(&self) -> SCResult<()> {
		feature_guard!(self.features_module(), b"featureName", true);
		Ok(())
	}

	#[endpoint(checkPause)]
	fn check_pause(&self) -> SCResult<bool> {
		Ok(self.pause_module().is_paused())
	}
}
