use clap::{Args, Parser, Subcommand};

#[derive(Default, PartialEq, Eq, Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<CliCommand>,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum CliCommand {
    #[command(name = "generate", about = "Gas schedule structs generation")]
    Generate(GenerateArg),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct GenerateArg {
    #[arg(short = 'v', long = "toml-version")]
    pub toml_version: u16,
}
