use std::fs;

use multiversx_sc::abi::ContractAbi;

use crate::{cli_args::BuildArgs, tools::post_build::check_tools_installed, CargoTomlContents};

use super::output_contract::{OutputContract, OutputContractGlobalConfig};

const OUTPUT_RELATIVE_PATH: &str = "../output";
const SNIPPETS_RELATIVE_PATH: &str = "../interact-rs";
const MULTI_CONTRACT_CONFIG_RELATIVE_PATH: &str = "../multicontract.toml";
const WASM_LIB_PATH: &str = "../wasm/src/lib.rs";
const WASM_NO_MANAGED_EI: &str = "wasm-no-managed-ei";
const WASM_NO_MANAGED_EI_LIB_PATH: &str = "../wasm-no-managed-ei/src/lib.rs";

pub struct MetaConfig {
    pub load_abi_git_version: bool,
    pub output_dir: String,
    pub snippets_dir: String,
    pub original_contract_abi: ContractAbi,
    pub output_contracts: OutputContractGlobalConfig,
}

impl MetaConfig {
    pub fn create(original_contract_abi: ContractAbi, load_abi_git_version: bool) -> MetaConfig {
        let output_contracts = OutputContractGlobalConfig::load_from_file_or_default(
            MULTI_CONTRACT_CONFIG_RELATIVE_PATH,
            &original_contract_abi,
        );

        MetaConfig {
            load_abi_git_version,
            output_dir: OUTPUT_RELATIVE_PATH.to_string(),
            snippets_dir: SNIPPETS_RELATIVE_PATH.to_string(),
            original_contract_abi,
            output_contracts,
        }
    }

    /// Generates all code for the wasm crate(s).
    pub fn generate_wasm_crates(&mut self) {
        self.remove_unexpected_wasm_crates();
        self.create_wasm_crate_dirs();
        self.generate_cargo_toml_for_secondary_contracts();
        self.generate_wasm_src_lib();
        copy_to_wasm_unmanaged_ei();
    }

    fn create_wasm_crate_dirs(&self) {
        for output_contract in &self.output_contracts.contracts {
            output_contract.create_wasm_crate_dir();
        }
    }

    /// Cargo.toml files for secondary contracts are generated from the main contract Cargo.toml,
    /// by changing the package name.
    pub fn generate_cargo_toml_for_secondary_contracts(&mut self) {
        let main_contract = self.output_contracts.main_contract_mut();

        let main_cargo_toml_contents =
            CargoTomlContents::load_from_file(main_contract.cargo_toml_path());
        main_contract.wasm_crate_name = main_cargo_toml_contents.package_name();

        for secondary_contract in self.output_contracts.secondary_contracts() {
            secondary_contract_cargo_toml(secondary_contract, &main_cargo_toml_contents)
                .save_to_file(secondary_contract.cargo_toml_path());
        }
    }
}

fn secondary_contract_cargo_toml(
    secondary_contract: &OutputContract,
    main_cargo_toml_contents: &CargoTomlContents,
) -> CargoTomlContents {
    let mut cargo_toml_contents = main_cargo_toml_contents.clone();
    cargo_toml_contents.change_package_name(secondary_contract.wasm_crate_name.clone());
    if !secondary_contract.settings.features.is_empty() {
        cargo_toml_contents
            .change_features_for_parent_crate_dep(secondary_contract.settings.features.as_slice());
    }
    cargo_toml_contents
}

impl MetaConfig {
    fn generate_wasm_src_lib(&self) {
        for output_contract in &self.output_contracts.contracts {
            output_contract.generate_wasm_src_lib_file();
        }
    }

    pub fn build(&mut self, mut build_args: BuildArgs) {
        check_tools_installed(&mut build_args);

        for output_contract in &self.output_contracts.contracts {
            output_contract.build_contract(&build_args, self.output_dir.as_str());
        }
    }

    /// Cleans the wasm crates and all other outputs.
    pub fn clean(&self) {
        self.clean_contract_crates();
        self.remove_output_dir();
    }

    fn clean_contract_crates(&self) {
        for output_contract in &self.output_contracts.contracts {
            output_contract.cargo_clean();
        }
    }

    /// Updates the Cargo.lock on all wasm crates.
    pub fn update(&self) {
        for output_contract in &self.output_contracts.contracts {
            output_contract.cargo_update();
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

        self.output_contracts
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
