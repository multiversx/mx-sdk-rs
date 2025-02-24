use std::{
    fs,
    path::{Path, PathBuf},
};

use multiversx_sc::abi::ContractAbi;

use crate::{
    cargo_toml::CargoTomlContents,
    cli::BuildArgs,
    print_util::{print_removing_wasm_crate, print_workspace_target_dir},
    tools::{check_tools_installed, find_current_workspace},
};

use super::{
    sc_config::ScConfig, wasm_cargo_toml_data::WasmCargoTomlData,
    wasm_cargo_toml_generate::generate_wasm_cargo_toml,
};

const OUTPUT_RELATIVE_PATH: &str = "output";
const SNIPPETS_RELATIVE_PATH: &str = "interactor";
const WASM_NO_MANAGED_EI: &str = "wasm-no-managed-ei";
const FRAMEWORK_NAME_BASE: &str = "multiversx-sc";

#[derive(Debug)]
pub struct MetaConfig {
    pub load_abi_git_version: bool,
    pub output_dir: PathBuf,
    pub snippets_dir: PathBuf,
    pub original_contract_abi: ContractAbi,
    pub sc_config: ScConfig,
}

impl MetaConfig {
    pub fn create(original_contract_abi: ContractAbi, load_abi_git_version: bool) -> MetaConfig {
        let sc_config = ScConfig::load_from_crate_or_default("..", &original_contract_abi);
        let output_relative_path = Path::new("..").join(OUTPUT_RELATIVE_PATH);
        let snippets_dir = Path::new("..").join(SNIPPETS_RELATIVE_PATH);

        MetaConfig {
            load_abi_git_version,
            output_dir: output_relative_path,
            snippets_dir,
            original_contract_abi,
            sc_config,
        }
    }

    pub fn reload_sc_config(&mut self) {
        self.sc_config = ScConfig::load_from_crate_or_default("..", &self.original_contract_abi);
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
        let main_cargo_toml_contents =
            CargoTomlContents::load_from_file(Path::new("..").join("Cargo.toml"));
        let crate_name = main_cargo_toml_contents.package_name();

        for contract in self.sc_config.contracts.iter() {
            let cargo_toml_data = WasmCargoTomlData {
                name: contract.wasm_crate_name.clone(),
                edition: main_cargo_toml_contents.package_edition(),
                profile: contract.settings.profile.clone(),
                framework_dependency: main_cargo_toml_contents
                    .dependency_raw_value(FRAMEWORK_NAME_BASE)
                    .expect("missing framework dependency in Cargo.toml"),
                contract_features: contract.settings.features.clone(),
                contract_default_features: contract.settings.default_features,
            };
            generate_wasm_cargo_toml(&cargo_toml_data, crate_name.as_str())
                .save_to_file(contract.cargo_toml_path());
        }
    }
}

impl MetaConfig {
    fn generate_wasm_src_lib(&self) {
        for contract_variant in &self.sc_config.contracts {
            contract_variant.generate_wasm_src_lib_file();
        }
    }

    pub fn build(&mut self, mut build_args: BuildArgs) {
        check_tools_installed(&mut build_args);
        adjust_target_dir_wasm(&mut build_args);

        for contract_variant in &self.sc_config.contracts {
            contract_variant.build_contract(&build_args, &self.output_dir);
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
        if !dir_name.starts_with("wasm") {
            return true;
        }

        if dir_name == WASM_NO_MANAGED_EI {
            return true;
        }

        self.sc_config
            .contracts
            .iter()
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
                    print_removing_wasm_crate(dir_name);
                    fs::remove_dir_all(path.path()).unwrap_or_else(|_| {
                        panic!("failed to remove unexpected directory {dir_name}")
                    });
                }
            }
        }
    }
}

fn adjust_target_dir_wasm(build_args: &mut BuildArgs) {
    if build_args.target_dir_wasm.is_some() {
        return;
    }

    if let Some(workspace) = find_current_workspace() {
        let target = workspace.join("target").canonicalize().unwrap();
        if let Some(target_str) = target.as_os_str().to_str() {
            build_args.target_dir_wasm = Some(target_str.to_string());
            print_workspace_target_dir(target_str);
        }
    }
}

/// This one is useful for some of the special unmanaged EI tests in the framework.
/// Will do nothing for regular contracts.
fn copy_to_wasm_unmanaged_ei() {
    let wasm_no_managed_ei_path = Path::new("..")
        .join("wasm-no-managed-ei")
        .join("src")
        .join("lib.rs");

    if wasm_no_managed_ei_path.exists() {
        let wasm_lib_path = Path::new("..").join("wasm").join("src").join("lib.rs");
        fs::copy(wasm_lib_path, wasm_no_managed_ei_path).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{cargo_toml::DependencyRawValue, contract::sc_config::ContractVariantProfile};

    const EXPECTED_CARGO_TOML_CONTENTS: &str =
        "# Code generated by the multiversx-sc build system. DO NOT EDIT.

# ##########################################
# ############## AUTO-GENERATED #############
# ##########################################

[package]
name = \"test\"
version = \"0.0.0\"
edition = \"2021\"
publish = false

[lib]
crate-type = [\"cdylib\"]

[profile.release]
codegen-units = 1
opt-level = \"z\"
lto = true
debug = false
panic = \"abort\"
overflow-checks = false

[profile.dev]
panic = \"abort\"

[dependencies.test-crate-name]
path = \"..\"

[dependencies.multiversx-sc-wasm-adapter]
version = \"x.y.z\"
path = \"../../../../framework/wasm-adapter\"

[workspace]
members = [\".\"]
";

    #[test]
    fn test_generate_cargo() {
        let path = Path::new("..")
            .join("..")
            .join("..")
            .join("framework")
            .join("base");
        let wasm_cargo_toml_data = super::WasmCargoTomlData {
            name: "test".to_string(),
            edition: "2021".to_string(),
            profile: ContractVariantProfile::default(),
            framework_dependency: DependencyRawValue {
                version: Some("x.y.z".to_owned()),
                path: Option::Some(path),
                ..Default::default()
            },
            contract_features: Vec::<String>::new(),
            contract_default_features: None,
        };
        let crate_name = "test-crate-name".to_string();
        let generated_contents =
            super::generate_wasm_cargo_toml(&wasm_cargo_toml_data, &crate_name);

        assert_eq!(
            generated_contents.to_string_pretty(),
            EXPECTED_CARGO_TOML_CONTENTS.to_string()
        );
    }
}
