use std::process::Command;

use multiversx_sc::abi::RustcAbi;
use rustc_version::VersionMeta;
use semver::Version;

/// Contains a representation of a Rust compiler (toolchain) version.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustcVersion {
    version_meta: VersionMeta,
    short_string: String,
}

impl RustcVersion {
    /// Parses the rustc version from sc-config.toml.
    pub fn from_opt_sc_config_serde(opt_serde_version: &Option<String>) -> Self {
        if let Some(serde_version) = opt_serde_version {
            Self::from_sc_config_serde(serde_version)
        } else {
            Self::current_version()
        }
    }

    pub fn from_sc_config_serde(serde_version: &str) -> Self {
        let version_meta = get_version_meta_for_toolchain(serde_version);
        RustcVersion {
            version_meta,
            short_string: serde_version.to_owned(),
        }
    }

    /// Retrieves the current rustc version from crate `rustc_version`.
    ///
    /// The value is embedded into the binary at compile time.
    pub fn current_version() -> RustcVersion {
        let version_meta =
            rustc_version::version_meta().expect("failed to get rustc version metadata");
        let short_string = rustc_version_to_string(&version_meta);
        RustcVersion {
            version_meta,
            short_string,
        }
    }

    /// Formats as a CLI for cargo or rustup, e.g. `cargo +1.88 build`.
    pub fn to_cli_arg(&self) -> String {
        format!("+{}", self.short_string)
    }

    pub fn to_abi(&self) -> RustcAbi {
        RustcAbi {
            version: version_to_string(&self.version_meta.semver),
            commit_hash: self.version_meta.commit_hash.clone().unwrap_or_default(),
            commit_date: self.version_meta.commit_date.clone().unwrap_or_default(),
            channel: format!("{:?}", self.version_meta.channel),
            host: self.version_meta.host.clone(),
            short: self.version_meta.short_version_string.clone(),
        }
    }
}

impl std::fmt::Display for RustcVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.short_string)
    }
}

fn rustc_version_to_string(version_meta: &VersionMeta) -> String {
    match version_meta.channel {
        rustc_version::Channel::Stable => format!(
            "{}-{}",
            version_to_string(&version_meta.semver),
            version_meta.host
        ),
        rustc_version::Channel::Nightly => {
            if let Some(build_date) = &version_meta.build_date {
                format!("nightly-{}-{}", build_date, version_meta.host)
            } else {
                "nightly".to_owned()
            }
        }
        _ => panic!("only stable and nightly supported"),
    }
}

/// Outputs major.minor if the other fields are zero or missing. Outputs the full string otherwise.
fn version_to_string(version: &Version) -> String {
    if version.patch == 0 && version.pre.is_empty() && version.build.is_empty() {
        format!("{}.{}", version.major, version.minor)
    } else {
        version.to_string()
    }
}

/// Gets the VersionMeta for a specific toolchain identifier, by calling `rustc -vV` with that toolchain.
fn get_version_meta_for_toolchain(toolchain: &str) -> VersionMeta {
    // Run rustc with the specific toolchain
    let output = Command::new("rustc")
        .arg(format!("+{}", toolchain))
        .arg("-vV")
        .output()
        .expect("failed to call rustc to get full toolchain info");

    if !output.status.success() {
        panic!(
            "rustc -vV failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Parse the version string into VersionMeta
    let version_string =
        String::from_utf8(output.stdout).expect("failed to parse rustc -vV output as UTF-8");
    rustc_version::version_meta_for(&version_string)
        .unwrap_or_else(|_| panic!("failed to parse rustc -vV output: {version_string}"))
}
