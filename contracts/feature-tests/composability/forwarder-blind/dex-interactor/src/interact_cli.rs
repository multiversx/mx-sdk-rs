use clap::{Args, Parser, Subcommand};

/// ForwarderBlind Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// ForwarderBlind Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy", about = "Deploy contract")]
    Deploy,
    #[command(name = "wrap-egld", about = "Wrap EGLD into WEGLD")]
    WrapEgld(WrapEgldArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct WrapEgldArgs {
    /// Amount of EGLD to wrap, in denomination (1 EGLD = 10^18)
    #[arg(short = 'a', long = "amount")]
    pub amount: u64,
}
