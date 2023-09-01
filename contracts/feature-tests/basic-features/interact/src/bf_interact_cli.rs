use clap::{Args, Parser, Subcommand};

/// Adder Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// Adder Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy", about = "Experiment with large storage")]
    Deploy,
    #[command(name = "large-storage", about = "Experiment with large storage")]
    LargeStorage(LargeStorageArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct LargeStorageArgs {
    /// The value to add
    #[arg(long = "kb")]
    pub size_kb: usize,
}
