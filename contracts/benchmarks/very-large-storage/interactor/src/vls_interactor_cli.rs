use clap::{Args, Parser, Subcommand};

/// Very Large Storage Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// Very Large Storage Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy", about = "Deploy contract")]
    Deploy,
    #[command(name = "append", about = "Append bytes to storage")]
    Append(AppendArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct AppendArgs {
    /// Number of bytes to append
    #[arg(short = 'n', long = "num-bytes")]
    pub num_bytes: u64,
}
