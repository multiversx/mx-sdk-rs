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
        name = "gen",
        about = "Generate all call tree layouts: layout/async_sharded.toml and layout/sync_chain.toml"
    )]
    Generate {
        #[arg(
            short = 'n',
            long,
            default_value_t = 10,
            help = "Number of contracts in the sync chain"
        )]
        n: usize,
    },
    #[command(
        name = "update-gas",
        about = "Recompute gas estimates in call_tree.toml and push updated programmed calls on-chain"
    )]
    UpdateGas,
    #[command(
        name = "setup",
        about = "Deploy all contracts from call_tree.toml, configure, and save addresses back"
    )]
    Setup,
    #[command(
        name = "bump",
        about = "Send the start transactions defined in call_tree.toml"
    )]
    Bump,
    #[command(
        name = "info",
        about = "Query the trace view for all deployed contracts and print the results"
    )]
    Info,
    #[command(name = "full", about = "Run setup, bump, and info in sequence")]
    Full,
}
