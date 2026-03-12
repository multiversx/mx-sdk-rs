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

const ASYNC_SHARDED_LAYOUT: &str = "layouts/async_sharded.toml";
const TRANSF_EXEC_SHARDED_LAYOUT: &str = "layouts/transf_exec_sharded.toml";
const SYNC_CHAIN_LAYOUT: &str = "layouts/sync_chain.toml";

#[tokio::main]
async fn main() {
    env_logger::init();

    let cli = mesh_interact_cli::InteractCli::parse();

    match &cli.command {
        Some(mesh_interact_cli::InteractCliCommand::Generate { n }) => {
            std::fs::create_dir_all("layouts").expect("failed to create layouts/ directory");

            let mut async_sharded = call_tree_config_gen::async_sharded();
            async_sharded.fill_gas_estimates();
            async_sharded.save_to_file(ASYNC_SHARDED_LAYOUT);
            println!("Async sharded layout saved to {ASYNC_SHARDED_LAYOUT}");

            let mut transf_exec_sharded = call_tree_config_gen::transf_exec_sharded();
            transf_exec_sharded.fill_gas_estimates();
            transf_exec_sharded.save_to_file(TRANSF_EXEC_SHARDED_LAYOUT);
            println!("Transfer-execute sharded layout saved to {TRANSF_EXEC_SHARDED_LAYOUT}");

            let mut sync_chain = call_tree_config_gen::sync_chain(*n);
            sync_chain.fill_gas_estimates();
            sync_chain.save_to_file(SYNC_CHAIN_LAYOUT);
            println!("Sync chain layout (n={n}) saved to {SYNC_CHAIN_LAYOUT}");
        }
        Some(mesh_interact_cli::InteractCliCommand::UpdateGas) => {
            let mut interact = ComposabilityInteract::init().await;
            let layout_path = interact.config.call_tree_path.clone();
            let mut layout = CallTreeLayout::load_from_file(&layout_path);
            layout.fill_gas_estimates();
            layout.save_to_file(&layout_path);
            println!("Gas estimates updated in {layout_path}");
            interact.set_programmed_calls(&layout).await;
        }
        Some(mesh_interact_cli::InteractCliCommand::Setup) => {
            let mut interact = ComposabilityInteract::init().await;
            let layout_path = interact.config.call_tree_path.clone();
            let layout = CallTreeLayout::load_from_file(&layout_path);
            interact.deploy_call_tree(&layout).await;
            interact.set_programmed_calls(&layout).await;
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
