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
    last_version: Option<String>,
}

impl VersionIterator {
    fn is_last_version(&self, version: &str) -> bool {
        if let Some(last_version) = &self.last_version {
            last_version == version
        } else {
            false
        }
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

pub fn iter_from_version(
    from_version: &str,
    last_version: Option<String>,
) -> Option<VersionIterator> {
    for (version_index, &version) in VERSIONS.iter().enumerate() {
        if version == from_version {
            return Some(VersionIterator {
                next_version: version_index + 1,
                last_version,
            });
        }
    }

    None
}
