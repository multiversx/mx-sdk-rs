use semver::{BuildMetadata, Prerelease, Version};

pub struct FrameworkVersion {
    pub version: Version,
}

impl FrameworkVersion {
    pub fn parse(version_str: &str) -> Self {
        let version_arr: Vec<&str> = version_str.split('.').collect();

        let version = Version {
            major: version_arr[0].parse().unwrap(),
            minor: version_arr[1].parse().unwrap(),
            patch: version_arr[2].parse().unwrap(),
            pre: Prerelease::EMPTY,
            build: BuildMetadata::EMPTY,
        };

        FrameworkVersion { version }
    }

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
}



// #[macro_use]
macro_rules! framework_version {
    ($arg:expr) => {
        FrameworkVersion::from_triple(multiversx_sc::derive::version_triple!($arg))
    };
}

// #[macro_use]
macro_rules! framework_versions {
    ($($arg:expr),+ $(,)?) => {
        &[$(framework_version!($arg)),+]
    };
}

const V123: FrameworkVersion = framework_version!(1.2.3);
const ALL: &[FrameworkVersion] = framework_versions!(1.2.3, 3.4.5);


