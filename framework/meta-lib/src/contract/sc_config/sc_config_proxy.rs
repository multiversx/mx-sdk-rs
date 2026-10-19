use std::path::PathBuf;

use serde::Deserialize;

/// Config entry for any of the three proxy-generation lists: `[[proxy]]`, `[[generate-abi]]`,
/// `[[generate-abi-raw]]`. Which list an entry came from is tracked separately as `ProxyFormat`.
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

    /// Only valid in `[[generate-abi]]`: name to give the generated call type, i.e. the
    /// `call = ...` argument of `#[multiversx_sc_abi::contract_abi(call = ...)]`. Defaults to
    /// the usual `<ContractName>Proxy`-style name when left unset.
    #[serde(default)]
    pub call: Option<String>,
}

impl ProxyConfigSerde {
    /// Validates what plain deserialization cannot enforce.
    ///
    /// `path-rename` entries with an empty `from` are rejected: `PathRename::from` defaults to
    /// `""` when the field is omitted, which would otherwise make every generated path match (via
    /// `rust_path.contains("")`) and get silently rewritten.
    pub fn validate(&self, format: ProxyFormat) {
        if let Some(path_rename) = &self.path_rename {
            for pr in path_rename {
                assert!(
                    !pr.from.is_empty(),
                    "invalid `path-rename` entry: `from` is missing or empty (`to` = {:?}); \
                     `from` must be set to the ABI name or path to rename",
                    pr.to
                );
            }
        }

        if let Some(call) = &self.call {
            assert!(
                format == ProxyFormat::Abi,
                "`call = {call:?}` is only allowed in `[[generate-abi]]` entries (found in `[[{}]]`, path = {:?})",
                format.list_name(),
                self.path
            );
        }
    }
}

/// Which generator produced a given `ProxyConfig`. Not read from TOML directly — assigned based
/// on which list (`[[proxy]]`/`[[generate-abi]]`/`[[generate-abi-raw]]`) an entry came from.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProxyFormat {
    #[default]
    Proxy,
    Abi,
    AbiRaw,
}

impl ProxyFormat {
    pub fn list_name(&self) -> &'static str {
        match self {
            ProxyFormat::Proxy => "proxy",
            ProxyFormat::Abi => "generate-abi",
            ProxyFormat::AbiRaw => "generate-abi-raw",
        }
    }
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct PathRename {
    #[serde(default)]
    pub from: String,

    #[serde(default)]
    pub to: String,
}

#[cfg(test)]
mod tests {
    use crate::contract::sc_config::sc_config_serde::ScConfigSerde;

    #[test]
    fn unknown_proxy_field_is_rejected() {
        for list in ["proxy", "generate-abi", "generate-abi-raw"] {
            let toml = format!("[[{list}]]\npath = \"src/proxy.rs\"\nbogus = 1\n");
            let err = toml::from_str::<ScConfigSerde>(&toml).unwrap_err();
            assert!(err.to_string().contains("bogus"), "{list}: {err}");
        }
    }

    #[test]
    fn unknown_path_rename_field_is_rejected() {
        let toml = "[[proxy]]\npath = \"src/proxy.rs\"\n[[proxy.path-rename]]\nfrom = \"a\"\nto = \"b\"\nbogus = 1\n";
        let err = toml::from_str::<ScConfigSerde>(toml).unwrap_err();
        assert!(err.to_string().contains("bogus"), "{err}");
    }

    #[test]
    fn known_proxy_fields_parse() {
        let toml = r#"
[[proxy]]
path = "src/proxy.rs"
add-labels = ["a"]

[[generate-abi]]
path = "src/abi.rs"
call = "MyCall"

[[generate-abi-raw]]
path = "src/raw.rs"
path-rename = [{ from = "x", to = "y" }]
"#;
        let config: ScConfigSerde = toml::from_str(toml).unwrap();
        config.validate();
        assert_eq!(config.generate_abi[0].call.as_deref(), Some("MyCall"));
        assert_eq!(
            config.generate_abi_raw[0].path_rename.as_ref().unwrap()[0].to,
            "y"
        );
    }

    #[test]
    #[should_panic(expected = "only allowed in `[[generate-abi]]`")]
    fn call_in_proxy_list_is_rejected() {
        let toml = "[[proxy]]\npath = \"src/proxy.rs\"\ncall = \"MyCall\"\n";
        toml::from_str::<ScConfigSerde>(toml).unwrap().validate();
    }

    #[test]
    #[should_panic(expected = "only allowed in `[[generate-abi]]`")]
    fn call_in_generate_abi_raw_list_is_rejected() {
        let toml = "[[generate-abi-raw]]\npath = \"src/proxy.rs\"\ncall = \"MyCall\"\n";
        toml::from_str::<ScConfigSerde>(toml).unwrap().validate();
    }
}
