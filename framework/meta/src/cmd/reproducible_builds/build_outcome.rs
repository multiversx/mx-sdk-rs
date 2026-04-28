use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use indexmap::IndexMap;
use serde::Serialize;

/// Mirrors the Python `BuildMetadata`, `BuildOptions`, `BuildOutcome`, and
/// `BuildOutcomeEntry` classes from `mx-sdk-rust-contract-builder`.
///
/// The resulting JSON is written to `<output>/artifacts.json`.

// ─── Metadata ────────────────────────────────────────────────────────────────

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactsBuildMetadata {
    pub version_rust: Option<String>,
    pub version_sc_tool: Option<String>,
    pub version_wasm_opt: Option<String>,
    pub target_platform: Option<String>,
}

impl ArtifactsBuildMetadata {
    /// Reads from the `BUILD_METADATA_*` env vars set inside the Docker image.
    /// Falls back to running the tools directly for local (non-Docker) builds.
    pub fn detect() -> Self {
        ArtifactsBuildMetadata {
            version_rust: env::var("BUILD_METADATA_VERSION_RUST")
                .ok()
                .or_else(detect_rustc_version),
            version_sc_tool: env::var("BUILD_METADATA_VERSION_SC_META")
                .ok()
                .or_else(|| Some(env!("CARGO_PKG_VERSION").to_string())),
            version_wasm_opt: env::var("BUILD_METADATA_VERSION_WASM_OPT")
                .ok()
                .or_else(detect_wasm_opt_version),
            target_platform: env::var("BUILD_METADATA_TARGETPLATFORM")
                .ok()
                .or_else(detect_target_platform),
        }
    }
}

// ─── Options ─────────────────────────────────────────────────────────────────

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactsBuildOptions {
    /// Kept for compatibility with the Python builder output.
    pub package_whole_project_src: bool,
    pub specific_contract: Option<String>,
    pub no_wasm_opt: bool,
    pub build_root_folder: String,
}

// ─── Per-contract artifacts ───────────────────────────────────────────────────

#[derive(Serialize)]
pub struct ContractArtifactFiles {
    pub bytecode: String,
    pub abi: String,
    #[serde(rename = "srcPackage")]
    pub src_package: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractOutcomeEntry {
    pub version: String,
    pub codehash: String,
    pub artifacts: ContractArtifactFiles,
}

// ─── Top-level outcome ────────────────────────────────────────────────────────

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildOutcome {
    pub build_metadata: ArtifactsBuildMetadata,
    pub build_options: ArtifactsBuildOptions,
    /// Contracts in path-sorted insertion order, matching the Python builder output.
    pub contracts: IndexMap<String, ContractOutcomeEntry>,
}

impl BuildOutcome {
    pub fn new(metadata: ArtifactsBuildMetadata, options: ArtifactsBuildOptions) -> Self {
        BuildOutcome {
            build_metadata: metadata,
            build_options: options,
            contracts: IndexMap::new(),
        }
    }

    /// Scans `output_subfolder` (the per-contract output dir) for build
    /// artifacts and adds an entry for each `.wasm` file found.
    ///
    /// `build_folder` is the contract directory in the build root, used to
    /// read the version from `Cargo.toml`.
    pub fn gather(&mut self, contract_name: &str, output_subfolder: &Path) {
        let wasm_files = glob_files(output_subfolder, ".wasm");
        for wasm in wasm_files {
            let stem = wasm
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();

            let codehash = read_codehash(output_subfolder, &stem);
            let abi = find_file_by_suffix(output_subfolder, ".abi.json")
                .unwrap_or_else(|| format!("{stem}.abi.json"));
            let src_package = find_file_by_suffix(output_subfolder, ".source.json")
                .unwrap_or_else(|| format!("{stem}.source.json"));

            self.contracts.insert(
                stem,
                ContractOutcomeEntry {
                    version: read_contract_version(output_subfolder, contract_name),
                    codehash,
                    artifacts: ContractArtifactFiles {
                        bytecode: wasm
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned(),
                        abi,
                        src_package,
                    },
                },
            );
        }
    }

    pub fn save(&self, output_folder: &Path) {
        let path = output_folder.join("artifacts.json");
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        self.serialize(&mut ser).unwrap();
        fs::write(&path, &buf).unwrap();
        println!("Artifacts summary: {}", path.display());
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn glob_files(dir: &Path, suffix: &str) -> Vec<PathBuf> {
    let Ok(rd) = fs::read_dir(dir) else {
        return vec![];
    };
    let mut files: Vec<PathBuf> = rd
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.to_string_lossy().ends_with(suffix))
        .collect();
    files.sort();
    files
}

fn find_file_by_suffix(dir: &Path, suffix: &str) -> Option<String> {
    glob_files(dir, suffix)
        .into_iter()
        .next()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
}

fn read_codehash(output_subfolder: &Path, stem: &str) -> String {
    let path = output_subfolder.join(format!("{stem}.codehash.txt"));
    fs::read_to_string(&path)
        .unwrap_or_default()
        .trim()
        .to_string()
}

/// Tries to read the version from the `.source.json` (it has `contractVersion`
/// in its metadata). Falls back to an empty string so a missing file is not fatal.
fn read_contract_version(output_subfolder: &Path, _contract_name: &str) -> String {
    let Some(src_json_path) = glob_files(output_subfolder, ".source.json")
        .into_iter()
        .next()
    else {
        return String::new();
    };
    let Ok(text) = fs::read_to_string(&src_json_path) else {
        return String::new();
    };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) else {
        return String::new();
    };
    v["metadata"]["contractVersion"]
        .as_str()
        .unwrap_or("")
        .to_string()
}

fn run_tool_version(program: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(program).args(args).output().ok()?;
    if out.status.success() {
        let raw = String::from_utf8_lossy(&out.stdout).trim().to_string();
        Some(raw)
    } else {
        None
    }
}

fn detect_rustc_version() -> Option<String> {
    // `rustc --version` → "rustc 1.93.0 (... ...)"
    run_tool_version("rustc", &["--version"])
        .map(|s| s.split_whitespace().nth(1).unwrap_or(&s).to_string())
}

fn detect_wasm_opt_version() -> Option<String> {
    // `wasm-opt --version` → "wasm-opt version 116 (... ...)" or similar
    run_tool_version("wasm-opt", &["--version"])
}

fn detect_target_platform() -> Option<String> {
    // `rustc -vV` includes "host: x86_64-unknown-linux-gnu"
    let out = run_tool_version("rustc", &["-vV"])?;
    for line in out.lines() {
        if let Some(host) = line.strip_prefix("host: ") {
            return Some(host.trim().to_string());
        }
    }
    None
}
