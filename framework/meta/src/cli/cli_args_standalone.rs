use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use multiversx_sc_meta_lib::cli::{CliArgsToRaw, ContractCliAction};

/// Parsed arguments of the meta crate CLI.
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(
    version,
    about,
    after_help = "
The MultiversX smart contract Meta crate can be used in two ways:
    A. Import it into a contract's specific meta-crate. 
        There it will receive access to the contract ABI generator. 
        Based on that it is able to build the contract and apply various tools.
        This part also contains the multi-contract config infrastructure.

    B. Use it as a standalone tool.
        It can be used to automatically upgrade contracts from one version to the next.

You are currently using the standalone tool (B).
"
)]
#[command(propagate_version = true)]
pub struct StandaloneCliArgs {
    #[command(subcommand)]
    pub command: Option<StandaloneCliAction>,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum StandaloneCliAction {
    #[command(name = "install", about = "Installs framework dependencies")]
    Install(InstallArgs),

    #[command(
        about = "General info about the contract an libraries residing in the targeted directory.."
    )]
    Info(InfoArgs),

    #[command(
        about = "Calls the meta crates for all contracts under given path with the given arguments."
    )]
    All(AllArgs),

    #[command(
        about = "Upgrades a contract to the latest version. Multiple contract crates are allowed."
    )]
    Upgrade(UpgradeArgs),

    #[command(name = "new", about = "Creates a contract by a pre-existing template")]
    Template(TemplateArgs),

    #[command(name = "templates", about = "Lists all pre-existing templates")]
    TemplateList(TemplateListArgs),

    #[command(
        name = "test-gen",
        about = "Generates Rust integration tests based on scenarios provided in the scenarios folder of each contract."
    )]
    TestGen(TestGenArgs),

    #[command(
        name = "scen-blackbox",
        about = "Generates blackbox tests from scenario files (.scen.json)."
    )]
    ScenBlackbox(ScenBlackboxArgs),

    #[command(name = "test", about = "Runs cargo test")]
    Test(TestArgs),

    #[command(name = "test-coverage", about = "Run test coverage and output report")]
    TestCoverage(TestCoverageArgs),

    #[command(name = "report", about = "Generate code report")]
    CodeReportGen(CodeReportArgs),

    #[command(
        about = "Generates a scenario test initialized with real data fetched from the blockchain."
    )]
    Account(AccountArgs),

    #[command(
        name = "local-deps",
        about = "Generates a report on the local dependencies of contract crates. Will explore indirect dependencies too."
    )]
    LocalDeps(LocalDepsArgs),

    #[command(
        name = "reproducible-build",
        alias = "rb",
        about = "Reproducible build operations."
    )]
    ReproducibleBuild(ReproducibleBuildArgs),

    #[command(
        name = "wallet",
        about = "Generates a new wallet or performs actions on an existing wallet."
    )]
    Wallet(WalletArgs),

    #[command(
        name = "cs",
        about = "Can install, start and stop a chain simulator configuration."
    )]
    ChainSimulator(ChainSimulatorArgs),
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ChainSimulatorArgs {
    #[command(subcommand)]
    pub command: ChainSimulatorCommand,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum ChainSimulatorCommand {
    #[command(
        about = "Pulls the latest chain simulator docker image available. Needs Docker installed."
    )]
    Install,

    #[command(about = "Starts the chain simulator.")]
    Start,

    #[command(about = "Stops the chain simulator.")]
    Stop,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct InfoArgs {
    /// Target directory to retrieve info from.
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// Ignore all directories with these names.
    #[arg(long, verbatim_doc_comment)]
    #[clap(global = true, default_value = "target")]
    pub ignore: Vec<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct TestArgs {
    /// Target directory where to generate contract integration tests (default: current directory)
    #[arg(short, long)]
    pub path: Option<String>,

    /// Run Debugger (Rust-only) and Go tests; deprecated in favor of -w or --wasm (default: "false")
    #[arg(short, long, default_value = "false")]
    pub go: bool,

    /// Run tests that are based on compiled contracts (default: "false")
    #[arg(short, long, default_value = "false")]
    pub wasm: bool,

    /// Run interactor tests using chain simulator (default: "false")
    #[arg(
        short = 'c',
        long = "chain-simulator",
        default_value = "false",
        verbatim_doc_comment
    )]
    pub chain_simulator: bool,

    /// Run mx-scenario-go (default: "false")
    /// Overrides other arguments
    #[arg(short, long, default_value = "false", verbatim_doc_comment)]
    pub scen: bool,

    /// Print the entire output from the Rust tests (default: "false")
    #[arg(short, long, default_value = "false", verbatim_doc_comment)]
    pub nocapture: bool,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, ValueEnum)]
