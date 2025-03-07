use std::path::PathBuf;

use crate::version::FrameworkVersion;

use super::{DependencyRawValue, VersionReq};

/// A dependency reference to a git commit. We mostly use git commits when referencing git.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitCommitReference {
    pub git: String,
    pub rev: String,
}

/// A dependency reference to a git branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitBranchReference {
    pub git: String,
    pub branch: String,
}

/// A dependency reference to a git tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitTagReference {
    pub git: String,
    pub tag: String,
}

/// Models how a dependency is expressed in Cargo.toml.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyReference {
    Version(VersionReq),
    GitCommit(GitCommitReference),
    GitBranch(GitBranchReference),
    GitTag(GitTagReference),
    Path(PathBuf),
    Unsupported(String),
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
            return match (self.rev, self.branch, self.tag) {
                (Some(rev), None, None) => {
                    DependencyReference::GitCommit(GitCommitReference { git, rev })
                },
                (None, Some(branch), None) => {
                    DependencyReference::GitBranch(GitBranchReference { git, branch })
                },

                (None, None, Some(tag)) => {
                    DependencyReference::GitTag(GitTagReference { git, tag })
                },

                (None, None, None) => DependencyReference::Unsupported(
                    "need at least one of: git commit, git brach, or git tag".to_owned(),
                ),
                _ => DependencyReference::Unsupported(
                    "can only have one of: git commit, git brach, or git tag".to_owned(),
                ),
            };
        }

        // explicit version = "..."
        // handled last, because it has the lowest priority, both path and git fields override it
        if let Some(version) = self.version {
            if let Some(version_req) = VersionReq::from_version_str(&version) {
                return DependencyReference::Version(version_req);
            } else {
                return DependencyReference::Unsupported(format!(
                    "unknown framework version: {version}"
                ));
            }
        }

        DependencyReference::Unsupported("expected at least one of: version, git, path".to_owned())
    }
}
