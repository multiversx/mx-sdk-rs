use std::{collections::BTreeSet, path::PathBuf};

use serde::{Deserialize, Serialize};

fn is_false(b: &bool) -> bool {
    !*b
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DependencyRawValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub workspace: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub features: BTreeSet<String>,
    #[serde(rename = "default-features", skip_serializing_if = "Option::is_none")]
    pub default_features: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
}

impl DependencyRawValue {
    pub fn from_version(version: &str) -> Self {
        Self {
            version: Some(version.to_owned()),
            ..Default::default()
        }
    }

    pub fn parse_toml_value(toml_value: &toml::Value) -> Self {
        match toml_value {
            toml::Value::String(version) => DependencyRawValue::from_version(version),
            _ => toml_value
                .clone()
                .try_into()
                .unwrap_or_else(|e| panic!("failed to parse dependency value: {e}")),
        }
    }

    pub fn into_toml_value(self) -> toml::Value {
        toml::Value::try_from(self).expect("failed to serialize dependency value")
    }

    /// Removes the `workspace = true` flag and replaces the dependency fields from `workspace_dep`.
    ///
    /// Copies specification fields (version, git, rev, branch, tag, path, default-features,
    /// package, registry) from the workspace dep. Does not touch `features` or `optional`,
    /// which are local-only attributes that the crate controls independently.
    pub fn replace_workspace_dep(&mut self, workspace_dep: &DependencyRawValue) {
        self.workspace = false;
        self.version = workspace_dep.version.clone();
        self.git = workspace_dep.git.clone();
        self.rev = workspace_dep.rev.clone();
        self.branch = workspace_dep.branch.clone();
        self.tag = workspace_dep.tag.clone();
        self.path = workspace_dep.path.clone();
        self.default_features = workspace_dep.default_features;
        self.package = workspace_dep.package.clone();
        self.registry = workspace_dep.registry.clone();
    }
}
