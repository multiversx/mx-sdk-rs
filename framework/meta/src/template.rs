use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::cli_args::TemplateArgs;

const TEMPLATE_REPO_ROOT: &str =
    "https://github.com/multiversx/mx-sdk-rs/tree/master/contracts/examples/";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateResponse {
    pub error: String,
    pub code: String,
    pub data: Option<Template>,
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
