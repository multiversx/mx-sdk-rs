use multiversx_sc::abi::ContractAbi;

use super::sc_config_proxy::PathRename;
const DEFAULT_PATH: &str = "/output/proxy.rs";

#[derive(Debug)]
pub struct ProxyConfig {
    pub path: String,
    pub override_import: String,
    pub path_rename: Vec<PathRename>,
    pub abi: ContractAbi,
}

impl ProxyConfig {
    pub fn new(
        path: String,
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
        ProxyConfig {
            path: DEFAULT_PATH.to_string(),
            override_import: String::new(),
            path_rename: Vec::new(),
            abi,
        }
    }
}
