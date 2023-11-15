use crate::cargo_toml_contents::CargoTomlContents;

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

impl WasmCargoTomlData {
    pub fn change_package_edition(&mut self, cargo_toml_contents: &CargoTomlContents) {
        self.edition = cargo_toml_contents.package_edition()
    }

    pub fn change_package_name(&mut self, new_package_name: &String) {
        self.name = new_package_name.to_owned()
    }

    pub fn change_adapter_dependencies(&mut self, cargo_toml_contents: &CargoTomlContents) {
        let (version, path) = cargo_toml_contents.get_adapter_dependencies();
        self.framework_version = version;
        self.framework_path = path;
    }

    pub fn change_profile(&mut self, contract_profile: &ContractVariantProfile) {
        self.profile = contract_profile.to_owned()
    }

    pub fn change_contract_features(&mut self, features: &Vec<String>) {
        self.contract_features = features.to_owned()
    }

    pub fn from(
        name: String,
        edition: String,
        profile: ContractVariantProfile,
        framework_version: String,
        framework_path: Option<String>,
        contract_features: Vec<String>,
    ) -> WasmCargoTomlData {
        WasmCargoTomlData {
            name,
            edition,
            profile,
            framework_version,
            framework_path,
            contract_features,
        }
    }
}
