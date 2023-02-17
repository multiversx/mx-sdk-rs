use clap::{ArgAction, Args, Parser, Subcommand};

use super::CliArgsToRaw;

/// Parsed arguments of the meta crate CLI.
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(
    version,
    about,
    after_help = "
The MultiversX smart contract Meta crate can be used in three ways:
    A. Import it into a contract's specific meta-crate. 
        There it will receive access to the contract ABI generator. 
        Based on that it is able to build the contract and apply various tools.
        This part also contains the multi-contract config infrastructure.

    B. Use it as a standalone tool.
        It can be used to automatically upgrade contracts from one version to the next.

    C. Create a new contract from a preexisting template.
    
You are currently using the template tool (A).
"
)]
#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct TemplateArgs {
    /// Provide the name of the template you want to clone
    #[arg(long = "name", verbatim_doc_comment)]
    pub name: String,
}

impl CliArgsToRaw for TemplateArgs {
    fn to_raw(&self) -> Vec<String> {
        Vec::new()
    }
}
