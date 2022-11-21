use std::{fs, process::Command};

use elrond_wasm::abi::{ContractAbi, EndpointLocationAbi};

use crate::meta::output_contract::WASM_OPT_NAME;

use super::{
    meta_build_args::BuildArgs,
    output_contract::{OutputContract, OutputContractConfig},
};

const OUTPUT_RELATIVE_PATH: &str = "../output";
const SNIPPETS_RELATIVE_PATH: &str = "../interact-rs";
const MULTI_CONTRACT_CONFIG_RELATIVE_PATH: &str = "../multicontract.toml";
const WASM_LIB_PATH: &str = "../wasm/src/lib.rs";
const WASM_LIB_PATH_NO_MANAGED_EI: &str = "../wasm-no-managed-ei/src/lib.rs";

pub struct ContractMetadata {
    pub location: EndpointLocationAbi,
    pub wasm_crate_name: String,
    pub wasm_crate_path: String,
    pub output_base_name: String,
    pub original_abi: ContractAbi,
}

pub struct MetaConfig {
    pub build_args: BuildArgs,
    pub output_dir: String,
    pub snippets_dir: String,
    pub main_contract: Option<ContractMetadata>,
    pub view_contract: Option<ContractMetadata>,
    pub output_contracts: OutputContractConfig,
}

impl MetaConfig {
    pub fn create(original_contract_abi: &ContractAbi, build_args: BuildArgs) -> MetaConfig {
        let output_contracts = OutputContractConfig::load_from_file_or_default(
            MULTI_CONTRACT_CONFIG_RELATIVE_PATH,
            original_contract_abi,
        );

        MetaConfig {
            build_args,
            output_dir: OUTPUT_RELATIVE_PATH.to_string(),
            snippets_dir: SNIPPETS_RELATIVE_PATH.to_string(),
            main_contract: None,
            view_contract: None,
            output_contracts,
        }
    }

    pub fn write_wasm_src_lib(&self) {
        for output_contract in &self.output_contracts.contracts {
            output_contract.write_wasm_src_lib();
        }
    }

    pub fn build_wasm(&mut self) {
        if self.build_args.wasm_opt && !is_wasm_opt_installed() {
            println!("Warning: {} not installed", WASM_OPT_NAME);
            self.build_args.wasm_opt = false;
        }

        for output_contract in &mut self.output_contracts.contracts {
            output_contract.build_contract(&self.build_args, self.output_dir.as_str());
        }
    }

    pub fn create_wasm_view_cargo_toml(&self) {
        let main_contract = self.output_contracts.main_contract();
        for secondary_contract in self.output_contracts.secondary_contracts() {
            fs::create_dir_all(&secondary_contract.wasm_crate_path()).unwrap();
            create_cargo_toml_from_source(main_contract, secondary_contract);
        }
    }

    pub fn clean_wasm(&self) {
        for output_contract in &self.output_contracts.contracts {
            output_contract.cargo_clean();
        }

        fs::remove_dir_all(&self.output_dir).expect("failed to remove output directory");
    }
}

fn create_cargo_toml_from_source(source: &OutputContract, dest: &OutputContract) {
    fs::copy(source.cargo_toml_path(), dest.cargo_toml_path()).unwrap();
}

fn is_wasm_opt_installed() -> bool {
    Command::new(WASM_OPT_NAME)
        .args(["--version"])
        .output()
        .is_ok()
}

/// This one is useful for some of the special unmanaged EI tests in the framework.
/// Will do nothing for regular contracts.
pub fn copy_to_wasm_unmanaged_ei() {
    if std::path::Path::new(WASM_LIB_PATH_NO_MANAGED_EI).exists() {
        fs::copy(WASM_LIB_PATH, WASM_LIB_PATH_NO_MANAGED_EI).unwrap();
    }
}
