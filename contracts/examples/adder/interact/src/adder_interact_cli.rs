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
    #[command(name = "add", about = "Add")]
    Add(AddArgs),
    #[command(name = "deploy", about = "Deploy")]
    Deploy,
    #[command(name = "sum", about = "Sum")]
    Sum,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct AddArgs {
    /// The value to add
    #[arg(short = 'v', long = "value", verbatim_doc_comment)]
    pub value: u8,
}
