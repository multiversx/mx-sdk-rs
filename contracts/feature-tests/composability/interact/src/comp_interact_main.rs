#![allow(clippy::too_many_arguments)]

mod call_tree;
mod call_tree_calling_functions;
mod call_tree_deploy;
mod comp_interact_cli;
mod comp_interact_config;
mod comp_interact_controller;
mod comp_interact_state;

mod forwarder_proxy;
mod forwarder_queue_proxy;
mod vault_proxy;

use call_tree::CallState;
use clap::Parser;
use comp_interact_controller::ComposabilityInteract;
use multiversx_sc_snippets::imports::*;

const FUNGIBLE_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("TOKEN-0000");

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
        },
        None => {},
    }
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
pub async fn transf_exec_by_user_cs_test() {
    let mut composability_interact = ComposabilityInteract::init().await;
    let wallet_address = composability_interact.wallet_address.clone();

    // set state for wallet address on chain simulator (esdt balance)

    let call_state = CallState::simple_example_1();

    let (vault, forwarder) = composability_interact
        .deploy_call_tree_contracts(&call_state)
        .await;

    let logs = composability_interact
        .interactor
        .tx()
        .from(wallet_address)
        .to(forwarder.first().unwrap())
        .typed(forwarder_proxy::ForwarderProxy)
        .forward_transf_exec_by_user_accept_funds(vault.first().unwrap())
        .single_esdt(
            &FUNGIBLE_TOKEN_ID.to_token_identifier(),
            0,
            &BigUint::from(100u64),
        )
        .returns(ReturnsLogs)
        .run()
        .await;

    println!("Logs: {logs:?}");
}
