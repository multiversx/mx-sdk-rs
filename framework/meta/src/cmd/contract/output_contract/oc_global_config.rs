use super::{oc_validate::validate_output_contract, OutputContract};

/// An entire project configuration.
///
/// It can contain one or several output contracts.
#[derive(Debug)]
pub struct OutputContractGlobalConfig {
    pub default_contract_config_name: String,
    pub contracts: Vec<OutputContract>,
}

impl OutputContractGlobalConfig {
    pub fn main_contract(&self) -> &OutputContract {
        self.contracts
            .iter()
            .find(|contract| contract.main)
            .unwrap_or_else(|| {
                panic!(
                    "Could not find default contract '{}' among the output contracts.",
                    self.default_contract_config_name
                )
            })
    }

    pub fn main_contract_mut(&mut self) -> &mut OutputContract {
        self.contracts
            .iter_mut()
            .find(|contract| contract.main)
            .unwrap_or_else(|| {
                panic!(
                    "Could not find default contract '{}' among the output contracts.",
                    self.default_contract_config_name
                )
            })
    }

    pub fn secondary_contracts(&self) -> impl Iterator<Item = &OutputContract> {
        self.contracts.iter().filter(move |contract| !contract.main)
    }

    pub fn secondary_contracts_mut(&mut self) -> impl Iterator<Item = &mut OutputContract> {
        self.contracts
            .iter_mut()
            .filter(move |contract| !contract.main)
    }

    pub fn get_contract_by_id(&self, contract_id: String) -> Option<&OutputContract> {
        self.contracts
            .iter()
            .find(|contract| contract.contract_id == contract_id)
    }

    pub fn get_contract_by_name(&self, contract_name: String) -> Option<&OutputContract> {
        self.contracts
            .iter()
            .find(|contract| contract.contract_name == contract_name)
    }

    /// Yields the contract with the given public name.
    pub fn find_contract(&self, contract_name: &str) -> &OutputContract {
        self.contracts
            .iter()
            .find(|contract| contract.contract_name == contract_name)
            .unwrap_or_else(|| panic!("output contract {contract_name} not found"))
    }

    pub fn validate_output_contracts(&self) {
        for contract in &self.contracts {
            validate_output_contract(contract).unwrap_or_else(|err| {
                panic!("Invalid output contract {}: {err}", contract.contract_name)
            });
        }
    }
}
