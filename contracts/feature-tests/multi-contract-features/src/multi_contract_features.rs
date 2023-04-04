#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait MultiContractFeatures {
    #[init]
    fn default_init(&self, sample_value: BigUint) {
        self.sample_value().set(sample_value);
    }

    #[init]
    #[label("alt-impl")]
    fn alternative_init(&self) -> &'static str {
        "alternative init"
    }

    #[view]
    #[storage_mapper("sample-value")]
    fn sample_value(&self) -> SingleValueMapper<BigUint>;

    #[view(sample_value)]
    #[label("alt-impl")]
    fn alternative_sample_value(&self) -> &'static str {
        "alternative message instead of sample value"
    }

    #[view]
    #[label("mcs-external-view")]
    fn external_pure(&self) -> i32 {
        1
    }

    #[view]
    #[label("mcs-external-view")]
    fn sample_value_external_get(&self) -> BigUint {
        self.sample_value().get()
    }

    /// This is not really a view.
    /// Designed to check what happens if we try to write to storage from an external view.
    #[endpoint]
    #[label("mcs-external-view")]
    fn sample_value_external_set(&self, sample_value: BigUint) {
        self.sample_value().set(sample_value);
    }

    #[view]
    fn example_feature_message(&self) -> &'static str {
        example_feature_message()
    }
}

#[cfg(feature = "example_feature")]
fn example_feature_message() -> &'static str {
    "example-feature on"
}

#[cfg(not(feature = "example_feature"))]
fn example_feature_message() -> &'static str {
    "example-feature off"
}
