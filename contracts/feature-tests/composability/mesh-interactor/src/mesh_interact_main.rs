#![allow(clippy::too_many_arguments)]

mod call_tree_calling_functions;
mod call_tree_config;
mod call_tree_config_gen;
mod call_tree_deploy;
mod call_tree_gas;
mod call_tree_info;
mod mesh_interact_cli;

use call_tree_config::CallTreeLayout;
use clap::Parser;
use mesh_interact_controller::ComposabilityInteract;
mod mesh_interact_controller;
use multiversx_sc_snippets::imports::*;


#[tokio::main]
async fn main() {
    env_logger::init();

    let cli = mesh_interact_cli::InteractCli::parse();

    match &cli.command {
        Some(mesh_interact_cli::InteractCliCommand::Generate { n }) => {
            call_tree_config_gen::generate_layouts(*n);
        }
        Some(mesh_interact_cli::InteractCliCommand::UpdateGas) => {
            let mut interact = ComposabilityInteract::init().await;
            let layout_path = interact.config.call_tree_path.clone();
            let mut layout = CallTreeLayout::load_from_file(&layout_path);
            layout.fill_gas_estimates();
            layout.save_to_file(&layout_path);
            println!("Gas estimates updated in {layout_path}");
            interact.program_calls(&layout).await;
        }
        Some(mesh_interact_cli::InteractCliCommand::Setup) => {
            let mut interact = ComposabilityInteract::init().await;
            let layout_path = interact.config.call_tree_path.clone();
            let layout = CallTreeLayout::load_from_file(&layout_path);
            interact.deploy_call_tree(&layout).await;
            interact.program_calls(&layout).await;
        }
        Some(mesh_interact_cli::InteractCliCommand::Bump) => {
            let mut interact = ComposabilityInteract::init().await;
            let layout_path = interact.config.call_tree_path.clone();
            let layout = CallTreeLayout::load_from_file(&layout_path);
            interact.bump(&layout).await;
        }
        Some(mesh_interact_cli::InteractCliCommand::Info) => {
            let mut interact = ComposabilityInteract::init().await;
            interact.query_trace_info().await;
        }
        None => {}
    }
}
