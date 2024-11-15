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
        about = "General info about the contract an libraries residing in the targetted directory.."
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
        about = "Generates a report on the local depedencies of contract crates. Will explore indirect depdencies too."
    )]
    LocalDeps(LocalDepsArgs),

    #[command(
        name = "wallet",
        about = "Generates a new wallet or performs actions on an existing wallet."
    )]
    Wallet(WalletArgs),
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
    /// Target directory where to generate contract integration tests.
    /// Will be current directory if not specified.
    #[arg(short, long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// This arg runs rust and go tests.
    /// Default value will be "false" if not specified.
    #[arg(short, long, default_value = "false", verbatim_doc_comment)]
    pub go: bool,

    /// This arg runs interactor tests using chain simulator
    /// Default value will be "false" if not specified
    #[arg(
        short = 'c',
        long = "chain-simulator",
        default_value = "false",
        verbatim_doc_comment
    )]
    pub chain_simulator: bool,

    /// This arg runs scenarios.
    /// Default value will be "false" if not specified.
    /// If scen and go are both specified, scen overrides the go argument.
    #[arg(short, long, default_value = "false", verbatim_doc_comment)]
    pub scen: bool,

    /// This arg prints the entire output of the vm.
    /// Default value will be "false" if not specified
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

impl AllArgs {
    pub fn target_dir_all_override(&self) -> Self {
        let mut result = self.clone();
        if let Some(target_dir_all) = &self.target_dir_all {
            result.target_dir_meta = Some(target_dir_all.clone());
            match &mut result.command {
                ContractCliAction::Build(build_args) => {
                    build_args.target_dir_wasm = Some(target_dir_all.clone());
                },
                ContractCliAction::BuildDbg(build_args) => {
                    build_args.target_dir_wasm = Some(target_dir_all.clone());
                },
                ContractCliAction::Twiggy(build_args) => {
                    build_args.target_dir_wasm = Some(target_dir_all.clone());
                },
                _ => {},
            }
        }
        result
    }

    pub fn to_cargo_run_args(&self) -> Vec<String> {
        let processed = self.target_dir_all_override();
        let mut raw = vec!["run".to_string()];
        if let Some(target_dir_meta) = &processed.target_dir_meta {
            raw.push("--target-dir".to_string());
            raw.push(target_dir_meta.clone());
        }
        raw.append(&mut processed.command.to_raw());
        if !processed.load_abi_git_version {
            raw.push("--no-abi-git-version".to_string());
        }
        raw
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
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct WalletConvertArgs {
    #[arg(long = "in-format", verbatim_doc_comment)]
    pub from: String,

    #[arg(long = "out-format", verbatim_doc_comment)]
    pub to: String,

    #[arg(long = "infile", verbatim_doc_comment)]
    pub infile: Option<String>,

    #[arg(long = "outfile", verbatim_doc_comment)]
    pub outfile: Option<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct WalletBech32Args {
    #[arg(long = "encode", verbatim_doc_comment)]
    pub hex_address: Option<String>,
    #[arg(long = "decode", verbatim_doc_comment)]
    pub bech32_address: Option<String>,
}
