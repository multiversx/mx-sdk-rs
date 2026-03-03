#![allow(clippy::too_many_arguments)]

mod call_tree_calling_functions;
mod call_tree_config;
mod call_tree_config_gen;
mod call_tree_deploy;
mod comp_interact_cli;
mod comp_interact_state;

mod forwarder_queue_proxy;
mod vault_proxy;

use call_tree_config::CALL_TREE_FILE;
use clap::Parser;
use comp_interact_controller::ComposabilityInteract;
mod comp_interact_controller;
use multiversx_sc_snippets::imports::*;

#[tokio::main]
async fn main() {
    env_logger::init();

    let cli = comp_interact_cli::InteractCli::parse();

    // Handle s1 before init() so the file can be created from scratch.
    if let Some(comp_interact_cli::InteractCliCommand::S1) = &cli.command {
        call_tree_config_gen::scenario_1().save_to_file(CALL_TREE_FILE);
        println!("Scenario 1 call tree saved to {CALL_TREE_FILE}");
        return;
    }

    let mut interact = ComposabilityInteract::init().await;

    match &cli.command {
        Some(comp_interact_cli::InteractCliCommand::Full(args)) => {
            interact
                .full_scenario(&args.endpoint_name, &args.endpoint_args)
                .await;
        }
        Some(comp_interact_cli::InteractCliCommand::S1) => unreachable!(),
        Some(comp_interact_cli::InteractCliCommand::Deploy) => {
            interact.deploy_call_tree().await;
        }
        Some(comp_interact_cli::InteractCliCommand::SetQueuedCalls) => {
            interact.set_queued_calls_from_config().await;
        }
        None => {}
    }
}
