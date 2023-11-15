use super::sc_config::ContractVariantProfile;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct WasmCargoTomlData {
    pub name: String,
    pub edition: String,
    pub profile: ContractVariantProfile,
    pub framework_version: String,
    pub framework_path: Option<String>,
    pub contract_features: Vec<String>,
}
