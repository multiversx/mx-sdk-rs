use std::path::PathBuf;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct DependencyRawValue {
    pub version: Option<String>,
    pub git: Option<String>,
    pub rev: Option<String>,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub path: Option<PathBuf>,
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
            toml::Value::Table(table) => {
                let mut result = DependencyRawValue::default();
                if let Some(toml::Value::String(version)) = table.get("version") {
                    result.version = Some(version.to_owned());
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
                result
            },
            _ => panic!("Unsupported dependency value"),
        }
    }

    pub fn into_toml_value(self) -> toml::Value {
        let mut table = toml::map::Map::new();

        if let Some(version) = self.version {
            table.insert("version".to_string(), toml::Value::String(version));
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

        toml::Value::Table(table)
    }
}
