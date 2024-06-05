use serde::Deserialize;

const DEFAULT_PATH: &str = "/output/proxy.rs";

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProxyConfigSerde {
    #[serde(default)]
    pub path: String,

    #[serde(default)]
    #[serde(rename = "override-import")]
    pub override_import: Option<String>,

    #[serde(default)]
    #[serde(rename = "path-rename")]
    pub path_rename: Option<Vec<PathRename>>,

    #[serde(default)]
    pub variant: Option<String>,

    #[serde(default)]
    #[serde(rename = "custom-endpoints")]
    pub custom_proxy_endpoints: Vec<String>,
}

impl ProxyConfigSerde {
    pub fn new() -> Self {
        Self {
            path: DEFAULT_PATH.to_string(),
            override_import: None,
            path_rename: None,
            variant: None,
            custom_proxy_endpoints: Vec::new(),
        }
    }
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct PathRename {
    #[serde(default)]
    pub from: String,

    #[serde(default)]
    pub to: String,
}
