use core::fmt;
use std::cmp::Ordering;

use semver::{BuildMetadata, Prerelease, Version};

#[derive(Debug, Clone, Eq)]
pub struct FrameworkVersion {
    pub version: Version,
}

impl FrameworkVersion {
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        let version = Version {
            major,
            minor,
            patch,
            pre: Prerelease::EMPTY,
            build: BuildMetadata::EMPTY,
        };

        FrameworkVersion { version }
    }

    pub const fn from_triple(triple: (u64, u64, u64)) -> Self {
        let (major, minor, patch) = triple;
        FrameworkVersion::new(major, minor, patch)
    }

    pub fn from_string_template(version_str: &str) -> Self {
        let version_arr: Vec<&str> = version_str.split('.').collect();

        let major: u64 = version_arr[0].parse().unwrap();
        let minor: u64 = version_arr[1].parse().unwrap();
        let patch: u64 = version_arr[2].parse().unwrap();

        FrameworkVersion::new(major, minor, patch)
    }
}

impl Ord for FrameworkVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version.cmp(&other.version)
    }
}

impl PartialOrd for FrameworkVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FrameworkVersion {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

impl fmt::Display for FrameworkVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version)
    }
}

pub fn is_sorted(versions: &[FrameworkVersion]) -> bool {
    versions.windows(2).all(|window| (window[0] < window[1]))
}

#[macro_export]
macro_rules! framework_version {
    ($arg:expr) => {
        FrameworkVersion::from_triple(multiversx_sc::derive::semver_tuple!($arg))
    };
}

#[macro_export]
macro_rules! framework_versions {
    ($($arg:expr),+ $(,)?) => {
        &[$(framework_version!($arg)),+]
    };
}
