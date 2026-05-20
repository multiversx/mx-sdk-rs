use anyhow::{Context, Result, bail};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use std::{
    fs,
    path::{Component, Path, PathBuf},
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
    let (folder, build_root) = unpack_packaged_src(Path::new(&args.packaged_src), &output_folder)
        .unwrap_or_else(|e| panic!("{e:#}"));
    println!("Unwrapped to:     {}", folder.display());
    println!("Build root folder: {build_root}");
}

/// Unpacks a `.source.json` to `unwrap_folder` and returns:
/// - the canonicalized unwrap folder
/// - the `buildRootFolder` recorded in the JSON metadata
pub fn unpack_packaged_src(src_path: &Path, unwrap_folder: &Path) -> Result<(PathBuf, String)> {
    let text = fs::read_to_string(src_path)
        .with_context(|| format!("Failed to read {}", src_path.display()))?;
    let packed: PackedSource = serde_json::from_str(&text)
        .with_context(|| format!("Failed to parse {}", src_path.display()))?;
    unpack_packed_source(&packed, unwrap_folder)
}

/// Unpacks a [`PackedSource`] to `unwrap_folder` and returns:
/// - the canonicalized unwrap folder
/// - the `buildRootFolder` recorded in the JSON metadata
pub fn unpack_packed_source(
    packed: &PackedSource,
    unwrap_folder: &Path,
) -> Result<(PathBuf, String)> {
    if unwrap_folder.exists() {
        fs::remove_dir_all(unwrap_folder)
            .with_context(|| format!("Failed to remove {}", unwrap_folder.display()))?;
    }

    for entry in &packed.entries {
        let file_path = safe_join(unwrap_folder, &entry.path)?;
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create {}", parent.display()))?;
        }
        let content = BASE64
            .decode(&entry.content)
            .with_context(|| format!("Failed to decode entry '{}'", entry.path))?;
        fs::write(&file_path, content)
            .with_context(|| format!("Failed to write {}", file_path.display()))?;
    }

    println!(
        "Unpacked {} entries to: {}",
        packed.entries.len(),
        unwrap_folder.display()
    );

    let folder = unwrap_folder
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize {}", unwrap_folder.display()))?;
    Ok((folder, packed.metadata.build_root_folder().to_string()))
}

/// Joins `base` with `rel`, returning an error if `rel` is not a safe relative path.
///
/// A path is considered unsafe if it is absolute or contains any `..` component,
/// either of which could write outside the intended output directory.
fn safe_join(base: &Path, rel: &str) -> Result<PathBuf> {
    let rel_path = Path::new(rel);
    for component in rel_path.components() {
        match component {
            Component::Normal(_) | Component::CurDir => {}
            Component::ParentDir => bail!("path traversal in source entry: {rel:?}"),
            Component::RootDir | Component::Prefix(_) => {
                bail!("absolute path in source entry: {rel:?}")
            }
        }
    }
    Ok(base.join(rel_path))
}

#[cfg(test)]
mod tests {
    use super::safe_join;
    use anyhow::Result;
    use std::path::Path;

    #[test]
    fn safe_join_normal() -> Result<()> {
        let base = Path::new("/tmp/out");
        let result = safe_join(base, "src/lib.rs")?;
        assert_eq!(result, base.join("src/lib.rs"));
        Ok(())
    }

    #[test]
    fn safe_join_cur_dir_component() -> Result<()> {
        // A leading "./" is acceptable (CurDir component).
        let base = Path::new("/tmp/out");
        let result = safe_join(base, "./Cargo.toml")?;
        assert_eq!(result, base.join("./Cargo.toml"));
        Ok(())
    }

    #[test]
    fn safe_join_rejects_parent_dir() {
        let err = safe_join(Path::new("/tmp/out"), "../etc/passwd").unwrap_err();
        assert!(
            err.to_string().contains("path traversal"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn safe_join_rejects_embedded_parent_dir() {
        let err = safe_join(Path::new("/tmp/out"), "src/../../etc/passwd").unwrap_err();
        assert!(
            err.to_string().contains("path traversal"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn safe_join_rejects_absolute_path() {
        let err = safe_join(Path::new("/tmp/out"), "/etc/passwd").unwrap_err();
        assert!(
            err.to_string().contains("absolute path"),
            "unexpected error: {err}"
        );
    }
}
