use std::{process::Command, str::FromStr};

use multiversx_sc::abi::RustcAbi;
use rustc_version::{LlvmVersion, VersionMeta};
use semver::Version;

/// Contains a representation of a Rust compiler (toolchain) version.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustcVersion {
    pub version_meta: VersionMeta,
    pub short_string: String,
}

impl RustcVersion {
    /// Constructs a `RustcVersion` from an optional toolchain name.
    ///
    /// If `Some`, delegates to [`Self::from_toolchain`].
    /// If `None`, falls back to [`Self::current_version`], which reflects the current default toolchain.
    pub fn from_opt_toolchain(opt_toolchain_name: Option<&str>) -> Self {
        if let Some(toolchain_name) = opt_toolchain_name {
            Self::from_toolchain(toolchain_name)
        } else {
            Self::current_version()
        }
    }

    /// Constructs a `RustcVersion` from a toolchain name (e.g. `"nightly-2024-01-01"` or `"stable"`).
    ///
    /// The name is passed verbatim as the `+<toolchain>` argument to `rustc -vV`
    /// (i.e. `rustc +nightly-2024-01-01 -vV`), which rustup intercepts to select the
    /// appropriate installed toolchain. The same string is stored in `short_string` and
    /// later used to build `+<toolchain>` arguments for `cargo` and `rustup` commands.
    ///
    /// Panics if the toolchain is not installed or if `rustc -vV` output cannot be parsed.
    pub fn from_toolchain(toolchain_name: &str) -> Self {
        let version_meta = get_version_meta_for_toolchain(toolchain_name);
        RustcVersion {
            version_meta,
            short_string: toolchain_name.to_owned(),
        }
    }

    /// Retrieves the version of the currently active `rustc` by running `rustc -vV` at runtime.
    ///
    /// Delegates to [`rustc_version::version_meta`], which invokes the binary pointed to by the
    /// `$RUSTC` environment variable (falling back to `rustc` if unset), and additionally
    /// respects `$RUSTC_WRAPPER`. No toolchain override (`+<name>`) is applied, so the result
    /// reflects whichever toolchain is currently active in the shell environment.
    ///
    /// Unlike [`Self::from_toolchain`], which stores the caller-supplied name verbatim as
    /// `short_string`, this method derives `short_string` from the parsed output and
    /// includes the host triple (e.g. `"1.88-x86_64-unknown-linux-gnu"` for stable,
    /// `"nightly-2024-01-01-x86_64-unknown-linux-gnu"` for nightly).
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
            version: self.version_meta.semver.to_string(),
            commit_hash: self.version_meta.commit_hash.clone().unwrap_or_default(),
            commit_date: self.version_meta.commit_date.clone().unwrap_or_default(),
            build_date: self.version_meta.build_date.clone(),
            channel: format!("{:?}", self.version_meta.channel),
            host: self.version_meta.host.clone(),
            short: self.version_meta.short_version_string.clone(),
            llvm_version: self
                .version_meta
                .llvm_version
                .clone()
                .map(|llvm_version| llvm_version.to_string()),
        }
    }

    pub fn from_abi(abi: &RustcAbi) -> Self {
        let semver = Version::parse(&abi.version).expect("failed to parse version");
        let channel = match abi.channel.as_str() {
            "Stable" => rustc_version::Channel::Stable,
            "Nightly" => rustc_version::Channel::Nightly,
            _ => panic!("unsupported channel: {}", abi.channel),
        };

        RustcVersion {
            version_meta: VersionMeta {
                semver,
                channel,
                commit_hash: if abi.commit_hash.is_empty() {
                    None
                } else {
                    Some(abi.commit_hash.clone())
                },
                commit_date: if abi.commit_date.is_empty() {
                    None
                } else {
                    Some(abi.commit_date.clone())
                },
                build_date: abi.build_date.clone(),
                host: abi.host.clone(),
                short_version_string: abi.short.clone(),
                llvm_version: abi.llvm_version.clone().map(|llvm_version_string| {
                    LlvmVersion::from_str(&llvm_version_string)
                        .expect("failed to parse LLVM version")
                }),
            },
            short_string: abi.short.clone(),
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
