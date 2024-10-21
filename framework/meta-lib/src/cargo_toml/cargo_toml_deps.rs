use crate::version::FrameworkVersion;

use super::VersionReq;


/// A dependency reference to a git commit. We mostly use git commits when referencing git.
///
/// TODO: add support for `branch` and `tag`.
#[derive(Debug, Clone)]
pub struct GitReference {
    pub git: String,
    pub rev: String,
}

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
