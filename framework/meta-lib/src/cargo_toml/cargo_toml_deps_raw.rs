use std::{collections::BTreeSet, path::PathBuf};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct DependencyRawValue {
    pub version: Option<String>,
    pub workspace: bool,
    pub git: Option<String>,
    pub rev: Option<String>,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub path: Option<PathBuf>,
    pub features: BTreeSet<String>,
    pub default_features: Option<bool>,
    pub optional: Option<bool>,
    pub package: Option<String>,
    pub registry: Option<String>,
}

/// All dependency table keys recognized by Cargo and modeled in [`DependencyRawValue`].
const KNOWN_DEP_KEYS: &[&str] = &[
    "version",
    "workspace",
    "path",
    "git",
    "rev",
    "branch",
    "tag",
    "features",
    "default-features",
    "optional",
    "package",
    "registry",
];

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
            toml::Value::Table(table) => {
                for key in table.keys() {
                    assert!(
                        KNOWN_DEP_KEYS.contains(&key.as_str()),
                        "unknown dependency key: {key:?}"
                    );
                }
                let mut result = DependencyRawValue::default();
                if let Some(toml::Value::String(version)) = table.get("version") {
                    result.version = Some(version.to_owned());
                }
                if let Some(toml::Value::Boolean(workspace)) = table.get("workspace") {
                    result.workspace = *workspace;
                }
                if let Some(toml::Value::String(path)) = table.get("path") {
                    result.path = Some(PathBuf::from(path));
                }
                if let Some(toml::Value::String(git)) = table.get("git") {
                    result.git = Some(git.to_owned());
                }
                if let Some(toml::Value::String(rev)) = table.get("rev") {
                    result.rev = Some(rev.to_owned());
                }
                if let Some(toml::Value::String(branch)) = table.get("branch") {
                    result.branch = Some(branch.to_owned());
                }
                if let Some(toml::Value::String(tag)) = table.get("tag") {
                    result.tag = Some(tag.to_owned());
                }
                if let Some(toml::Value::Array(features)) = table.get("features") {
                    result.features = features
                        .iter()
                        .map(|feature| feature.as_str().expect("feature is not a string"))
                        .map(str::to_owned)
                        .collect();
                }
                if let Some(toml::Value::Boolean(default_features)) =
                    table.get("default-features")
                {
                    result.default_features = Some(*default_features);
                }
                if let Some(toml::Value::Boolean(optional)) = table.get("optional") {
                    result.optional = Some(*optional);
                }
                if let Some(toml::Value::String(package)) = table.get("package") {
                    result.package = Some(package.to_owned());
                }
                if let Some(toml::Value::String(registry)) = table.get("registry") {
                    result.registry = Some(registry.to_owned());
                }
                result
            }
            _ => panic!("Unsupported dependency value"),
        }
    }

    pub fn into_toml_value(self) -> toml::Value {
        let mut table = toml::map::Map::new();

        if let Some(version) = self.version {
            table.insert("version".to_string(), toml::Value::String(version));
        }

        if self.workspace {
            table.insert("workspace".to_string(), toml::Value::Boolean(true));
        }

        if let Some(git) = self.git {
            table.insert("git".to_string(), toml::Value::String(git));
        }

        if let Some(rev) = self.rev {
            table.insert("rev".to_string(), toml::Value::String(rev));
        }

        if let Some(branch) = self.branch {
            table.insert("branch".to_string(), toml::Value::String(branch));
        }

        if let Some(tag) = self.tag {
            table.insert("tag".to_string(), toml::Value::String(tag));
        }

        if let Some(path) = self.path {
            table.insert(
                "path".to_string(),
                toml::Value::String(path.to_string_lossy().into_owned()),
            );
        }

        if !self.features.is_empty() {
            table.insert(
                "features".to_string(),
                toml::Value::Array(
                    self.features
                        .iter()
                        .map(|feature| toml::Value::String(feature.clone()))
                        .collect(),
                ),
            );
        }

        if let Some(default_features) = self.default_features {
            table.insert(
                "default-features".to_string(),
                toml::Value::Boolean(default_features),
            );
        }

        if let Some(optional) = self.optional {
            table.insert("optional".to_string(), toml::Value::Boolean(optional));
        }

        if let Some(package) = self.package {
            table.insert("package".to_string(), toml::Value::String(package));
        }

        if let Some(registry) = self.registry {
            table.insert("registry".to_string(), toml::Value::String(registry));
        }

        toml::Value::Table(table)
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
