use clap::{Args, Parser, Subcommand};

/// Large ManagedVec storage gas benchmark
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(about = "Deploys both contract variants and produces a gas cost table")]
    Bench(BenchArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct BenchArgs {
    /// Comma-separated list of ManagedVec lengths to benchmark.
    #[arg(
        long,
        value_delimiter = ',',
        default_value = "1,10,20,50,100,120,150,200,500"
    )]
    pub sizes: Vec<usize>,

    /// Markdown file where the report is written.
    #[arg(long, default_value = "gas_report.md")]
    pub output: String,
}
