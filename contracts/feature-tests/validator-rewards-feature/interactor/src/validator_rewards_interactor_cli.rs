use clap::{Parser, Subcommand};

/// Validator Rewards Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// Validator Rewards Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy", about = "Deploy contract")]
    Deploy,
    #[command(name = "rewards", about = "Print cumulated and unclaimed rewards")]
    Rewards,
    #[command(name = "claim", about = "Claim all unclaimed rewards, as owner")]
    Claim,
}
