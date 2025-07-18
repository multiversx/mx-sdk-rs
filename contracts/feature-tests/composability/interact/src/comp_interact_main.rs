#![allow(clippy::too_many_arguments)]

mod call_tree;
mod call_tree_calling_functions;
mod call_tree_deploy;
mod comp_interact_cli;
mod comp_interact_config;
mod comp_interact_controller;
mod comp_interact_state;

mod forwarder_queue_proxy;
mod vault_proxy;

use clap::Parser;
use comp_interact_controller::ComposabilityInteract;
use multiversx_sc_snippets::imports::*;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut composability_interact = ComposabilityInteract::init().await;

    let cli = comp_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(comp_interact_cli::InteractCliCommand::Full(args)) => {
            composability_interact
                .full_scenario(&args.endpoint_name, &args.endpoint_args)
                .await;
        }
        None => {}
    }
}
