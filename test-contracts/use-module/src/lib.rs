
#![no_std]

imports!();

use elrond_wasm_module_features::*;

#[elrond_wasm_derive::contract(UseModuleExampleImpl)]
pub trait UseModuleExample {

    #[module(FeaturesModuleImpl)]
    fn features_module(&self) -> FeaturesModuleImpl<T, BigInt, BigUint>;

    #[init]
    fn init(&self) {
    }

    #[endpoint(checkFeatureGuard)]
    fn check_feature_guard(&self) -> SCResult<()> {
        feature_guard!(self.features_module(), b"featureName", true);
        Ok(())
    }
}
