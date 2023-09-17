/// The last version to be used for upgrades and templates.
///
/// Should be edited every time a new version of the framework is released.
pub const LAST_VERSION: &str = "0.43.4";

/// Indicates where to stop with the upgrades.
pub const LAST_UPGRADE_VERSION: &str = LAST_VERSION;

pub const LAST_TEMPLATE_VERSION: &str = LAST_VERSION;

/// Known versions for the upgrader.
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
    "0.39.6",
    "0.39.7",
    "0.39.8",
    "0.40.0",
    "0.40.1",
    "0.41.0",
    "0.41.1",
    "0.41.2",
    "0.41.3",
    "0.42.0",
    "0.43.0",
    "0.43.1",
    "0.43.2",
    "0.43.3",
    "0.43.4",
];

/// We started supporting contract templates with version 0.43.0.
pub fn template_versions() -> &'static [&'static str] {
    &VERSIONS[33..]
}

pub fn validate_template_tag(tag: &str) -> bool {
    let versions = template_versions();
    versions.iter().any(|&tt| tt == tag)
}

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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn template_versions_test() {
        assert_eq!(template_versions()[0], "0.43.0");

        assert!(validate_template_tag("0.43.0"));
        assert!(!validate_template_tag("0.42.0"));
    }
}
