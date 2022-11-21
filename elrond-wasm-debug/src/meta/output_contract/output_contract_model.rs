use std::iter::Filter;

use elrond_wasm::{abi::ContractAbi, contract_base};

pub const DEFAULT_LABEL: &str = "default";

#[derive(Debug)]
pub struct OutputContractConfig {
    pub default: String,
    pub contracts: Vec<OutputContract>,
}

impl OutputContractConfig {
    pub fn main_contract(&self) -> &OutputContract {
        self.contracts
            .iter()
            .find(|contract| contract.name == self.default)
            .unwrap()
    }

    pub fn secondary_contracts(&self) -> impl Iterator<Item = &OutputContract> {
        self.contracts
            .iter()
            .filter(move |contract| contract.name != self.default)
    }
}

#[derive(Debug)]
pub struct OutputContract {
    pub external_view: bool,
    pub name: String,
    pub abi: ContractAbi,
}

impl OutputContract {
    pub fn wasm_crate_name(&self, main: bool) -> String {
        if main {
            return "wasm".to_string();
        } else {
            format!("wasm-{}", &self.name)
        }
    }

    pub fn wasm_crate_path(&self, main: bool) -> String {
        format!("../{}", &self.wasm_crate_name(main))
    }

    pub fn cargo_toml_path(&self, main: bool) -> String {
        format!("{}/Cargo.toml", &self.wasm_crate_path(main))
    }

    /// This is where Rust will initially compile the WASM binary.
    pub fn wasm_compilation_output_path(
        &self,
        explicit_target_dir: &Option<String>,
        main: bool,
    ) -> String {
        let target_dir = explicit_target_dir
            .clone()
            .unwrap_or_else(|| format!("{}/target", &self.wasm_crate_path(main),));
        format!(
            "{}/wasm32-unknown-unknown/release/{}.wasm",
            &target_dir,
            &self.wasm_crate_name(main).replace('-', "_")
        )
    }

    pub fn abi_output_name(&self) -> String {
        format!("{}.abi.json", &self.name)
    }

    pub fn wasm_output_name(&self) -> String {
        format!("{}.wasm", &self.name)
    }
}
