use crate::{
    version::FrameworkVersion,
    version_history::{find_version_by_str, LAST_VERSION},
};

/// Models how a dependency is expressed in Cargo.toml.
#[derive(Debug, Clone)]
pub enum DependencyReference {
    Version(VersionReq),
    Git(GitReference),
    Path(String),
}

impl DependencyReference {
    pub fn is_framework_version(&self, version: &FrameworkVersion) -> bool {
        if let DependencyReference::Version(version_req) = self {
            &version_req.semver == version
        } else {
            false
        }
    }
}

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

/// A dependency reference to a git commit. We mostly use git commits when referencing git.
///
/// TODO: add support for `branch` and `tag`.
#[derive(Debug, Clone)]
pub struct GitReference {
    pub git: String,
    pub rev: String,
}
