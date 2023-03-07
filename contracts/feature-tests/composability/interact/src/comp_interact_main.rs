mod call_tree;
mod call_tree_deploy;
mod comp_interact_cli;
mod comp_interact_config;
mod comp_interact_controller;
mod comp_interact_state;

use clap::Parser;

use comp_interact_controller::ComposabilityInteract;

use forwarder_raw::ProxyTrait as ForwarderRawProxyTrait;
use multiversx_sc_snippets::{
    env_logger,
    multiversx_sc_scenario::{ContractInfo, DebugApi},
    tokio,
};

#[tokio::main]
async fn main() {
    DebugApi::dummy();
    env_logger::init();

    let mut composability_interact = ComposabilityInteract::init().await;

    let cli = comp_interact_cli::InteractCli::parse();
    match &cli.command {
        // Some(comp_interact_cli::InteractCliCommand::DeployVault) => {
        //     composability_interact.deploy_vault().await;
        // },
        // Some(comp_interact_cli::InteractCliCommand::DeployForwarderRaw) => {
        //     composability_interact.deploy_forwarder_raw().await;
        // },
        // Some(comp_interact_cli::InteractCliCommand::DeployPromises) => {
        //     composability_interact.deploy_promises().await;
        // },
        Some(comp_interact_cli::InteractCliCommand::Full) => {
            composability_interact.full_scenario().await;
        },
        None => {},
    }
}
