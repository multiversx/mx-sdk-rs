use semver::{BuildMetadata, Prerelease, Version};

pub struct FrameworkVersion {
    pub version: Version,
}

impl FrameworkVersion {
    pub fn new(version_bytes: &[u8]) -> Self {
        let version_str = String::from_utf8_lossy(version_bytes).to_string();
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
}

#[macro_export]
macro_rules! sc_version {
    ($($arg:expr),+ $(,)?) => {
        multiversx_sc::derive::format_version!($($arg),+);
    };
}
