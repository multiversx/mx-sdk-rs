use std::path::Path;

use multiversx_sc_meta_lib::cargo_toml::CargoTomlContents;

use super::scenario_loader::{self, ScenarioFile};

/// Holds the inputs needed to generate blackbox tests for a contract:
/// the crate name and the loaded scenario files.
pub struct ContractScenarios {
    pub crate_name: String,
    pub scenario_files: Vec<ScenarioFile>,
}

impl ContractScenarios {
    /// Loads scenario files and crate name from the contract directory.
    /// Returns `None` if there is no `scenarios/` folder or it contains no `.scen.json` files.
    pub fn load(contract_path: &Path) -> Option<Self> {
        let scenarios_dir = contract_path.join("scenarios");
        if !scenarios_dir.exists() {
            return None;
        }

        let mut scenario_files = scenario_loader::load_scenario_files(&scenarios_dir);
        if scenario_files.is_empty() {
            return None;
        }

        // ensures that the order is deterministic and doesn't depend on the filesystem
        scenario_files.sort_by(|a, b| a.file_name.cmp(&b.file_name));

        let cargo_toml_path = contract_path.join("Cargo.toml");
        let cargo_toml = CargoTomlContents::load_from_file(&cargo_toml_path);
        let crate_name = cargo_toml.package_name().replace('-', "_");

        Some(ContractScenarios {
            crate_name,
            scenario_files,
        })
    }
}
