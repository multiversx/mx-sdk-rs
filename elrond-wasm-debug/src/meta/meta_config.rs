use std::{fs, process::Command};

use elrond_wasm::abi::ContractAbi;

use crate::meta::output_contract::WASM_OPT_NAME;

use super::{meta_build_args::BuildArgs, output_contract::OutputContractConfig};

const OUTPUT_RELATIVE_PATH: &str = "../output";
const SNIPPETS_RELATIVE_PATH: &str = "../interact-rs";
const MULTI_CONTRACT_CONFIG_RELATIVE_PATH: &str = "../multicontract.toml";
const WASM_LIB_PATH: &str = "../wasm/src/lib.rs";
const WASM_NO_MANAGED_EI: &str = "wasm-no-managed-ei";
const WASM_NO_MANAGED_EI_LIB_PATH: &str = "../wasm-no-managed-ei/src/lib.rs";

pub struct MetaConfig {
    pub build_args: BuildArgs,
    pub output_dir: String,
    pub snippets_dir: String,
    pub original_contract_abi: ContractAbi,
    pub output_contracts: OutputContractConfig,
}

impl MetaConfig {
    pub fn create(original_contract_abi: ContractAbi, build_args: BuildArgs) -> MetaConfig {
        let output_contracts = OutputContractConfig::load_from_file_or_default(
            MULTI_CONTRACT_CONFIG_RELATIVE_PATH,
            &original_contract_abi,
        );

        MetaConfig {
            build_args,
            output_dir: OUTPUT_RELATIVE_PATH.to_string(),
            snippets_dir: SNIPPETS_RELATIVE_PATH.to_string(),
            original_contract_abi,
            output_contracts,
        }
    }

    /// Generates all code for the wasm crate(s).
    pub fn generate_wasm_crates(&self) {
        self.remove_unexpected_wasm_crates();
        self.copy_secondary_contract_cargo_toml();
        self.write_wasm_src_lib();
        copy_to_wasm_unmanaged_ei();
    }

    fn write_wasm_src_lib(&self) {
        for output_contract in &self.output_contracts.contracts {
            output_contract.write_wasm_src_lib_file();
        }
    }

    pub fn copy_secondary_contract_cargo_toml(&self) {
        let main_contract = self.output_contracts.main_contract();
        for secondary_contract in self.output_contracts.secondary_contracts() {
            fs::create_dir_all(&secondary_contract.wasm_crate_path()).unwrap();
            fs::copy(
                main_contract.cargo_toml_path(),
                secondary_contract.cargo_toml_path(),
            )
            .unwrap();
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

    /// Cleans the wasm crates and all other outputs.
    pub fn clean_wasm(&self) {
        self.clean_contract_crates();
        self.remove_output_dir();
    }

    fn clean_contract_crates(&self) {
        for output_contract in &self.output_contracts.contracts {
            output_contract.cargo_clean();
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
                    println!("Removing crate {}", dir_name);
                    fs::remove_dir_all(path.path()).unwrap_or_else(|_| {
                        panic!("failed to remove unexpected directory {}", dir_name)
                    });
                }
            }
        }
    }
}

fn is_wasm_opt_installed() -> bool {
    Command::new(WASM_OPT_NAME)
        .args(["--version"])
        .output()
        .is_ok()
}

/// This one is useful for some of the special unmanaged EI tests in the framework.
/// Will do nothing for regular contracts.
fn copy_to_wasm_unmanaged_ei() {
    if std::path::Path::new(WASM_NO_MANAGED_EI_LIB_PATH).exists() {
        fs::copy(WASM_LIB_PATH, WASM_NO_MANAGED_EI_LIB_PATH).unwrap();
    }
}
