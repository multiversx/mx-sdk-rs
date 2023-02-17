use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const TEMPLATE_REPO_ROOT: &str =
    "https://github.com/multiversx/mx-sdk-rs/tree/master/contracts/examples/";

/// Parsed arguments of the meta crate CLI.
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct TemplateCliArgs {
    /// Provide the target API you want the real data to come from
    #[arg(long = "api")]
    #[clap(global = true)]
    pub api: Option<String>,

    #[command(subcommand)]
    pub command: Option<TemplateCliAction>,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum TemplateCliAction {
    #[command(about = "Creates a contract by a pre-existing template")]
    Template(TemplateArgs),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateResponse {
    pub error: String,
    pub code: String,
    pub data: Option<Template>,
}

/// Entry point in the program when calling it as a standalone tool.
pub async fn cli_template() {
    let cli_args = TemplateCliArgs::parse();
    match &cli_args.command {
        Some(TemplateCliAction::Template(args)) => {
            download_contract_template(args);
        },
        None => {},
    }
}

pub async fn download_contract_template(args: &TemplateArgs) -> Result<Template> {
    let client = Client::new();
    let resp = client
        .get(get_template_location(args))
        .send()
        .await?
        .json::<TemplateResponse>()
        .await?;

    match resp.data {
        None => Err(anyhow!("{}", resp.error)),
        Some(b) => Ok(b),
    }
}

fn get_template_location(args: &TemplateArgs) -> String {
    let mut location = TEMPLATE_REPO_ROOT.to_string();
    location.push_str(&args.name);
    location
}
