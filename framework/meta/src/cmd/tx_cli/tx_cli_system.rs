mod system_tx_session;
mod tx_cli_system_esdt;

use crate::cli::cli_args_tx_system::{SystemCliAction, SystemCliArgs};

pub async fn tx_system(args: &SystemCliArgs) {
    let result = match &args.command {
        SystemCliAction::Esdt(esdt_args) => tx_cli_system_esdt::tx_system_esdt(esdt_args).await,
    };

    if let Err(e) = result {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}
