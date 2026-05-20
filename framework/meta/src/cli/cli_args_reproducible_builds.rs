use clap::{Args, Subcommand};

use super::cli_args_standalone::LocalDepsArgs;

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ReproducibleBuildArgs {
    #[command(subcommand)]
    pub command: ReproducibleBuildCliAction,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum ReproducibleBuildCliAction {
    #[command(
        name = "build",
        about = "Runs the reproducible build inside a pinned Docker container."
    )]
    Build(ReproducibleBuildBuildArgs),

    #[command(
        name = "local-build",
        about = "Builds all contracts locally, mirroring the Docker reproducible build pipeline."
    )]
    LocalBuild(ReproducibleBuildLocalBuildArgs),

    #[command(
        name = "local-deps",
        about = "Generates a report on the local dependencies of the contract."
    )]
    LocalDeps(LocalDepsArgs),

    #[command(
        name = "publish",
        about = "Submits a contract publication request to the verifier service."
    )]
    Publish(ReproducibleBuildPublishArgs),

    #[command(
        name = "unpublish",
        about = "Removes a previously published Smart Contract from the verifier service."
    )]
    Unpublish(ReproducibleBuildUnpublishArgs),

    #[command(
        name = "check",
        about = "Checks whether a contract is currently verified on the verifier service."
    )]
    Check(ReproducibleBuildCheckArgs),

    #[command(
        name = "download",
        about = "Downloads the ABI and source files of a verified contract from the verifier service."
    )]
    Download(ReproducibleBuildDownloadArgs),

    #[command(
        name = "source-pack",
        about = "Packages the contract source code into a self-contained JSON file, suitable for reproducible builds."
    )]
    SourcePack(SourcePackArgs),

    #[command(
        name = "source-unpack",
        about = "Unpacks a .source.json file produced by a previous build back to the filesystem."
    )]
    SourceUnpack(SourceUnpackArgs),

    #[command(
        name = "init-config",
        about = "Creates a default sc-reproducible-build.toml in the current (or specified) directory."
    )]
    InitConfig(InitConfigArgs),

    #[command(
        name = "release-notes",
        about = "Generates a Markdown release-notes fragment from an artifacts.json produced by a previous build."
    )]
    ReleaseNotes(ReleaseNotesArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct InitConfigArgs {
    /// Directory where sc-reproducible-build.toml will be written.
    /// Defaults to the current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// Overwrite the file if it already exists.
    #[arg(long, default_value = "false", verbatim_doc_comment)]
    pub overwrite: bool,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct ReleaseNotesArgs {
    /// Path to the artifacts.json file produced by a previous reproducible build.
    /// Defaults to "artifacts.json" in the current directory.
    #[arg(long, default_value = "artifacts.json", verbatim_doc_comment)]
    pub artifacts: String,

    /// Docker image name (including tag) used for the build, e.g.
    /// "multiversx/sdk-rust-contract-builder:v8.0.0".
    /// When provided, a "Built using Docker image" header line is emitted.
    #[arg(long, verbatim_doc_comment)]
    pub docker_image: Option<String>,

    /// Write the release notes to this file instead of stdout.
    #[arg(long, verbatim_doc_comment)]
    pub output: Option<String>,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ReproducibleBuildLocalBuildArgs {
    /// Project folder (workspace root or single contract folder).
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// Output folder where build artifacts and source JSON files will be placed.
    /// A subfolder per contract name will be created inside it.
    /// If not specified, taken from [general] output in sc-reproducible-build.toml.
    #[arg(long, verbatim_doc_comment)]
    pub output: Option<String>,

    /// Cargo target directory for compilation.
    /// Defaults to /tmp/sc-target if not specified.
    #[arg(long = "target-dir", verbatim_doc_comment)]
    pub target_dir: Option<String>,

    /// Folder where the project will be copied before building.
    /// Defaults to /tmp/sc-build if not specified.
    #[arg(long = "build-root", verbatim_doc_comment)]
    pub build_root: Option<String>,

    /// Only build the contract with this name (as found in Cargo.toml).
    /// If not specified, all contracts under the project folder are built.
    #[arg(long, verbatim_doc_comment)]
    pub contract: Option<String>,

    /// Do not optimize wasm files after the build.
    #[arg(long = "no-wasm-opt", default_value = "false", verbatim_doc_comment)]
    pub no_wasm_opt: bool,

    /// If the output folder is not empty, wipe it before building instead of aborting.
    #[arg(long, default_value = "false", verbatim_doc_comment)]
    pub overwrite: bool,

    /// Path to a `.source.json` file produced by a previous build.
    /// When set, the source is unpacked to /tmp/unwrapped/ and the build
    /// proceeds from there, reproducing the original layout exactly.
    /// Mutually exclusive with --path.
    #[arg(long = "packaged-src", verbatim_doc_comment)]
    pub packaged_src: Option<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct SourcePackArgs {
    /// Project folder (workspace root or single contract folder).
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// Only pack the contract with this name (as found in Cargo.toml).
    /// If not specified, all contracts under the project folder are packed.
    #[arg(long, verbatim_doc_comment)]
    pub contract: Option<String>,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct SourceUnpackArgs {
    /// Path to the `.source.json` file to unpack.
    #[arg(long = "packaged-src", verbatim_doc_comment)]
    pub packaged_src: String,

    /// Folder where the source files will be extracted.
    /// Defaults to /tmp/unwrapped if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub output: Option<String>,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ReproducibleBuildPublishArgs {
    /// The bech32 address of the deployed contract to publish.
    #[arg(verbatim_doc_comment)]
    pub contract: String,

    /// Path to the .source.json file produced by a previous build.
    #[arg(long = "packaged-src", verbatim_doc_comment)]
    pub packaged_src: String,

    /// Docker image tag that was used to build the contract.
    /// e.g. `multiversx/sdk-rust-contract-builder:v8.0.0`
    #[arg(long = "docker-image", verbatim_doc_comment)]
    pub docker_image: String,

    /// URL of the verifier service.
    #[arg(long = "verifier-url", verbatim_doc_comment)]
    pub verifier_url: String,

    /// For multicontract repos: the specific contract variant to publish.
    #[arg(long = "contract-variant", verbatim_doc_comment)]
    pub contract_variant: Option<String>,

    /// Path to a PEM wallet file used to sign the verification request.
    /// Mutually exclusive with --keystore.
    #[arg(long, verbatim_doc_comment)]
    pub pem: Option<String>,

    /// Path to a keystore JSON wallet file used to sign the verification request.
    /// Mutually exclusive with --pem.
    #[arg(long, verbatim_doc_comment)]
    pub keystore: Option<String>,

    /// Keystore password (plain text). If omitted, will prompt interactively.
    #[arg(long = "keystore-password", verbatim_doc_comment)]
    pub keystore_password: Option<String>,

    /// Skip the confirmation prompt before submitting.
    #[arg(
        long = "skip-confirmation",
        short = 'y',
        default_value = "false",
        verbatim_doc_comment
    )]
    pub skip_confirmation: bool,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ReproducibleBuildUnpublishArgs {
    /// The bech32 address of the deployed contract to unpublish.
    #[arg(verbatim_doc_comment)]
    pub contract: String,

    /// The code hash of the contract to unpublish.
    #[arg(long = "code-hash", verbatim_doc_comment)]
    pub code_hash: String,

    /// URL of the verifier service.
    #[arg(long = "verifier-url", verbatim_doc_comment)]
    pub verifier_url: String,

    /// Path to a PEM wallet file used to sign the request.
    /// Mutually exclusive with --keystore.
    #[arg(long, verbatim_doc_comment)]
    pub pem: Option<String>,

    /// Path to a keystore JSON wallet file used to sign the request.
    /// Mutually exclusive with --pem.
    #[arg(long, verbatim_doc_comment)]
    pub keystore: Option<String>,

    /// Keystore password (plain text). If omitted, will prompt interactively.
    #[arg(long = "keystore-password", verbatim_doc_comment)]
    pub keystore_password: Option<String>,

    /// Skip the confirmation prompt before submitting.
    #[arg(
        long = "skip-confirmation",
        short = 'y',
        default_value = "false",
        verbatim_doc_comment
    )]
    pub skip_confirmation: bool,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ReproducibleBuildCheckArgs {
    /// The bech32 address of the deployed contract to check.
    #[arg(verbatim_doc_comment)]
    pub contract: String,

    /// URL of the verifier service.
    #[arg(long = "verifier-url", verbatim_doc_comment)]
    pub verifier_url: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ReproducibleBuildDownloadArgs {
    /// The bech32 address of the deployed contract to download.
    #[arg(verbatim_doc_comment)]
    pub contract: String,

    /// URL of the verifier service.
    #[arg(long = "verifier-url", verbatim_doc_comment)]
    pub verifier_url: String,

    /// Output directory where the ABI and source files will be written.
    /// Defaults to the current directory.
    #[arg(long, verbatim_doc_comment)]
    pub output: Option<String>,

    /// How many levels of dependencies to include (default: -1 = all).
    #[arg(long, verbatim_doc_comment)]
    pub depth: Option<i64>,

    /// Include test files in the downloaded sources.
    #[arg(
        long = "include-test-files",
        default_value = "false",
        verbatim_doc_comment
    )]
    pub include_test_files: bool,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ReproducibleBuildBuildArgs {
    /// Pinned Docker image tag to run the build in.
    /// e.g. `multiversx/sdk-rust-contract-builder:v12.0.0`
    /// If not specified, taken from [general] docker-image in sc-reproducible-build.toml.
    #[arg(long = "docker-image", verbatim_doc_comment)]
    pub docker_image: Option<String>,

    /// Project folder (workspace root or single contract folder).
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub project: Option<String>,

    /// Output folder where build artifacts will be placed.
    /// Defaults to `<project>/output-docker/`.
    #[arg(long, verbatim_doc_comment)]
    pub output: Option<String>,

    /// If the output folder is not empty, wipe it before building instead of aborting.
    #[arg(long, default_value = "false", verbatim_doc_comment)]
    pub overwrite: bool,

    /// Only build the contract with this name (as found in Cargo.toml).
    /// If not specified, all contracts under the project folder are built.
    #[arg(long, verbatim_doc_comment)]
    pub contract: Option<String>,

    /// Do not optimize wasm files after the build.
    #[arg(long = "no-wasm-opt", default_value = "false", verbatim_doc_comment)]
    pub no_wasm_opt: bool,

    /// Override the build root path inside the container.
    /// Defaults to the container's built-in default (/tmp/sc-build).
    #[arg(long = "build-root", verbatim_doc_comment)]
    pub build_root: Option<String>,

    /// Do not pass `--interactive` to `docker run`.
    /// Required in non-interactive environments such as CI.
    #[arg(
        long = "no-docker-interactive",
        default_value = "false",
        verbatim_doc_comment
    )]
    pub no_docker_interactive: bool,

    /// Do not pass `--tty` to `docker run`.
    /// Required in non-interactive environments such as CI.
    #[arg(long = "no-docker-tty", default_value = "false", verbatim_doc_comment)]
    pub no_docker_tty: bool,

    /// Skip forcing `--platform linux/amd64` on the Docker run.
    #[arg(
        long = "no-default-platform",
        default_value = "false",
        verbatim_doc_comment
    )]
    pub no_default_platform: bool,

    /// Set CARGO_TERM_VERBOSE=true inside the container.
    #[arg(long = "cargo-verbose", default_value = "false", verbatim_doc_comment)]
    pub cargo_verbose: bool,
}
