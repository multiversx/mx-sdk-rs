use std::path::PathBuf;

use serde::Deserialize;

/// Fields shared by all three proxy-generation lists (`[[proxy]]`, `[[generate-abi]]`,
/// `[[generate-abi-raw]]`), flattened into each of them.
#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct ProxyConfigCommon {
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
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct ProxyConfigSerde {
    #[serde(flatten)]
    pub common: ProxyConfigCommon,
}

/// Config for `[[generate-abi]]`: produces a `#[contract_abi(call = ...)]`-annotated trait, the
/// *input* the `contract_abi` macro consumes (framework-agnostic, no `TxProxyTrait`/`VMApi`
/// dependency), unlike the legacy `[[proxy]]` output.
#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct GenerateAbiConfigSerde {
    #[serde(flatten)]
    pub common: ProxyConfigCommon,

    /// Name to give the generated call type, i.e. the `call = ...` argument of
    /// `#[contract_abi(call = ...)]`. Defaults to the usual `<ContractName>Proxy`-style name
    /// when left unset.
    #[serde(default)]
    pub call: Option<String>,
}

/// Config for `[[generate-abi-raw]]`: produces the raw `AbiProxyTrait`/`ProxyArg`/`IntoXxx`
/// implementation, hand-writable style, i.e. what the `contract_abi` macro expands a
/// `[[generate-abi]]` trait to.
#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct GenerateAbiRawConfigSerde {
    #[serde(flatten)]
    pub common: ProxyConfigCommon,
}

/// Which generator produced a given `ProxyConfig`. No longer read directly from TOML (each
/// generator now has its own `[[proxy]]`/`[[generate-abi]]`/`[[generate-abi-raw]]` list instead
/// of a shared list with a `format` flag) — assigned by `process_proxy_contracts` based on which
/// list an entry came from.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
