use std::{
    collections::BTreeMap,
    path::{Component, Path, PathBuf},
};

use toml::value::Table;

use super::{CARGO_TOML_DEPENDENCIES, CargoTomlContents, DependencyRawValue};

const WORKSPACE: &str = "workspace";

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorkspaceDependencies {
    pub workspace_path: PathBuf,
    pub dependencies: BTreeMap<String, DependencyRawValue>,
}

impl WorkspaceDependencies {
    pub fn load_from_dir(workspace_path: impl AsRef<Path>) -> Self {
        let workspace_path = workspace_path.as_ref();
        let cargo_toml_contents =
            CargoTomlContents::load_from_file(workspace_path.join("Cargo.toml"));

        Self::from_cargo_toml(workspace_path, &cargo_toml_contents)
    }

    pub fn from_cargo_toml(
        workspace_path: impl AsRef<Path>,
        cargo_toml_contents: &CargoTomlContents,
    ) -> Self {
        let dependencies = Self::dependencies_table(cargo_toml_contents)
            .map(Self::parse_dependencies_table)
            .unwrap_or_default();

        Self {
            workspace_path: workspace_path.as_ref().to_path_buf(),
            dependencies,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.dependencies.is_empty()
    }

    pub fn get(&self, crate_name: &str) -> Option<&DependencyRawValue> {
        self.dependencies.get(crate_name)
    }

    /// Resolves a single dependency from workspace root to a concrete value.
    ///
    /// This rebases workspace-relative `path` entries to be relative to `relative_dir`.
    pub(crate) fn resolve_dependency(
        &self,
        crate_name: &str,
        relative_dir: &Path,
    ) -> DependencyRawValue {
        let mut workspace_dependency = self
            .get(crate_name)
            .unwrap_or_else(|| panic!("missing workspace dependency in Cargo.toml: {crate_name}"))
            .clone();

        if let Some(path) = &mut workspace_dependency.path {
            *path = path_relative_from_to(
                &absolute_path(&self.workspace_path).join(&path),
                &absolute_path(relative_dir),
            );
        }

        workspace_dependency
    }

    fn dependencies_table(cargo_toml_contents: &CargoTomlContents) -> Option<&Table> {
        cargo_toml_contents
            .toml_value
            .get(WORKSPACE)
            .and_then(|workspace| workspace.get(CARGO_TOML_DEPENDENCIES))
            .and_then(|deps| deps.as_table())
    }

    fn parse_dependencies_table(deps: &Table) -> BTreeMap<String, DependencyRawValue> {
        deps.iter()
            .map(|(crate_name, value)| {
                (
                    crate_name.to_owned(),
                    DependencyRawValue::parse_toml_value(value),
                )
            })
            .collect()
    }
}

fn absolute_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        normalize_path(path)
    } else {
        normalize_path(&std::env::current_dir().unwrap().join(path))
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                result.pop();
            }
            _ => result.push(component.as_os_str()),
        }
    }
    result
}

fn path_relative_from_to(path: &Path, base: &Path) -> PathBuf {
    let path = normalize_path(path);
    let base = normalize_path(base);
    let path_components: Vec<_> = path.components().collect();
    let base_components: Vec<_> = base.components().collect();
    let common_len = path_components
        .iter()
        .zip(base_components.iter())
        .take_while(|(path_component, base_component)| path_component == base_component)
        .count();

    let mut relative_path = PathBuf::new();
    for _ in common_len..base_components.len() {
        relative_path.push("..");
    }
    for component in &path_components[common_len..] {
        relative_path.push(component.as_os_str());
    }
    relative_path
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn test_path_relative_from_to() {
        assert_eq!(
            super::path_relative_from_to(
                Path::new("/repo/framework/base"),
                Path::new("/repo/contracts/examples/adder"),
            ),
            Path::new("..")
                .join("..")
                .join("..")
                .join("framework")
                .join("base"),
        );
    }
}
