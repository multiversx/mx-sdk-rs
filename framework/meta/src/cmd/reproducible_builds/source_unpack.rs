use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::cli::SourceUnpackArgs;

use super::source_json_model::PackedSource;

pub const HARDCODED_UNWRAP_FOLDER: &str = "/tmp/unwrapped";

/// CLI entry point for `sc-meta reproducible-build source-unpack`.
pub fn source_unpack(args: &SourceUnpackArgs) {
    let output_folder = args
        .output
        .as_deref()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(HARDCODED_UNWRAP_FOLDER));
    let (folder, build_root) = unpack_packaged_src(Path::new(&args.packaged_src), &output_folder);
    println!("Unwrapped to:     {}", folder.display());
    println!("Build root folder: {build_root}");
}

/// Unpacks a `.source.json` to `unwrap_folder` and returns:
/// - the canonicalized unwrap folder
/// - the `buildRootFolder` recorded in the JSON metadata
pub fn unpack_packaged_src(src_path: &Path, unwrap_folder: &Path) -> (PathBuf, String) {
    let text = fs::read_to_string(src_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", src_path.display()));
    let packed: PackedSource = serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("Failed to parse {}: {e}", src_path.display()));

    if unwrap_folder.exists() {
        fs::remove_dir_all(unwrap_folder).unwrap();
    }

    for entry in &packed.entries {
        let file_path = unwrap_folder.join(&entry.path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let content = BASE64
            .decode(&entry.content)
            .unwrap_or_else(|e| panic!("Failed to decode entry '{}': {e}", entry.path));
        fs::write(&file_path, content).unwrap();
    }

    println!(
        "Unpacked {} entries to: {}",
        packed.entries.len(),
        unwrap_folder.display()
    );

    let folder = unwrap_folder.canonicalize().unwrap();
    (folder, packed.metadata.build_options.build_root_folder)
}
