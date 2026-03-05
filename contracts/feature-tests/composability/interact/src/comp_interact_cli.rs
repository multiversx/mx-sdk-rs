use clap::{Parser, Subcommand};

/// Composability Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// Composability Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(
        name = "s1",
        about = "Generate scenario 1 (root → vault) and save to call_tree.toml"
    )]
    S1,
    #[command(
        name = "setup",
        about = "Deploy all contracts from call_tree.toml, configure, and save addresses back"
    )]
    Setup,
    #[command(
        name = "run",
        about = "Send the start transactions defined in call_tree.toml"
    )]
    Run,
    #[command(
        name = "info",
        about = "Query the trace view for all deployed contracts and print the results"
    )]
    Info,
}
