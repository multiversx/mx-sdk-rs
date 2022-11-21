use elrond_wasm::abi::ContractAbi;

pub const DEFAULT_LABEL: &str = "default";

#[derive(Debug)]
pub struct OutputContractConfig {
    pub default_contract_config_name: String,
    pub contracts: Vec<OutputContract>,
}

impl OutputContractConfig {
    pub fn main_contract(&self) -> &OutputContract {
        self.contracts
            .iter()
            .find(|contract| contract.config_name == self.default_contract_config_name)
            .unwrap_or_else(|| {
                panic!(
                    "Could not find default contract '{}' among the output contracts.",
                    self.default_contract_config_name
                )
            })
    }

    pub fn secondary_contracts(&self) -> impl Iterator<Item = &OutputContract> {
        self.contracts
            .iter()
            .filter(move |contract| contract.config_name != self.default_contract_config_name)
    }
}

/// Represents a contract created by the framework when building.
///
/// It might have only some of the endpoints written by the developer and maybe some other function.
#[derive(Debug)]
pub struct OutputContract {
    /// External view contracts are just readers of data from another contract.
    pub external_view: bool,

    /// The name, as defined in `multicontract.toml`.
    pub config_name: String,

    /// The name, as seen in the generated contract names.
    pub public_name: String,

    /// Filtered and processed ABI of the output contract.
    pub abi: ContractAbi,
}

impl OutputContract {
    pub fn wasm_crate_name(&self, main: bool) -> String {
        if main {
            return "wasm".to_string();
        } else {
            format!("wasm-{}", &self.public_name)
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
        format!("{}.abi.json", &self.public_name)
    }

    pub fn wasm_output_name(&self) -> String {
        format!("{}.wasm", &self.public_name)
    }
}
