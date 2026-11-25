use colored::Colorize;

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
