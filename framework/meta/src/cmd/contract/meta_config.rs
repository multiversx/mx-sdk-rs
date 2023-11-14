use std::fs;

use multiversx_sc::abi::ContractAbi;

use crate::{cli_args::BuildArgs, tools::check_tools_installed, CargoTomlContents};

use super::{
    sc_config::{ContractVariant, ScConfig},
    wasm_cargo_toml_data::WasmCargoTomlData,
};

const OUTPUT_RELATIVE_PATH: &str = "../output";
const SNIPPETS_RELATIVE_PATH: &str = "../interact-rs";
const WASM_LIB_PATH: &str = "../wasm/src/lib.rs";
const WASM_NO_MANAGED_EI: &str = "wasm-no-managed-ei";
const WASM_NO_MANAGED_EI_LIB_PATH: &str = "../wasm-no-managed-ei/src/lib.rs";

pub struct MetaConfig {
    pub load_abi_git_version: bool,
    pub output_dir: String,
    pub snippets_dir: String,
    pub original_contract_abi: ContractAbi,
    pub sc_config: ScConfig,
}

impl MetaConfig {
    pub fn create(original_contract_abi: ContractAbi, load_abi_git_version: bool) -> MetaConfig {
        let sc_config = ScConfig::load_from_crate_or_default("..", &original_contract_abi);

        MetaConfig {
            load_abi_git_version,
            output_dir: OUTPUT_RELATIVE_PATH.to_string(),
            snippets_dir: SNIPPETS_RELATIVE_PATH.to_string(),
            original_contract_abi,
            sc_config,
        }
    }

    /// Generates all code for the wasm crate(s).
    pub fn generate_wasm_crates(&mut self) {
        self.remove_unexpected_wasm_crates();
        self.create_wasm_crate_dirs();
        self.generate_cargo_toml_for_all_wasm_crates();
        self.generate_wasm_src_lib();
        copy_to_wasm_unmanaged_ei();
    }

    fn create_wasm_crate_dirs(&self) {
        for contract_variant in &self.sc_config.contracts {
            contract_variant.create_wasm_crate_dir();
        }
    }

    /// Cargo.toml files for all wasm crates are generated from the main contract Cargo.toml,
    /// by changing the package name.
    pub fn generate_cargo_toml_for_all_wasm_crates(&mut self) {
        let main_cargo_toml_contents = CargoTomlContents::load_from_file("../Cargo.toml");
        let mut cargo_toml_data = WasmCargoTomlData::default();
        cargo_toml_data.change_package_edition(&main_cargo_toml_contents);
        cargo_toml_data.change_adapter_dependencies(&main_cargo_toml_contents);

        for contract in self.sc_config.contracts.iter() {
            cargo_toml_data.change_package_name(&contract.wasm_crate_name);
            cargo_toml_data.change_profile(&contract.settings.contract_variant_profile);
            generate_cargo_toml(&cargo_toml_data, contract).save_to_file(contract.cargo_toml_path());
        }
    }
}

fn generate_cargo_toml(cargo_toml_data: &WasmCargoTomlData, contract: &ContractVariant) -> CargoTomlContents {
    let mut new_cargo = CargoTomlContents::new();

    new_cargo.add_package_info(
        &cargo_toml_data.name,
        "0.0.0".to_string(),
        cargo_toml_data.edition.clone(),
        false,
    );

    //add lib
    new_cargo.add_lib();

    //add profile
    new_cargo.add_contract_variant_profile(&cargo_toml_data.profile);

    //add deps
    new_cargo.add_deps(&cargo_toml_data.name, &cargo_toml_data.framework_version, &cargo_toml_data.framework_path);

    //check features
    if !contract.settings.features.is_empty() {
        new_cargo.change_features_for_parent_crate_dep(contract.settings.features.as_slice());
    }

    //insert default workspace
    new_cargo.add_workspace(&["."]);

    new_cargo
}

impl MetaConfig {
    fn generate_wasm_src_lib(&self) {
        for contract_variant in &self.sc_config.contracts {
            contract_variant.generate_wasm_src_lib_file();
        }
    }

    pub fn build(&mut self, mut build_args: BuildArgs) {
        check_tools_installed(&mut build_args);

        for contract_variant in &self.sc_config.contracts {
            contract_variant.build_contract(&build_args, self.output_dir.as_str());
        }
    }

    /// Cleans the wasm crates and all other outputs.
    pub fn clean(&self) {
        self.clean_contract_crates();
        self.remove_output_dir();
    }

    fn clean_contract_crates(&self) {
        for contract_variant in &self.sc_config.contracts {
            contract_variant.cargo_clean();
        }
    }

    /// Updates the Cargo.lock on all wasm crates.
    pub fn update(&self) {
        for contract_variant in &self.sc_config.contracts {
            contract_variant.cargo_update();
        }
    }

    fn remove_output_dir(&self) {
        fs::remove_dir_all(&self.output_dir).expect("failed to remove output directory");
    }

    fn is_expected_crate(&self, dir_name: &str) -> bool {
        if !dir_name.starts_with("wasm-") {
            return true;
        }

        if dir_name == WASM_NO_MANAGED_EI {
            return true;
        }

        self.sc_config
            .secondary_contracts()
            .any(|contract| contract.wasm_crate_dir_name().as_str() == dir_name)
    }

    fn remove_unexpected_wasm_crates(&self) {
        let list_iter = fs::read_dir("..").expect("error listing contract directory");
        for path_result in list_iter {
            let path = path_result.expect("error processing file name in contract directory");
            if path
                .metadata()
                .expect("error retrieving file metadata")
                .is_dir()
            {
                let file_name = path.file_name();
                let dir_name = file_name.to_str().expect("error processing dir name");
                if !self.is_expected_crate(dir_name) {
                    println!("Removing crate {dir_name}");
                    fs::remove_dir_all(path.path()).unwrap_or_else(|_| {
                        panic!("failed to remove unexpected directory {dir_name}")
                    });
                }
            }
        }
    }
}

/// This one is useful for some of the special unmanaged EI tests in the framework.
/// Will do nothing for regular contracts.
fn copy_to_wasm_unmanaged_ei() {
    if std::path::Path::new(WASM_NO_MANAGED_EI_LIB_PATH).exists() {
        fs::copy(WASM_LIB_PATH, WASM_NO_MANAGED_EI_LIB_PATH).unwrap();
    }
}
