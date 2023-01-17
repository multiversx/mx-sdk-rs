/// Used for retrieving crate versions.
pub const FRAMEWORK_CRATE_NAMES: &[&str] = &[
    "multiversx-sc",
    "multiversx-sc-scenario",
    "multiversx-sc-meta",
    "multiversx-sc-modules",
    "elrond-wasm",
    "elrond-wasm-debug",
    "elrond-wasm-modules",
    "elrond-wasm-node",
    "elrond-interact-snippets",
];

/// Not necessarily the last entry in `VERSIONS`.
///
/// Indicates where to stop with the upgrades.
pub const LAST_VERSION: &str = "0.39.0";

#[rustfmt::skip]
pub const VERSIONS: &[&str] = &[
    "0.38.0",
    "0.39.0",
    "0.39.1",
];

pub struct VersionIterator {
    next_version: usize,
}

impl Iterator for VersionIterator {
    type Item = (&'static str, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_version > 0 && self.next_version < VERSIONS.len() {
            let from_version = VERSIONS[self.next_version - 1];
            if from_version == LAST_VERSION {
                None
            } else {
                let to_version = VERSIONS[self.next_version];
                let result = (from_version, to_version);
                self.next_version += 1;
                Some(result)
            }
        } else {
            None
        }
    }
}

pub fn iter_from_version(from_version: &str) -> Option<VersionIterator> {
    for (version_index, &version) in VERSIONS.iter().enumerate() {
        if version == from_version {
            return Some(VersionIterator {
                next_version: version_index + 1,
            });
        }
    }

    None
}
