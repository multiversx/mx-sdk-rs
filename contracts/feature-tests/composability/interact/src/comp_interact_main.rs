#![allow(clippy::too_many_arguments)]

mod call_tree_calling_functions;
mod call_tree_config;
mod call_tree_config_gen;
mod call_tree_deploy;
mod call_tree_info;
mod comp_interact_cli;
mod comp_interact_state;
mod call_tree_gas;

use call_tree_config::CALL_TREE_FILE;
use clap::Parser;
use comp_interact_controller::ComposabilityInteract;
mod comp_interact_controller;
use multiversx_sc_snippets::imports::*;

#[tokio::main]
async fn main() {
    env_logger::init();

    let cli = comp_interact_cli::InteractCli::parse();

    match &cli.command {
        Some(comp_interact_cli::InteractCliCommand::S1) => {
            let mut config = call_tree_config_gen::scenario_1();
            config.fill_gas_estimates();
            config.save_to_file(CALL_TREE_FILE);
            println!("Scenario 1 call tree saved to {CALL_TREE_FILE}");
        }
        Some(comp_interact_cli::InteractCliCommand::Setup) => {
            let mut interact = ComposabilityInteract::init().await;
            println!("Deploying call tree contracts...");
            interact.deploy_call_tree().await;
            println!("Setting up programmed calls from config...");
            interact.set_queued_calls_from_config().await;
        }
        Some(comp_interact_cli::InteractCliCommand::Bump) => {
            let mut interact = ComposabilityInteract::init().await;
            interact.bump().await;
        }
        Some(comp_interact_cli::InteractCliCommand::Info) => {
            let mut interact = ComposabilityInteract::init().await;
            interact.query_trace_info().await;
        }
        None => {}
    }
}
