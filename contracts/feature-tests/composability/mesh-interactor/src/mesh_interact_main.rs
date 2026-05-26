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

impl ComposabilityInteract {
    fn load_layout(&self) -> CallTreeLayout {
        CallTreeLayout::load_from_file(&self.config.call_tree_path)
    }

    async fn cmd_update_gas(&mut self) {
        println!("Updating gas estimates...");
        let layout_path = self.config.call_tree_path.clone();
        let mut layout = self.load_layout();
        layout.fill_gas_estimates();
        layout.save_to_file(&layout_path);
        println!("Gas estimates updated in {layout_path}");
        self.program_calls(&layout).await;
    }

    async fn cmd_setup(&mut self) {
        println!("Setting up contracts...");
        let layout = self.load_layout();
        self.deploy_call_tree(&layout).await;
        self.program_calls(&layout).await;
        self.program_returns(&layout).await;
    }

    async fn cmd_bump(&mut self) {
        println!("Bumping...");
        let layout = self.load_layout();
        self.bump(&layout).await;
    }

    async fn cmd_full(&mut self) {
        println!("Running full sequence: setup + bump + info...");
        self.cmd_setup().await;
        self.cmd_bump().await;
        self.query_trace_info().await;
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let cli = mesh_interact_cli::InteractCli::parse();

    match &cli.command {
        Some(mesh_interact_cli::InteractCliCommand::Generate { n }) => {
            call_tree_config_gen::generate_layouts(*n);
        }
        Some(mesh_interact_cli::InteractCliCommand::UpdateGas) => {
            ComposabilityInteract::init().await.cmd_update_gas().await;
        }
        Some(mesh_interact_cli::InteractCliCommand::Setup) => {
            ComposabilityInteract::init().await.cmd_setup().await;
        }
        Some(mesh_interact_cli::InteractCliCommand::Bump) => {
            ComposabilityInteract::init().await.cmd_bump().await;
        }
        Some(mesh_interact_cli::InteractCliCommand::Info) => {
            ComposabilityInteract::init().await.query_trace_info().await;
        }
        Some(mesh_interact_cli::InteractCliCommand::Full) => {
            ComposabilityInteract::init().await.cmd_full().await;
        }
        None => {}
    }
}
