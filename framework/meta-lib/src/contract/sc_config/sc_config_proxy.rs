use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct ProxyConfigSerde {
    #[serde(default)]
    pub path: PathBuf,

    #[serde(default)]
    #[serde(rename = "override-import")]
    pub override_import: Option<String>,

    #[serde(default)]
    #[serde(rename = "path-rename")]
    pub path_rename: Option<Vec<PathRename>>,

    #[serde(default)]
    pub variant: Option<String>,

    #[serde(rename = "add-unlabelled")]
    pub add_unlabelled: Option<bool>,

    #[serde(default)]
    #[serde(rename = "add-labels")]
    pub add_labels: Vec<String>,

    #[serde(default)]
    #[serde(rename = "add-endpoints")]
    pub add_endpoints: Vec<String>,

    /// Output source shape: the legacy `TxProxyTrait`-based proxy (default), a
    /// `#[contract_abi(call = ...)]`-annotated trait (`"abi"`), or the raw, hand-writable
    /// `AbiProxyTrait` implementation that macro expands to (`"abi-raw"`, `adder_abi.rs` shape).
    #[serde(default)]
    pub format: ProxyFormat,
}

#[derive(Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ProxyFormat {
    #[default]
    Proxy,
    Abi,
    AbiRaw,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathRename {
    #[serde(default)]
    pub from: String,

    #[serde(default)]
    pub to: String,
}
