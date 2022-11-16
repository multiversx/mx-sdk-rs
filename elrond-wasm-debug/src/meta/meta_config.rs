use std::{fs::create_dir_all, path::PathBuf};
use crate::meta::multi_contract::MultiContract;
use elrond_wasm::abi::{ContractAbi};

#[derive(Debug)]
pub struct BuildArgs {
    pub debug_symbols: bool,
    pub wasm_name_override: Option<String>,
    pub wasm_name_suffix: Option<String>,
    pub wasm_opt: bool,
    pub target_dir: Option<String>,
}

impl Default for BuildArgs {
    fn default() -> Self {
        BuildArgs {
            debug_symbols: false,
            wasm_name_override: None,
            wasm_name_suffix: None,
            wasm_opt: true,
            target_dir: None,
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
    pub location: String,
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
    pub fn wasm_compilation_output_path(&self, explicit_target_dir: &Option<String>) -> String {
        let target_dir = explicit_target_dir
            .clone()
            .unwrap_or_else(|| format!("{}/target", &self.wasm_crate_path,));
        format!(
            "{}/wasm32-unknown-unknown/release/{}.wasm",
            &target_dir,
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
    pub snippets_dir: String,
    pub contracts: Vec<ContractMetadata>,
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
            "--target-dir" => {
                let arg = iter
                    .next()
                    .expect("argument `--target-dir` must be followed by argument");
                result.target_dir = Some(arg.clone());
            },
            _ => {},
        }
    }

    result
}

impl MetaConfig {
    pub fn create(original_contract_abi: &ContractAbi, args: &[String]) -> MetaConfig {
        let mut locations :Vec<String> = original_contract_abi.constructors
        .iter()
        .flat_map(|endpoint| endpoint.locations.iter().map(|&location|location.to_owned()))
        .collect();

        locations.dedup();

        let mut contracts: Vec<ContractMetadata> = vec![];
        let build_args = process_args(args);

        for location in locations{
            let contract_abi: ContractAbi;
            let mut wasm_crate_path= "../wasm".to_string();
            let contract_crate_name: String;
            match location.as_str() {
                "main" => { 
                    contract_abi = original_contract_abi.main_contract(); 
                    contract_crate_name = contract_abi.get_crate_name().to_string(); 
                }
                _=> {
                    contract_abi = original_contract_abi.secondary_contract(&location);    
                    wasm_crate_path = format!("{}-{}", &wasm_crate_path, &location);
                    contract_crate_name = format!("{}-{}", contract_abi.get_crate_name(), &location);
                }
            }

            contracts.push(ContractMetadata {
                location: location.to_string(),
                wasm_crate_name: format!("{}-wasm", &contract_crate_name),
                wasm_crate_path,
                output_base_name: contract_crate_name,
                abi: contract_abi.clone(),
            });
        }

        MetaConfig {
            build_args,
            output_dir: "../output".to_string(),
            snippets_dir: "../interact-rs".to_string(),
            contracts,
        }
    }

    fn interpret_toml(original_contract_abi: &ContractAbi, args: &[String]) -> MetaConfig {
        let content = std::fs::read_to_string("multicontract.toml").unwrap();
        let contract_details: MultiContract = toml::from_str(&content).unwrap();

        let mut contracts: Vec<ContractMetadata> = vec![];
        let build_args = process_args(args);

        for (location, _) in contract_details.labels {
            let contract_abi: ContractAbi;
            let mut wasm_crate_path= "../wasm".to_string();
            let contract_crate_name: String;

            if &location == &contract_details.default{
                contract_abi = original_contract_abi.main_contract(); 
                contract_crate_name = contract_abi.get_crate_name().to_string(); 
            }
            else{
                contract_abi = original_contract_abi.secondary_contract(&location);    
                wasm_crate_path = format!("{}-{}", &wasm_crate_path, &location);
                contract_crate_name = format!("{}-{}", contract_abi.get_crate_name(), &location);
            }
            contracts.push(ContractMetadata {
                location,
                wasm_crate_name: format!("{}-wasm", &contract_crate_name),
                wasm_crate_path,
                output_base_name: contract_crate_name,
                abi: contract_abi.clone(),
            });
        }

        MetaConfig {
            build_args,
            output_dir: "../output".to_string(),
            snippets_dir: "../interact-rs".to_string(),
            contracts,
        }
    }
}

impl ContractMetadata {
    pub fn create_dir_all(&self) {
        create_dir_all(PathBuf::from(&self.wasm_crate_path).join("src")).unwrap();
    }
}
