use rustc_version::{version_meta, VersionMeta};

/// Contains a representation of a Rust compiler (toolchain) version.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustcVersion(String);

impl RustcVersion {
    /// Parses the rustc version from sc-config.toml.
    pub fn from_sc_config_serde(opt_serde_version: &Option<String>) -> Self {
        if let Some(serde_version) = opt_serde_version {
            RustcVersion(serde_version.clone())
        } else {
            RustcVersion::current_version()
        }
    }

    /// Retrieves the current rustc version from crate `rustc_version`.
    ///
    /// The value is embedded into the binray at compile time.
    pub fn current_version() -> RustcVersion {
        let version_meta = version_meta().expect("failed to get rustc version metadata");
        RustcVersion(rustc_version_to_string(&version_meta))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Formats as a CLI for cargo or rustup, e.g. `cargo +1.88 build`.
    pub fn to_cli_arg(&self) -> String {
        format!("+{}", self.0)
    }
}

impl std::fmt::Display for RustcVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn rustc_version_to_string(version_meta: &VersionMeta) -> String {
    match version_meta.channel {
        rustc_version::Channel::Stable => version_meta.semver.to_string(),
        rustc_version::Channel::Nightly => {
            if let Some(build_date) = &version_meta.build_date {
                format!("nightly-{}", build_date)
            } else {
                "nightly".to_owned()
            }
        }
        _ => panic!("only stable and nightly supported"),
    }
}
