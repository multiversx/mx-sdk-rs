
#![no_std]

imports!();

#[cfg(feature = "elrond-wasm-module-features-default")]
use elrond_wasm_module_features_default::*;
#[cfg(feature = "elrond-wasm-module-features-wasm")]
use elrond_wasm_module_features_wasm::*;

#[cfg(feature = "elrond-wasm-module-pause-default")]
use elrond_wasm_module_pause_default::*;
#[cfg(feature = "elrond-wasm-module-pause-wasm")]
use elrond_wasm_module_pause_wasm::*;

#[elrond_wasm_derive::contract(UseModuleExampleImpl)]
pub trait UseModuleExample {

    #[module(FeaturesModuleImpl)]
    fn features_module(&self) -> FeaturesModuleImpl<T, BigInt, BigUint>;

    #[module(PauseModuleImpl)]
    fn pause_module(&self) -> PauseModuleImpl<T, BigInt, BigUint>;

    #[init]
    fn init(&self) {
    }

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
