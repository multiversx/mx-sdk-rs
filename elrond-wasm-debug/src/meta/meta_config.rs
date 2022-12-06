use std::fs;

use elrond_wasm::abi::ContractAbi;

use super::{
    meta_build_args::BuildArgs,
    output_contract::{CargoTomlContents, OutputContractConfig},
};

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
        let main_contract = self.output_contracts.main_contract();

        // using the same local structure for all contracts is enough for now
        let mut cargo_toml_contents =
            CargoTomlContents::load_from_file(main_contract.cargo_toml_path());
        for secondary_contract in self.output_contracts.secondary_contracts() {
            cargo_toml_contents.change_package_name(secondary_contract.wasm_crate_name());
            cargo_toml_contents.save_to_file(secondary_contract.cargo_toml_path());
        }
    }

    fn generate_wasm_src_lib(&self) {
        for output_contract in &self.output_contracts.contracts {
            output_contract.generate_wasm_src_lib_file();
        }
    }

    pub fn build(&mut self) {
        self.check_tools_installed();

        for output_contract in &self.output_contracts.contracts {
            output_contract.build_contract(&self.build_args, self.output_dir.as_str());
        }
    }

    /// Convenince functionality, to get all flags right for the debug build.
    pub fn build_dbg(&mut self) {
        self.build_args.wasm_name_suffix = Some("dbg".to_string());
        self.build_args.wasm_opt = false;
        self.build_args.debug_symbols = true;
        self.build_args.wat = true;
        self.build_args.extract_imports = false;
        self.build();
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

/// This one is useful for some of the special unmanaged EI tests in the framework.
/// Will do nothing for regular contracts.
fn copy_to_wasm_unmanaged_ei() {
    if std::path::Path::new(WASM_NO_MANAGED_EI_LIB_PATH).exists() {
        fs::copy(WASM_LIB_PATH, WASM_NO_MANAGED_EI_LIB_PATH).unwrap();
    }
}
