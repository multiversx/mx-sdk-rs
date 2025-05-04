use std::collections::{HashMap, HashSet};

use serde::Deserialize;

use crate::folder_structure::RelevantDirectories;

const CARGO_TOML: &str = "Cargo.toml";
const SCENARIO: &str = "multiversx-sc-scenario";
const WASMER: &str = "wasmer";

#[derive(Debug, Deserialize)]
struct CargoToml {
    #[serde(default, rename = "dev-dependencies")]
    dev_dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]

enum Dependency {
    Simple(String),
    Detailed(DependencyDetail),
}

#[derive(Debug, Deserialize)]
struct DependencyDetail {
    #[serde(default)]
    pub features: Vec<String>,
}

pub fn check_executor(relevant_directories: &RelevantDirectories) {
    let mut scenario_features: HashSet<String> = HashSet::new();

    for dir in relevant_directories.iter() {
        let toml_dir = dir.path.join(CARGO_TOML);
        let content = std::fs::read_to_string(toml_dir).unwrap();
        let cargo_toml_content: CargoToml = toml::from_str(&content).unwrap();
        let dependencies = &cargo_toml_content.dev_dependencies;

        if let Some(Dependency::Detailed(dependency)) = dependencies.get(SCENARIO) {
            for feature in dependency.features.iter() {
                if !scenario_features.contains(feature) {
                    scenario_features.insert(feature.clone());
                }
            }
        }
    }

    let count = scenario_features
        .iter()
        .filter(|s| s.contains(WASMER))
        .count();

    if count > 1 {
        panic!("Cannot import two different executors: found multiple wasmer components");
    }
}
