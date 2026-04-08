use super::upgrade_common::{replace_in_files, version_bump_in_cargo_toml};
use crate::folder_structure::RelevantDirectory;
use regex::Regex;
use ruplacer::Query;
use std::path::Path;

/// Migrate `0.58` to `0.59.0`, including the version bump.
pub fn upgrade_to_59_0(dir: &RelevantDirectory) {
    v_0_59_replace_in_files(dir.path.as_ref());

    let (from_version, to_version) = dir.upgrade_in_progress.clone().unwrap();
    version_bump_in_cargo_toml(&dir.path, &from_version, &to_version);
}

fn v_0_59_replace_in_files(sc_crate_path: &Path) {
    replace_in_files(
        sc_crate_path,
        "*rs",
        &[
            Query::regex(
                Regex::new(r"\bReturnsBackTransfers\b").unwrap(),
                "ReturnsBackTransfersLegacy",
            ),
            Query::regex(
                Regex::new(r"\bReturnsBackTransfersReset\b").unwrap(),
                "ReturnsBackTransfersLegacyReset",
            ),
            Query::regex(
                Regex::new(r"\bReturnsBackTransfersMultiESDT\b").unwrap(),
                "ReturnsBackTransfersLegacyMultiESDT",
            ),
        ][..],
    );
}
