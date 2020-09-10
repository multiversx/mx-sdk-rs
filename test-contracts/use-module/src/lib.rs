
#![no_std]

imports!();

use elrond_wasm_module_features::*;
use elrond_wasm_module_pause::*;

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
