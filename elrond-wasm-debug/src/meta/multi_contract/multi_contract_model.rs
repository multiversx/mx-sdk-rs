use elrond_wasm::abi::ContractAbi;



pub const DEFAULT_LABEL: &str = "default";

#[derive(Debug)]
pub struct MultiContractConfig {
    pub main_contract_name: String,
    pub contracts: Vec<ContractMetadata>,
}

#[derive(Debug)]
pub struct ContractMetadata {
    pub external_view: bool,
    pub wasm_crate_name: String,
    pub wasm_crate_path: String,
    pub output_name: String,
    pub abi: ContractAbi,
}
