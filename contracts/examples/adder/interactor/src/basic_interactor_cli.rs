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
    #[command(name = "deploy", about = "Deploy contract")]
    Deploy,
    #[command(name = "upgrade", about = "Upgrade contract")]
    Upgrade(UpgradeArgs),
    #[command(name = "sum", about = "Print sum")]
    Sum,
    #[command(name = "add", about = "Add value")]
    Add(AddArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct AddArgs {
    /// The value to add
    #[arg(short = 'v', long = "value")]
    pub value: u32,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct UpgradeArgs {
    /// The value to add
    #[arg(short = 'v', long = "value")]
    pub value: u32,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct MultiDeployArgs {
    /// The number of contracts to deploy
    #[arg(short = 'c', long = "count")]
    pub count: usize,
}
