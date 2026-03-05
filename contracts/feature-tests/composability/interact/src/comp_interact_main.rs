#![allow(clippy::too_many_arguments)]

mod call_tree_calling_functions;
mod call_tree_config;
mod call_tree_config_gen;
mod call_tree_deploy;
mod call_tree_info;
mod comp_interact_cli;
mod comp_interact_state;

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
            call_tree_config_gen::scenario_1().save_to_file(CALL_TREE_FILE);
            println!("Scenario 1 call tree saved to {CALL_TREE_FILE}");
        }
        Some(comp_interact_cli::InteractCliCommand::Setup) => {
            let mut interact = ComposabilityInteract::init().await;
            interact.deploy_call_tree().await;
            interact.set_queued_calls_from_config().await;
        }
        Some(comp_interact_cli::InteractCliCommand::Run) => {
            let mut interact = ComposabilityInteract::init().await;
            interact.run_start_calls().await;
        }
        Some(comp_interact_cli::InteractCliCommand::Info) => {
            let mut interact = ComposabilityInteract::init().await;
            interact.query_trace_info().await;
        }
        None => {}
    }
}
