use semver::{BuildMetadata, Prerelease, Version};

pub struct FrameworkVersion {
    pub version: Version,
}

impl FrameworkVersion {
    pub fn new(major: i8, minor: i16, patch: i16) -> Self {
        let version = Version {
            major: major.try_into().unwrap(),
            minor: minor.try_into().unwrap(),
            patch: patch.try_into().unwrap(),
            pre: Prerelease::EMPTY,
            build: BuildMetadata::EMPTY,
        };

        FrameworkVersion { version }
    }
}

#[macro_export]
macro_rules! known_versions {
    ($($v:literal),*)  => {
        [$(
            {
                let version: Vec<&str> = $v.split('.').collect();
                FrameworkVersion::new(version[0].parse().unwrap(),version[1].parse().unwrap(),version[2].parse().unwrap())
            }
        ),*]
    };
}
