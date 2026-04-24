use serde::{Deserialize, Serialize};

use super::local_deps::DependencyDepth;

pub const SCHEMA_VERSION: &str = "2.0.0";
pub const SOURCE_JSON_EXTENSION: &str = ".source.json";

/// Sentinel depth for project-level files (mirrors Python's `sys.maxsize`).
pub const SYS_MAXSIZE: DependencyDepth = i64::MAX as DependencyDepth;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceFileEntry {
    pub path: String,
    pub content: String,
    pub module: String,
    pub dependency_depth: DependencyDepth,
    pub is_test_file: bool,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceBuildMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_rust: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_sc_tool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_wasm_opt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_platform: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceBuildOptions {
    /// Kept for compatibility with the Python builder.
    pub package_whole_project_src: bool,
    pub specific_contract: Option<String>,
    pub build_root_folder: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceMetadata {
    pub contract_name: String,
    pub contract_version: String,
    pub build_metadata: SourceBuildMetadata,
    pub build_options: SourceBuildOptions,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackedSource {
    pub schema_version: String,
    pub metadata: SourceMetadata,
    pub entries: Vec<SourceFileEntry>,
}
