use std::{fs::create_dir_all, path::PathBuf};

use elrond_wasm::abi::{ContractAbi, EndpointLocationAbi};

#[derive(Default, Debug)]
pub struct BuildArgs {
    pub debug_symbols: bool,
    pub wasm_name: Option<String>,
}

pub struct ContractMetadata {
    pub wasm_crate_name: String,
    pub wasm_crate_path: String,
    pub output_base_name: String,
    pub abi: ContractAbi,
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
                result.wasm_name = Some(name.clone());
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
        let main_output_base_name = build_args
            .wasm_name
            .clone()
            .unwrap_or_else(|| main_contract_crate_name.clone());

        let main_contract = ContractMetadata {
            wasm_crate_name: format!("{}-wasm", &main_contract_crate_name),
            wasm_crate_path: "../wasm".to_string(),
            output_base_name: main_output_base_name,
            abi: main_contract_abi.clone(),
        };

        let view_contract_opt =
            if original_contract_abi.location_exists(EndpointLocationAbi::ViewContract) {
                let view_contract_abi =
                    original_contract_abi.secondary_contract(EndpointLocationAbi::ViewContract);
                Some(ContractMetadata {
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
