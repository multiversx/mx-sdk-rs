use colored::Colorize;
use multiversx_sdk::chain_core::types::GasResult;

pub(super) fn query_err_message(err: &anyhow::Error) {
    eprintln!(
        "{}{}",
        "Query failed: ".to_string().red().bold(),
        err.to_string().red().bold()
    );
}

pub(super) fn simulate_gas_transfer_err_message(err: &anyhow::Error) {
    eprintln!(
        "{}{}",
        "Gas simulation for transfer failed: "
            .to_string()
            .red()
            .bold(),
        err.to_string().red().bold()
    );
}

pub(super) fn simulate_gas_deploy_err_message(err: &anyhow::Error) {
    eprintln!(
        "{}{}",
        "Gas simulation for deploy failed: "
            .to_string()
            .red()
            .bold(),
        err.to_string().red().bold()
    );
}

/// Printed when the gas simulation completed, but did not produce a usable gas estimate.
pub(super) fn simulate_gas_no_estimate_err_message(tx_kind: &str, gas_result: &GasResult) {
    eprintln!(
        "{}",
        format!("Gas simulation for {tx_kind} did not produce an estimate: {gas_result}")
            .red()
            .bold()
    );
}

pub(crate) fn simulate_gas_sc_call_err_message(err: &anyhow::Error) {
    eprintln!(
        "{}{}",
        "Gas simulation for SC call failed: "
            .to_string()
            .red()
            .bold(),
        err.to_string().red().bold()
    );
}
