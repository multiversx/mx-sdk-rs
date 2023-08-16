use std::path::PathBuf;

use super::OutputContractSettings;
use crate::cli_args::BuildArgs;
use multiversx_sc::abi::ContractAbi;

/// Represents a contract created by the framework when building.
///
/// It might have only some of the endpoints written by the developer and maybe some other function.
pub struct OutputContract {
    /// If it is the main contract, then the wasm crate is called just `wasm`,
    ///and the wasm `Cargo.toml` is provided by the dev.
    pub main: bool,

    /// The contract id is defined in `multicontract.toml`. It has no effect on the produced assets.
    ///
    /// It can be the same as the contract name, but it is not necessary.
    pub contract_id: String,

    /// The name, as seen in the generated contract names.
    ///
    /// It is either defined in the multicontract.toml, or is inferred from the main crate name.
    pub contract_name: String,

    /// The name of the wasm crate, as it appear in Cargo.toml. It is normally the `contract_name` field, followed by the `-wasm` suffix.
    ///
    /// However, the main contract Cargo.toml is given explicitly, so this name might differ.
    pub wasm_crate_name: String,

    /// Collection of flags, specified in the multicontract config.
    pub settings: OutputContractSettings,

    /// Filtered and processed ABI of the output contract.
    pub abi: ContractAbi,
}

impl OutputContract {
    pub fn public_name_snake_case(&self) -> String {
        self.contract_name.replace('-', "_")
    }

    /// The name of the directory of the wasm crate.
    ///
    /// Note this does not necessarily have to match the wasm crate name defined in Cargo.toml.
    pub fn wasm_crate_dir_name(&self) -> String {
        if self.main {
            "wasm".to_string()
        } else {
            format!("wasm-{}", &self.contract_name)
        }
    }

    pub fn wasm_crate_path(&self) -> String {
        format!("../{}", &self.wasm_crate_dir_name())
    }

    pub fn cargo_toml_path(&self) -> String {
        format!("{}/Cargo.toml", &self.wasm_crate_path())
    }

    pub fn wasm_crate_name_snake_case(&self) -> String {
        self.wasm_crate_name.replace('-', "_")
    }

    pub fn resolve_wasm_target_dir(&self, explicit_target_dir: &Option<String>) -> String {
        let wasm_crate_path = self.wasm_crate_path();
        if let Some(explicit_target_dir) = explicit_target_dir {
            // usually the explicit_target_dir is absolute,
            // but if it isn't, we need to take the path of the wasm crate into account
            PathBuf::from(wasm_crate_path)
                .join(explicit_target_dir)
                .to_str()
                .unwrap()
                .to_string()
        } else {
            format!("{}/target", &wasm_crate_path)
        }
    }

    /// This is where Rust will initially compile the WASM binary.
    pub fn wasm_compilation_output_path(&self, explicit_target_dir: &Option<String>) -> String {
        let target_dir = self.resolve_wasm_target_dir(explicit_target_dir);

        format!(
            "{}/wasm32-unknown-unknown/release/{}.wasm",
            &target_dir,
            &self.wasm_crate_name_snake_case(),
        )
    }

    pub fn abi_output_name(&self) -> String {
        format!("{}.abi.json", &self.contract_name)
    }

    fn output_name_base(&self, build_args: &BuildArgs) -> String {
        if let Some(wasm_name_override) = &build_args.wasm_name_override {
            wasm_name_override.clone()
        } else if let Some(suffix) = &build_args.wasm_name_suffix {
            format!("{}-{suffix}", &self.contract_name)
        } else {
            self.contract_name.clone()
        }
    }

    pub fn wasm_output_name(&self, build_args: &BuildArgs) -> String {
        format!("{}.wasm", self.output_name_base(build_args))
    }

    pub fn wat_output_name(&self, build_args: &BuildArgs) -> String {
        format!("{}.wat", self.output_name_base(build_args))
    }

    pub fn mxsc_file_output_name(&self, build_args: &BuildArgs) -> String {
        format!("{}.mxsc.json", self.output_name_base(build_args))
    }

    pub fn imports_json_output_name(&self, build_args: &BuildArgs) -> String {
        format!("{}.imports.json", self.output_name_base(build_args))
    }

    pub fn twiggy_top_name(&self, build_args: &BuildArgs) -> String {
        format!("twiggy-top-{}.txt", self.output_name_base(build_args))
    }

    pub fn twiggy_paths_name(&self, build_args: &BuildArgs) -> String {
        format!("twiggy-paths-{}.txt", self.output_name_base(build_args))
    }

    pub fn twiggy_monos_name(&self, build_args: &BuildArgs) -> String {
        format!("twiggy-monos-{}.txt", self.output_name_base(build_args))
    }

    pub fn twiggy_dominators_name(&self, build_args: &BuildArgs) -> String {
        format!(
            "twiggy-dominators-{}.txt",
            self.output_name_base(build_args)
        )
    }

    pub fn endpoint_names(&self) -> Vec<String> {
        self.abi
            .endpoints
            .iter()
            .map(|endpoint| endpoint.name.to_string())
            .collect()
    }

    /// Yields "init" + all endpoint names + "callBack" (if it exists).
    ///
    /// Should correspond to all wasm exported functions.
    pub fn all_exported_function_names(&self) -> Vec<String> {
        let mut result = vec!["init".to_string()];
        result.append(&mut self.endpoint_names());
        if self.abi.has_callback {
            result.push("callBack".to_string());
        }
        result
    }
}

impl std::fmt::Debug for OutputContract {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OutputContract")
            .field("main", &self.main)
            .field("config_name", &self.contract_id)
            .field("public_name", &self.contract_name)
            .finish()
    }
}
