use std::{fs, path::Path};

use serde::Deserialize;

pub const CONFIG_FILE_NAME: &str = "sc-reproducible-build.toml";

/// Top-level structure of `sc-reproducible-build.toml`.
#[derive(Debug, Default, Deserialize)]
pub struct ReproducibleBuildProjectConfig {
    pub general: Option<GeneralConfig>,
    pub build: Option<BuildConfig>,
    pub publish: Vec<PublishConfig>,
}

/// The `[general]` section: settings shared across all commands (build, publish, …).
#[derive(Debug, Default, Deserialize)]
pub struct GeneralConfig {
    /// Docker image used for `sc-meta rb build` and `sc-meta rb publish`.
    #[serde(rename = "docker-image")]
    pub docker_image: Option<String>,
}

/// The `[build]` section: settings specific to `build` and `local-build`.
#[derive(Debug, Default, Deserialize)]
pub struct BuildConfig {
    /// Output directory on the host where build artifacts are written.
    pub output: Option<String>,

    /// Only build the contract with this name (as found in Cargo.toml).
    pub contract: Option<String>,

    /// Skip wasm-opt post-processing.
    #[serde(rename = "no-wasm-opt")]
    pub no_wasm_opt: Option<bool>,

    /// Wipe the output folder before building if it is not empty.
    pub overwrite: Option<bool>,

    /// Folder where the project is copied before building.
    /// For `build`: path inside the container. For `local-build`: path on the host.
    #[serde(rename = "build-root")]
    pub build_root: Option<String>,
}

/// One `[[publish]]` entry: an on-chain deployment to publish verification for.
#[derive(Debug, Deserialize)]
pub struct PublishConfig {
    pub name: String,

    /// On-chain bech32 address of the deployed contract.
    /// Also accepts `contract` as an alias for backwards compatibility.
    pub address: Option<String>,

    /// Path to the `.source.json` produced by a previous build.
    pub source: Option<String>,

    /// Verifier service URL for this deployment.
    #[serde(rename = "verifier-url")]
    pub verifier_url: Option<String>,

    /// Path to a PEM wallet file used to sign the publication request.
    pub pem: Option<String>,

    /// Path to a keystore wallet file (alternative to `pem`).
    pub keystore: Option<String>,

    /// Password for the keystore (omit to be prompted interactively).
    #[serde(rename = "keystore-password")]
    pub keystore_password: Option<String>,
}

impl ReproducibleBuildProjectConfig {
    /// Loads `sc-reproducible-build.toml` from `dir`.
    /// Returns a default empty config if the file does not exist.
    pub fn load_from_dir(dir: &Path) -> Self {
        let config_path = dir.join(CONFIG_FILE_NAME);
        if !config_path.exists() {
            return Self::default();
        }
        let contents = fs::read_to_string(&config_path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {e}", config_path.display()));
        toml::from_str(&contents)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {e}", config_path.display()))
    }
}
