use std::path::{Path, PathBuf};

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
}

impl ProxyConfigSerde {
    pub fn new() -> Self {
        Self {
            path: Path::new("output").join("proxy.rs"),
            override_import: None,
            path_rename: None,
            variant: None,
            add_unlabelled: None,
            add_labels: Vec::new(),
            add_endpoints: Vec::new(),
        }
    }
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathRename {
    #[serde(default)]
    pub from: String,

    #[serde(default)]
    pub to: String,
}
