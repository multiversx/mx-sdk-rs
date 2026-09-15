use std::path::{Path, PathBuf};

use multiversx_sc::abi::ContractAbi;

use super::sc_config_proxy::{PathRename, ProxyFormat};

#[derive(Debug)]
pub struct ProxyConfig {
    pub path: PathBuf,
    pub override_import: String,
    pub path_rename: Vec<PathRename>,
    pub abi: ContractAbi,
    pub format: ProxyFormat,
    /// `[[generate-abi]]`'s `call` field: the name to give the generated call type, i.e. the
    /// `call = ...` argument of `#[multiversx_sc_abi::contract_abi(call = ...)]`.
    /// Unused outside `ProxyFormat::Abi`.
    pub call_name: Option<String>,
}

impl ProxyConfig {
    pub fn new(
        path: PathBuf,
        override_imports: Option<String>,
        path_rename: Option<Vec<PathRename>>,
        abi: ContractAbi,
        format: ProxyFormat,
        call_name: Option<String>,
    ) -> Self {
        ProxyConfig {
            path,
            override_import: override_imports.unwrap_or_default(),
            path_rename: path_rename.unwrap_or_default(),
            abi,
            format,
            call_name,
        }
    }

    pub fn output_dir_proxy_config(abi: ContractAbi) -> Self {
        let proxy_output = abi.get_crate_name_for_code() + "_proxy.rs";
        ProxyConfig {
            path: Path::new("output").join(proxy_output),
            override_import: String::new(),
            path_rename: Vec::new(),
            abi,
            format: ProxyFormat::Proxy,
            call_name: None,
        }
    }
}
