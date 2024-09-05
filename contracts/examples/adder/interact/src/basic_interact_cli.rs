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
    #[command(name = "add", about = "Add value")]
    Add(AddArgs),
    #[command(name = "deploy", about = "Deploy contract")]
    Deploy,
    #[command(name = "feed", about = "Feed contract EGLD")]
    Feed,
    #[command(name = "multi-deploy", about = "Multiple deploy contracts")]
    MultiDeploy(MultiDeployArgs),
    #[command(name = "sum", about = "Print sum")]
    Sum,
    #[command(name = "upgrade", about = "Upgrade contract")]
    Upgrade(UpgradeArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct AddArgs {
    /// The value to add
    #[arg(short = 'v', long = "value")]
    pub value: u32,

    /// Repeat this number of times
    #[arg(short = 'c', long = "count", default_value = "1")]
    pub count: usize,
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
