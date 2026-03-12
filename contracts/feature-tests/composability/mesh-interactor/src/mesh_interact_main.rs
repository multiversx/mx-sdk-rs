#![allow(clippy::too_many_arguments)]

mod call_tree_calling_functions;
mod call_tree_config;
mod call_tree_config_gen;
mod call_tree_deploy;
mod call_tree_gas;
mod call_tree_info;
mod mesh_interact_cli;
mod mesh_interact_state;

use call_tree_config::{CALL_TREE_FILE, CallTreeConfig};
use clap::Parser;
use mesh_interact_controller::ComposabilityInteract;
mod mesh_interact_controller;
use multiversx_sc_snippets::imports::*;

#[tokio::main]
async fn main() {
    env_logger::init();

    let cli = mesh_interact_cli::InteractCli::parse();

    match &cli.command {
        Some(mesh_interact_cli::InteractCliCommand::S1) => {
            let mut config = call_tree_config_gen::scenario_1();
            config.fill_gas_estimates();
            config.save_to_file(CALL_TREE_FILE);
            println!("Scenario 1 call tree saved to {CALL_TREE_FILE}");
        }
        Some(mesh_interact_cli::InteractCliCommand::S2 { n }) => {
            let mut config = call_tree_config_gen::scenario_2(*n);
            config.fill_gas_estimates();
            config.save_to_file(CALL_TREE_FILE);
            println!("Scenario 2 call tree (n={n}) saved to {CALL_TREE_FILE}");
        }
        Some(mesh_interact_cli::InteractCliCommand::UpdateGas) => {
            let mut config = CallTreeConfig::load_from_file(CALL_TREE_FILE);
            config.fill_gas_estimates();
            config.save_to_file(CALL_TREE_FILE);
            println!("Gas estimates updated in {CALL_TREE_FILE}");
            let mut interact = ComposabilityInteract::init().await;
            interact.set_programmed_calls().await;
        }
        Some(mesh_interact_cli::InteractCliCommand::Setup) => {
            let mut interact = ComposabilityInteract::init().await;
            interact.deploy_call_tree().await;
            interact.set_programmed_calls().await;
        }
        Some(mesh_interact_cli::InteractCliCommand::Bump) => {
            let mut interact = ComposabilityInteract::init().await;
            interact.bump().await;
        }
        Some(mesh_interact_cli::InteractCliCommand::Info) => {
            let mut interact = ComposabilityInteract::init().await;
            interact.query_trace_info().await;
        }
        None => {}
    }
}
