use crate::{framework_version, framework_versions, version::FrameworkVersion};

/// The last version to be used for upgrades and templates.
///
/// Should be edited every time a new version of the framework is released.
pub const LAST_VERSION: FrameworkVersion = framework_version!(0.46.1);

/// Indicates where to stop with the upgrades.
pub const LAST_UPGRADE_VERSION: FrameworkVersion = LAST_VERSION;

pub const LAST_TEMPLATE_VERSION: FrameworkVersion = framework_version!(0.45.2);

#[rustfmt::skip]
/// Known versions for the upgrader.
pub const VERSIONS: &[FrameworkVersion] = framework_versions![
    0.28.0, 
    0.29.0, 
    0.29.2, 
    0.29.3, 
    0.30.0, 
    0.31.0, 
    0.31.1, 
    0.32.0, 
    0.33.0, 
    0.33.1, 
    0.34.0, 
    0.34.1,
    0.35.0, 
    0.36.0, 
    0.36.1, 
    0.37.0, 
    0.38.0, 
    0.39.0, 
    0.39.1, 
    0.39.2, 
    0.39.3, 
    0.39.4, 
    0.39.5, 
    0.39.6,
    0.39.7, 
    0.39.8, 
    0.40.0, 
    0.40.1, 
    0.41.0, 
    0.41.1, 
    0.41.2, 
    0.41.3, 
    0.42.0, 
    0.43.0, 
    0.43.1, 
    0.43.2,
    0.43.3, 
    0.43.4, 
    0.43.5, 
    0.44.0, 
    0.45.0, 
    0.45.2,
    0.46.0,
    0.46.1,
];

pub const LOWER_VERSION_WITH_TEMPLATE_TAG: &FrameworkVersion = &VERSIONS[33];
pub const TEMPLATE_VERSIONS_WITH_AUTOGENERATED_WASM: &FrameworkVersion = &VERSIONS[40];
pub const TEMPLATE_VERSIONS_WITH_AUTOGENERATED_JSON: &FrameworkVersion = &VERSIONS[39];

/// We started supporting contract templates with version 0.43.0.
pub fn validate_template_tag(tag_str: &str) -> bool {
    let tag: FrameworkVersion = FrameworkVersion::from_string_template(tag_str);

    tag >= *LOWER_VERSION_WITH_TEMPLATE_TAG && tag <= LAST_VERSION
}

pub fn is_template_with_autogenerated_wasm(tag: FrameworkVersion) -> bool {
    tag >= *TEMPLATE_VERSIONS_WITH_AUTOGENERATED_WASM
}

pub fn is_template_with_autogenerated_json(tag: FrameworkVersion) -> bool {
    tag >= *TEMPLATE_VERSIONS_WITH_AUTOGENERATED_JSON
}

pub fn find_version_by_str(tag: &str) -> Option<&FrameworkVersion> {
    VERSIONS.iter().find(|&v| v.version.to_string() == tag)
}

pub struct VersionIterator {
    next_version: usize,
    last_version: FrameworkVersion,
}

impl VersionIterator {
    fn is_last_version(&self, version: &FrameworkVersion) -> bool {
        self.last_version == *version
    }
}

impl Iterator for VersionIterator {
    type Item = (&'static FrameworkVersion, &'static FrameworkVersion);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_version > 0 && self.next_version < VERSIONS.len() {
            let from_version = &VERSIONS[self.next_version - 1];

            if self.is_last_version(from_version) {
                None
            } else {
                let to_version = &VERSIONS[self.next_version];
                let result = (from_version, to_version);
                self.next_version += 1;
                Some(result)
            }
        } else {
            None
        }
    }
}

pub fn versions_iter(last_version: FrameworkVersion) -> VersionIterator {
    VersionIterator {
        next_version: 1,
        last_version,
    }
}

#[cfg(test)]
pub mod tests {

    use crate::version::is_sorted;

    use super::*;

    #[test]
    fn compare_versions_test() {
        let f1: FrameworkVersion = framework_version!(0.44.0);
        let f2: FrameworkVersion = framework_version!(0.41.2);

        assert!(f1 > f2);
    }

    #[test]
    fn template_versions_test() {
        assert!(validate_template_tag("0.43.0"));
        assert!(!validate_template_tag("0.42.0"));
        assert!(!validate_template_tag("0.47.0"));
    }

    #[test]
    fn template_versions_with_autogenerated_wasm_test() {
        assert!(is_template_with_autogenerated_wasm(framework_version!(
            0.45.0
        )));
        assert!(!is_template_with_autogenerated_wasm(framework_version!(
            0.44.0
        )));
    }

    #[test]
    fn template_versions_with_autogenerated_json_test() {
        assert!(is_template_with_autogenerated_json(framework_version!(
            0.44.0
        )));
        assert!(!is_template_with_autogenerated_json(framework_version!(
            0.43.0
        )));
    }

    #[test]
    fn find_version_by_str_test() {
        let version = find_version_by_str("0.28.0");
        match version {
            Some(v) => assert_eq!(VERSIONS[0], *v),
            None => unreachable!(),
        }
    }

    #[test]
    fn framework_version_test() {
        assert!(is_sorted(VERSIONS));
    }
}
