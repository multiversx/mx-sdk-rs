use crate::{
    version::FrameworkVersion,
    version_history::{LAST_VERSION, find_version_by_str},
};

/// Crate version requirements, as expressed in Cargo.toml. A very crude version.
///
/// TODO: replace with semver::VersionReq at some point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionReq {
    pub semver: FrameworkVersion,
    pub is_strict: bool,
}
impl VersionReq {
    pub fn from_version_str(raw: &str) -> Option<Self> {
        if let Some(stripped_version) = raw.strip_prefix('=') {
            Some(VersionReq {
                semver: find_version_by_str(stripped_version)?.clone(),
                is_strict: true,
            })
        } else {
            Some(VersionReq {
                semver: find_version_by_str(raw)?.clone(),
                is_strict: false,
            })
        }
    }

    pub fn from_version_str_or_latest(raw: &str) -> Self {
        if let Some(stripped_version) = raw.strip_prefix('=') {
            VersionReq {
                semver: find_version_by_str(stripped_version)
                    .unwrap_or(&LAST_VERSION)
                    .clone(),
                is_strict: true,
            }
        } else {
            VersionReq {
                semver: find_version_by_str(raw).unwrap_or(&LAST_VERSION).clone(),
                is_strict: false,
            }
        }
    }

    pub fn strict(self) -> Self {
        Self {
            semver: self.semver,
            is_strict: true,
        }
    }

    pub fn into_string(self) -> String {
        if self.is_strict {
            format!("={}", self.semver)
        } else {
            self.semver.to_string()
        }
    }
}
