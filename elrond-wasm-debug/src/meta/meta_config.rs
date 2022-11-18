use elrond_wasm::abi::{ContractAbi, EndpointLocationAbi};

use super::{
    meta_build_args::BuildArgs,
    output_contract::{self, OutputContract, OutputContractConfig},
};

pub struct ContractMetadata {
    pub location: EndpointLocationAbi,
    pub wasm_crate_name: String,
    pub wasm_crate_path: String,
    pub output_base_name: String,
    pub original_abi: ContractAbi,
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
    pub main_contract: Option<ContractMetadata>,
    pub view_contract: Option<ContractMetadata>,
    pub output_contracts: OutputContractConfig,
}

impl MetaConfig {
    pub fn create(original_contract_abi: &ContractAbi, build_args: BuildArgs) -> MetaConfig {
        // let main_contract_abi = original_contract_abi.main_contract();
        // let main_contract_crate_name = main_contract_abi.get_crate_name();

        // let main_contract = ContractMetadata {
        //     location: EndpointLocationAbi::MainContract,
        //     wasm_crate_name: format!("{}-wasm", &main_contract_crate_name),
        //     wasm_crate_path: "../wasm".to_string(),
        //     output_base_name: main_contract_crate_name.to_string(),
        //     original_abi: main_contract_abi.clone(),
        // };

        // let view_contract_opt =
        //     if original_contract_abi.location_exists(EndpointLocationAbi::ViewContract) {
        //         let view_contract_abi =
        //             original_contract_abi.secondary_contract(EndpointLocationAbi::ViewContract);
        //         Some(ContractMetadata {
        //             location: EndpointLocationAbi::ViewContract,
        //             wasm_crate_name: format!("{}-wasm", &main_contract_crate_name),
        //             wasm_crate_path: "../wasm-view".to_string(),
        //             output_base_name: format!("{}-view", main_contract_crate_name),
        //             original_abi: view_contract_abi,
        //         })
        //     } else {
        //         None
        //     };

        let output_contracts = OutputContractConfig::load_from_file_or_default(
            "../multicontract.toml",
            original_contract_abi,
        );

        MetaConfig {
            build_args,
            output_dir: "../output".to_string(),
            snippets_dir: "../interact-rs".to_string(),
            main_contract: None,
            view_contract: None,
            output_contracts,
        }
    }
}
