use clap::{Args, Parser, Subcommand};

/// GovernanceFuncCalls Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// GovernanceFuncCalls Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(about = "Propose")]
    Propose,

    #[command(about = "View config")]
    ViewConfig,

    #[command(about = "View proposal")]
    ViewProposal,

    #[command(about = "Vote")]
    Vote,
}
