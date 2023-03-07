use clap::{Args, Parser, Subcommand};

use super::account_tool;

/// Parsed arguments of the meta crate CLI.
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct ScenarioCliArgs {
    /// Provide the target API you want the real data to come from
    #[arg(long = "api")]
    #[clap(global = true)]
    pub api: Option<String>,

    #[command(subcommand)]
    pub command: Option<ScenarioCliAction>,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum ScenarioCliAction {
    #[command(
        about = "Generates a scenario test initialized with real data fetched from the blockchain."
    )]
    Account(AccountArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct AccountArgs {
    /// Provide the address you want to retrieve data from
    #[arg(long = "address", verbatim_doc_comment)]
    pub address: String,
}

/// Entry point in the program when calling it as a standalone tool.
pub async fn cli_main() {
    let cli_args = ScenarioCliArgs::parse();
    let api = cli_args.api.expect("API needs tp be specified");
    match &cli_args.command {
        Some(ScenarioCliAction::Account(args)) => {
            account_tool::print_account_as_scenario_set_state(api, args).await;
        },
        None => {},
    }
}
