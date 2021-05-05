#![no_std]

pub mod internal_mod_a;
pub mod internal_mod_b;
pub mod internal_mod_c;

elrond_wasm::imports!();

// #[cfg(feature = "elrond-wasm-module-features-default")]
// pub use elrond_wasm_module_features_default as features;
// #[cfg(feature = "elrond-wasm-module-features-wasm")]
// pub use elrond_wasm_module_features_wasm as features;

// #[cfg(feature = "elrond-wasm-module-pause-default")]
// pub use elrond_wasm_module_pause_default as pause;
// #[cfg(feature = "elrond-wasm-module-pause-wasm")]
// pub use elrond_wasm_module_pause_wasm as pause;

// use features::*;
// use pause::*;

/// Contract that tests that using modules works correctly.
/// Also provides testing for the most common modules:
/// - FeaturesModule
/// - PauseModule
#[elrond_wasm_derive::contract(UseModuleImpl)]
pub trait UseModule:
	internal_mod_a::InternalModuleA + internal_mod_b::InternalModuleB + internal_mod_c::InternalModuleC
{
	// #[module(InteralModuleAImpl)]
	// fn internal_module_a(
	// 	&self,
	// ) -> internal_mod_a::implementation::InteralModuleA<T, BigInt, BigUint>;

	// #[module(InteralModuleBImpl)]
	// fn internal_module_b(
	// 	&self,
	// ) -> internal_mod_b::implementation::InteralModuleB<T, BigInt, BigUint>;

	// #[module(FeaturesModuleImpl)]
	// fn features_module(&self) -> features::implementation::FeaturesModule<T, BigInt, BigUint>;

	// #[module(PauseModuleImpl)]
	// fn pause_module(&self) -> pause::implementation::PauseModule<T, BigInt, BigUint>;

	#[init]
	fn init(&self) {}

	// /// Validates that the "featureName" feature is on.
	// /// Uses the `feature_guard!` macro.
	// #[endpoint(checkFeatureGuard)]
	// fn check_feature_guard(&self) -> SCResult<()> {
	// 	feature_guard!(self.features_module(), b"featureName", true);
	// 	Ok(())
	// }

	// #[endpoint(checkPause)]
	// fn check_pause(&self) -> SCResult<bool> {
	// 	Ok(self.pause_module().is_paused())
	// }
}
