use std::{fs::create_dir_all, path::PathBuf};

use elrond_wasm::abi::{ContractAbi, EndpointLocationAbi};

#[derive(Debug)]
pub struct BuildArgs {
    pub debug_symbols: bool,
    pub wasm_name_override: Option<String>,
    pub wasm_name_suffix: Option<String>,
    pub wasm_opt: bool,
}

impl Default for BuildArgs {
    fn default() -> Self {
        BuildArgs {
            debug_symbols: false,
            wasm_name_override: None,
            wasm_name_suffix: None,
            wasm_opt: true,
        }
    }
}

impl BuildArgs {
    pub fn wasm_name(&self, contract_metadata: &ContractMetadata) -> String {
        if let Some(wasm_name_override) = &self.wasm_name_override {
            return wasm_name_override.clone();
        }
        if let Some(wasm_suffix) = &self.wasm_name_suffix {
            format!(
                "{}-{}.wasm",
                contract_metadata.output_base_name, wasm_suffix
            )
        } else {
            contract_metadata.wasm_output_name()
        }
    }
}

pub struct ContractMetadata {
    pub location: EndpointLocationAbi,
    pub wasm_crate_name: String,
    pub wasm_crate_path: String,
    pub output_base_name: String,
    pub abi: ContractAbi,
}

impl ContractMetadata {
    pub fn cargo_toml_path(&self) -> String {
        format!("{}/Cargo.toml", &self.wasm_crate_path)
    }

    /// This is where Rust will initially compile the WASM binary.
    pub fn wasm_compilation_output_path(&self) -> String {
        format!(
            "{}/target/wasm32-unknown-unknown/release/{}.wasm",
            &self.wasm_crate_path,
            &self.wasm_crate_name.replace('-', "_")
        )
    }

    pub fn abi_output_name(&self) -> String {
        format!("{}.abi.json", &self.output_base_name)
    }

    pub fn wasm_output_name(&self) -> String {
        format!("{}.wasm", &self.output_base_name)
    }
}

pub struct MetaConfig {
    pub build_args: BuildArgs,
    pub output_dir: String,
    pub main_contract: Option<ContractMetadata>,
    pub view_contract: Option<ContractMetadata>,
}

pub fn process_args(args: &[String]) -> BuildArgs {
    let mut result = BuildArgs::default();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--wasm-symbols" => {
                result.debug_symbols = true;
            },
            "--wasm-name" => {
                let name = iter
                    .next()
                    .expect("argument `--wasm-name` must be followed by the desired name");
                result.wasm_name_override = Some(name.clone());
            },
            "--wasm-suffix" => {
                let suffix = iter
                    .next()
                    .expect("argument `--wasm-suffix` must be followed by the desired suffix");
                result.wasm_name_suffix = Some(suffix.clone());
            },
            "--no-wasm-opt" => {
                result.wasm_opt = false;
            },
            _ => {},
        }
    }

    result
}

impl MetaConfig {
    pub fn create(original_contract_abi: &ContractAbi, args: &[String]) -> MetaConfig {
        let build_args = process_args(args);

        let main_contract_abi = original_contract_abi.main_contract();
        let main_contract_crate_name = main_contract_abi.get_crate_name();

        let main_contract = ContractMetadata {
            location: EndpointLocationAbi::MainContract,
            wasm_crate_name: format!("{}-wasm", &main_contract_crate_name),
            wasm_crate_path: "../wasm".to_string(),
            output_base_name: main_contract_crate_name.to_string(),
            abi: main_contract_abi.clone(),
        };

        let view_contract_opt =
            if original_contract_abi.location_exists(EndpointLocationAbi::ViewContract) {
                let view_contract_abi =
                    original_contract_abi.secondary_contract(EndpointLocationAbi::ViewContract);
                Some(ContractMetadata {
                    location: EndpointLocationAbi::ViewContract,
                    wasm_crate_name: format!("{}-wasm", &main_contract_crate_name),
                    wasm_crate_path: "../wasm-view".to_string(),
                    output_base_name: format!("{}-view", main_contract_crate_name),
                    abi: view_contract_abi,
                })
            } else {
                None
            };

        MetaConfig {
            build_args,
            output_dir: "../output".to_string(),
            main_contract: Some(main_contract),
            view_contract: view_contract_opt,
        }
    }
}

impl ContractMetadata {
    pub fn create_dir_all(&self) {
        create_dir_all(PathBuf::from(&self.wasm_crate_path).join("src")).unwrap();
    }
}
