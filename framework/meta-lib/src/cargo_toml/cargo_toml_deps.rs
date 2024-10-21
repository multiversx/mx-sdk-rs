use crate::version::FrameworkVersion;

use super::{DependencyRawValue, VersionReq};

/// A dependency reference to a git commit. We mostly use git commits when referencing git.
///
/// TODO: add support for `branch` and `tag`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitCommitReference {
    pub git: String,
    pub rev: String,
}

/// Models how a dependency is expressed in Cargo.toml.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyReference {
    Version(VersionReq),
    GitCommit(GitCommitReference),
    Path(String),
    Unsupported,
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

impl DependencyRawValue {
    /// Interprets the raw dependency value as one of several possible formats.
    pub fn interpret(self) -> DependencyReference {
        // path is top priority
        if let Some(path) = self.path {
            return DependencyReference::Path(path);
        }

        if let Some(git) = self.git {
            let rev = self.rev.unwrap_or_default();
            return DependencyReference::GitCommit(GitCommitReference {
                git: git.clone(),
                rev: rev.to_owned(),
            });
        }

        // explicit version = "..."
        // handled last, because it has the lowest priority, both path and git fields override it
        if let Some(version) = self.version {
            return DependencyReference::Version(VersionReq::from_version_str(&version));
        }

        DependencyReference::Unsupported
    }
}
