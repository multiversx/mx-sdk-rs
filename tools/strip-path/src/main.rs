use multiversx_sc_meta::cargo_toml::WorkspaceDependencies;
use multiversx_sc_meta::folder_structure::strip_path;
use std::path::Path;

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let workspace_dependencies =
        WorkspaceDependencies::find_from_dir(&root).expect("failed to find workspace");
    strip_path(
        Path::new(&root),
        &["target".to_string()],
        &workspace_dependencies,
    );
}
