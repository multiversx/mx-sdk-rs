use multiversx_sc::abi::ContractAbi;

use super::sc_config_proxy::PathRename;
const OUTPUT_DIR_PROXY_PATH: &str = "/output/proxy.rs";

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

    pub fn output_dir_proxy_config(abi: ContractAbi) -> Self {
        ProxyConfig {
            path: OUTPUT_DIR_PROXY_PATH.to_string(),
            override_import: String::new(),
            path_rename: Vec::new(),
            abi,
        }
    }
}
