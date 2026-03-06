use super::{
    ContractVariant, contract_variant_validate::validate_contract_variant,
    proxy_config::ProxyConfig,
};

/// Allowed file names for the SC config.
///
/// The current preferred name is `sc-config.toml`.
///
/// `multicontract.toml` is the legacy name.
/// Was changed because the config file broadened in scope, beyond multi-contract build.
pub const SC_CONFIG_FILE_NAMES: &[&str] = &["sc-config.toml", "multicontract.toml"];

/// An entire project configuration.
///
/// It can contain one or several contract variants.
#[derive(Debug)]
pub struct ScConfig {
    pub contracts: Vec<ContractVariant>,
    pub proxy_configs: Vec<ProxyConfig>,
}

impl ScConfig {
    pub fn get_contract_by_id(&self, contract_id: &str) -> Option<&ContractVariant> {
        self.contracts
            .iter()
            .find(|contract| contract.contract_id == contract_id)
    }

    pub fn get_contract_by_name(&self, contract_name: &str) -> Option<&ContractVariant> {
        self.contracts
            .iter()
            .find(|contract| contract.contract_name == contract_name)
    }

    /// Yields the contract with the given public name.
    pub fn find_contract(&self, contract_name: &str) -> &ContractVariant {
        self.contracts
            .iter()
            .find(|contract| contract.contract_name == contract_name)
            .unwrap_or_else(|| panic!("contract variant {contract_name} not found"))
    }

    pub fn validate_contract_variants(&self) {
        for contract in &self.contracts {
            validate_contract_variant(contract).unwrap_or_else(|err| {
                panic!("Invalid contract variant {}: {err}", contract.contract_name)
            });
        }
    }
}
