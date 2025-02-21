use std::path::{Path, PathBuf};

use multiversx_sc::abi::ContractAbi;

use super::sc_config_proxy::PathRename;

#[derive(Debug)]
pub struct ProxyConfig {
    pub path: PathBuf,
    pub override_import: String,
    pub path_rename: Vec<PathRename>,
    pub abi: ContractAbi,
}

impl ProxyConfig {
    pub fn new(
        path: PathBuf,
        override_imports: Option<String>,
        path_rename: Option<Vec<PathRename>>,
        abi: ContractAbi,
    ) -> Self {
        ProxyConfig {
            path,
            override_import: override_imports.unwrap_or_default(),
            path_rename: path_rename.unwrap_or_default(),
            abi,
        }
    }

    pub fn new_with_default_path(abi: ContractAbi) -> Self {
        let default_path = Path::new("output").join("proxy.rs");
        ProxyConfig {
            path: default_path,
            override_import: String::new(),
            path_rename: Vec::new(),
            abi,
        }
    }
}
