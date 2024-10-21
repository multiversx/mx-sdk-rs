use crate::cargo_toml::DependencyRawValue;

use super::sc_config::ContractVariantProfile;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct WasmCargoTomlData {
    pub name: String,
    pub edition: String,
    pub profile: ContractVariantProfile,
    pub framework_dependency: DependencyRawValue,
    pub contract_features: Vec<String>,
    pub contract_default_features: Option<bool>,
}
