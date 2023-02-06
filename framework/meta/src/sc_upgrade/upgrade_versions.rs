/// Not necessarily the last entry in `VERSIONS`.
///
/// Indicates where to stop with the upgrades.
pub const DEFAULT_LAST_VERSION: &str = "0.39.5";

/// Known version for the upgrader.
#[rustfmt::skip]
pub const VERSIONS: &[&str] = &[
    "0.28.0",
    "0.29.0",
    "0.29.2",
    "0.29.3",
    "0.30.0",
    "0.31.0",
    "0.31.1",
    "0.32.0",
    "0.33.0",
    "0.33.1",
    "0.34.0",
    "0.34.1",
    "0.35.0",
    "0.36.0",
    "0.36.1",
    "0.37.0",
    "0.38.0",
    "0.39.0",
    "0.39.1",
    "0.39.2",
    "0.39.3",
    "0.39.4",
    "0.39.5",
];

pub struct VersionIterator {
    next_version: usize,
    last_version: String,
}

impl VersionIterator {
    fn is_last_version(&self, version: &str) -> bool {
        self.last_version == version
    }
}

impl Iterator for VersionIterator {
    type Item = (&'static str, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_version > 0 && self.next_version < VERSIONS.len() {
            let from_version = VERSIONS[self.next_version - 1];

            if self.is_last_version(from_version) {
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

pub fn versions_iter(last_version: String) -> VersionIterator {
    VersionIterator {
        next_version: 1,
        last_version,
    }
}
