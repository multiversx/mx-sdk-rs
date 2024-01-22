use crate::{
    version::FrameworkVersion,
    version_history::{find_version_by_str, LAST_VERSION},
};

/// Crate version requirements, as expressed in Cargo.toml. A very crude version.
///
/// TODO: replace with semver::VersionReq at some point.
#[derive(Debug, Clone)]
pub struct VersionReq {
    pub semver: FrameworkVersion,
    pub is_strict: bool,
}
impl VersionReq {
    pub fn from_string(raw: String) -> Self {
        if let Some(stripped_version) = raw.strip_prefix('=') {
            VersionReq {
                semver: find_version_by_str(stripped_version)
                    .unwrap_or(&LAST_VERSION)
                    .clone(),
                is_strict: true,
            }
        } else {
            VersionReq {
                semver: find_version_by_str(&raw).unwrap_or(&LAST_VERSION).clone(),
                is_strict: false,
            }
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