pub enum OutputFormat {
    /// Markdown pretty-print summary
    #[default]
    Markdown,

    /// JSON summary
    Json,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct TestCoverageArgs {
    /// Output file path
    #[arg(short, long, verbatim_doc_comment)]
    pub output: String,

    /// Output format
    #[arg(short, long, verbatim_doc_comment)]
    pub format: Option<OutputFormat>,

    /// Ignore files by path patterns
    #[arg(short = 'i', long = "ignore-filename-regex", verbatim_doc_comment)]
    pub ignore_filename_regex: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct CodeReportArgs {
    #[command(subcommand)]
    pub command: CodeReportAction,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum CodeReportAction {
    #[command(name = "compile", about = "Generates the contract report.")]
    Compile(CompileArgs),

    #[command(name = "compare", about = "Compare two contract reports.")]
    Compare(CompareArgs),

    #[command(
        name = "convert",
        about = "Converts a contract report to a Markdown file."
    )]
    Convert(ConvertArgs),
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct CompileArgs {
    /// Target directory where to generate code report.
    #[arg(short, long, verbatim_doc_comment)]
    pub path: PathBuf,

    /// Path to the Markdown or JSON file where the report results will be written.
    #[arg(short, long, verbatim_doc_comment)]
    pub output: PathBuf,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct CompareArgs {
    /// Path to the previous version of code report JSON file
    /// that will be used for comparison.
    #[arg(short, long, verbatim_doc_comment)]
    pub baseline: PathBuf,

    /// Path to the current version of the code report JSON file
    /// that will be compared.
    #[arg(short, long, verbatim_doc_comment)]
    pub new: PathBuf,

    /// Path to the Markdown file where the comparison results will be written.
    #[arg(short, long, verbatim_doc_comment)]
    pub output: PathBuf,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ConvertArgs {
    /// Path to the JSON report file that needs to be converted to Markdown format.
    #[arg(short, long, verbatim_doc_comment)]
    pub input: PathBuf,

    /// Path to the Markdown file where the report results will be written.
    #[arg(short, long, verbatim_doc_comment)]
    pub output: PathBuf,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct MetaLibArgs {
    #[arg(
        long = "no-abi-git-version",
        help = "Skips loading the Git version into the ABI",
        action = ArgAction::SetFalse
    )]
    #[clap(global = true)]
    pub load_abi_git_version: bool,

    /// For the meta crates, allows specifying the target directory where the Rust compiler will build the intermediary files.
    /// Sharing the same target directory can speed up building multiple contract crates at once.
    #[arg(long = "target-dir-meta", verbatim_doc_comment)]
    #[clap(global = true)]
    pub target_dir_meta: Option<String>,

    /// Overrides both the --target-dir-meta and the --target-dir-wasm args.
    #[arg(long = "target-dir-all", verbatim_doc_comment)]
    #[clap(global = true)]
    pub target_dir_all: Option<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct AllArgs {
    #[command(subcommand)]
    pub command: ContractCliAction,

    /// Target directory where to call all contract meta crates.
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    #[clap(global = true)]
    pub path: Option<String>,

    /// Ignore all directories with these names.
    #[arg(long, verbatim_doc_comment)]
    #[clap(global = true, default_value = "target")]
    pub ignore: Vec<String>,

    #[command(flatten)]
    pub meta_lib_args: MetaLibArgs,
}

impl AllArgs {
    pub fn target_dir_all_override(&self) -> Self {
        let mut result = self.clone();
        if let Some(target_dir_all) = &self.meta_lib_args.target_dir_all {
            result.meta_lib_args.target_dir_meta = Some(target_dir_all.clone());
            match &mut result.command {
                ContractCliAction::Build(build_args) => {
                    build_args.target_dir_wasm = Some(target_dir_all.clone());
                }
                ContractCliAction::BuildDbg(build_args) => {
                    build_args.target_dir_wasm = Some(target_dir_all.clone());
                }
                ContractCliAction::Twiggy(build_args) => {
                    build_args.target_dir_wasm = Some(target_dir_all.clone());
                }
                _ => {}
            }
        }
        result
    }
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct UpgradeArgs {
    /// Target directory where to upgrade contracts.
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// Ignore all directories with these names.
    #[arg(long, verbatim_doc_comment)]
    #[clap(global = true, default_value = "target")]
    pub ignore: Vec<String>,

    /// Overrides the version to upgrade to.
    /// By default it will be the last version out.
    #[arg(long = "to", verbatim_doc_comment)]
    pub override_target_version: Option<String>,

    /// Skips 'cargo check' after upgrade
    #[arg(short, long, default_value = "false", verbatim_doc_comment)]
    pub no_check: bool,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct PackArgs {
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
pub struct ReproducibleBuildArgs {
    #[command(subcommand)]
    pub command: ReproducibleBuildCliAction,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum ReproducibleBuildCliAction {
    #[command(
        name = "source-pack",
        about = "Packages the contract source code into a self-contained JSON file, suitable for reproducible builds."
    )]
    SourcePack(PackArgs),

    #[command(
        name = "local-build",
        about = "Builds all contracts locally, mirroring the Docker reproducible build pipeline."
    )]
    LocalBuild(LocalBuildArgs),

    #[command(
        name = "docker-build",
        about = "Runs the reproducible build inside a pinned Docker container."
    )]
    DockerBuild(DockerBuildArgs),

    #[command(
        name = "local-deps",
        about = "Generates a report on the local dependencies of the contract."
    )]
    LocalDeps(LocalDepsArgs),
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct LocalBuildArgs {
    /// Project folder (workspace root or single contract folder).
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// Output folder where build artifacts and source JSON files will be placed.
    /// A subfolder per contract name will be created inside it.
    #[arg(long, verbatim_doc_comment)]
    pub output: String,

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
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DockerBuildArgs {
    /// Pinned Docker image tag to run the build in.
    /// e.g. `multiversx/sc-meta-reproducible-build:0.65.1`
    #[arg(long = "docker-image", verbatim_doc_comment)]
    pub docker_image: String,

    /// Project folder (workspace root or single contract folder).
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub project: Option<String>,

    /// Output folder where build artifacts will be placed.
    /// Defaults to `<project>/output-docker/`.
    #[arg(long, verbatim_doc_comment)]
    pub output: Option<String>,

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

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct LocalDepsArgs {
    /// Target directory where to generate local deps reports.
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// Ignore all directories with these names.
    #[arg(long, verbatim_doc_comment)]
    #[clap(global = true, default_value = "target")]
    pub ignore: Vec<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct TemplateArgs {
    /// The new name the contract is to receive.
    /// If missing, the template name will be considered.
    #[arg(long, verbatim_doc_comment)]
    pub name: Option<String>,

    /// The contract template to clone.
    #[arg(long, verbatim_doc_comment)]
    pub template: String,

    /// The framework version on which the contracts should be created.
    #[arg(long, verbatim_doc_comment)]
    pub tag: Option<String>,

    /// Target directory where to create the new contract directory.
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<PathBuf>,

    /// The author of the contract.
    /// If missing, the default author will be considered.
    #[arg(long, verbatim_doc_comment)]
    pub author: Option<String>,
}

impl CliArgsToRaw for TemplateArgs {
    fn to_raw(&self) -> Vec<String> {
        Vec::new()
    }
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct TemplateListArgs {
    /// The framework version referred to.
    #[arg(long = "tag", verbatim_doc_comment)]
    pub tag: Option<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct TestGenArgs {
    /// Target directory where to generate contract integration tests.
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// Ignore all directories with these names.
    #[arg(long, verbatim_doc_comment)]
    #[clap(global = true, default_value = "target")]
    pub ignore: Vec<String>,

    /// Creates test files if they don't exist.
    #[arg(long, verbatim_doc_comment)]
    pub create: bool,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct ScenBlackboxArgs {
    /// Target directory where to generate contract blackbox tests.
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// Override test files if they already exist.
    #[arg(long, verbatim_doc_comment)]
    pub overwrite: bool,

    /// Ignore all directories with these names.
    #[arg(long, verbatim_doc_comment)]
    #[clap(global = true, default_value = "target")]
    pub ignore: Vec<String>,

    /// Output file path for the generated blackbox test.
    /// If not specified, the default path inside the contract's tests/ folder is used.
    #[arg(long, verbatim_doc_comment)]
    pub output: Option<String>,
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Parser)]
#[command(propagate_version = true)]
pub struct InstallArgs {
    #[command(subcommand)]
    pub command: Option<InstallCommand>,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InstallCommand {
    #[command(about = "Installs all the known tools")]
    All,

    #[command(about = "Installs the `mx-scenario-go` tool")]
    MxScenarioGo(InstallMxScenarioGoArgs),

    #[command(name = "wasm32", about = "Installs the `wasm32` target")]
    Wasm32(InstallWasm32Args),

    #[command(name = "wasm-opt", about = "Installs the `wasm-opt` tool")]
    WasmOpt(InstallWasmOptArgs),

    #[command(name = "debugger", about = "Installs the lldb debugger script tool")]
    Debugger(InstallDebuggerArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct InstallMxScenarioGoArgs {
    /// The framework version on which the contracts should be created.
    #[arg(long, verbatim_doc_comment)]
    pub tag: Option<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct InstallWasm32Args {}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct InstallWasmOptArgs {}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct InstallDebuggerArgs {}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct AccountArgs {
    /// Provide the target API you want the data to come from
    #[arg(long = "api")]
    #[clap(global = true)]
    pub api: Option<String>,

    /// Provide the address you want to retrieve data from
    #[arg(long = "address", verbatim_doc_comment)]
    pub address: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum WalletAction {
    #[command(name = "new", about = "Creates a new wallet")]
    New(WalletNewArgs),

    #[command(
        name = "bech32",
        about = "Encodes/decodes a bech32 address to/from hex"
    )]
    Bech32(WalletBech32Args),
    #[command(name = "convert", about = "Converts a wallet")]
    Convert(WalletConvertArgs),
}

#[derive(Clone, PartialEq, Eq, Debug, Parser)]
#[command(propagate_version = true)]
pub struct WalletArgs {
    #[command(subcommand)]
    pub command: WalletAction,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct WalletNewArgs {
    /// The type of wallet to create.
    #[arg(long = "format", verbatim_doc_comment)]
    pub wallet_format: Option<String>,

    /// The name of the wallet to create.
    #[arg(long = "outfile", verbatim_doc_comment)]
    pub outfile: Option<String>,

    #[arg(long = "hrp", verbatim_doc_comment)]
    pub hrp: Option<String>,

    /// If set, mines a wallet assigned to the given shard ID.
    /// For the standard 3-shard mainnet configuration, valid shard IDs are 0, 1, or 2.
    #[arg(long = "shard", verbatim_doc_comment)]
    pub shard: Option<u8>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct WalletConvertArgs {
    /// The format of the input wallet. Allowed values: mnemonic, pem, keystore-secret.
    #[arg(long = "in-format", verbatim_doc_comment)]
    pub from: String,

    /// The format of the output wallet. Allowed values: pem, keystore-secret.
    /// Supported conversions: mnemonic -> pem, keystore-secret -> pem, pem -> keystore-secret.
    #[arg(long = "out-format", verbatim_doc_comment)]
    pub to: String,

    #[arg(long = "infile", verbatim_doc_comment)]
    pub infile: Option<String>,

    #[arg(long = "outfile", verbatim_doc_comment)]
    pub outfile: Option<String>,

    #[arg(long = "hrp", verbatim_doc_comment)]
    pub hrp: Option<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct WalletBech32Args {
    #[arg(long = "encode", verbatim_doc_comment)]
    pub hex_address: Option<String>,
    #[arg(long = "decode", verbatim_doc_comment)]
    pub bech32_address: Option<String>,
}
