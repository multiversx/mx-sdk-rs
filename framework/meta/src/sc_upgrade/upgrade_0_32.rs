use super::upgrade_common::{replace_in_files, version_bump_in_cargo_toml};
use crate::folder_structure::RelevantDirectory;
use ruplacer::Query;
use std::path::Path;

/// Migrate `0.30` to `0.31.0`, including the version bump.
pub fn upgrade_to_32_0(dir: &RelevantDirectory) {
    v_0_32_replace_in_files(dir.path.as_ref());

    let (from_version, to_version) = dir.upgrade_in_progress.unwrap();
    version_bump_in_cargo_toml(&dir.path, from_version, to_version);
}

fn v_0_32_replace_in_files(sc_crate_path: &Path) {
    replace_in_files(
        sc_crate_path,
        "*rs",
        &[Query::substring(
            "TokenIdentifier::egld()",
            "EgldOrEsdtTokenIdentifier::egld()",
        )][..],
    );
}
