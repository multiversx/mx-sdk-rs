use elrond_wasm::abi::ContractAbi;

pub const DEFAULT_LABEL: &str = "default";

#[derive(Debug)]
pub struct OutputContractConfig {
    pub contracts: Vec<OutputContract>,
}

#[derive(Debug)]
pub struct OutputContract {
    pub external_view: bool,
    pub name: String,
    pub abi: ContractAbi,
}

impl OutputContract {
    pub fn wasm_crate_name(&self) -> String {
        format!("wasm-{}", &self.name)
    }

    pub fn wasm_crate_path(&self) -> String {
        format!("../{}", &self.wasm_crate_name())
    }

    pub fn cargo_toml_path(&self) -> String {
        format!("{}/Cargo.toml", &self.wasm_crate_path())
    }

    /// This is where Rust will initially compile the WASM binary.
    pub fn wasm_compilation_output_path(&self, explicit_target_dir: &Option<String>) -> String {
        let target_dir = explicit_target_dir
            .clone()
            .unwrap_or_else(|| format!("{}/target", &self.wasm_crate_path(),));
        format!(
            "{}/wasm32-unknown-unknown/release/{}.wasm",
            &target_dir,
            &self.wasm_crate_name().replace('-', "_")
        )
    }

    pub fn abi_output_name(&self) -> String {
        format!("{}.abi.json", &self.name)
    }

    pub fn wasm_output_name(&self) -> String {
        format!("{}.wasm", &self.name)
    }
}
