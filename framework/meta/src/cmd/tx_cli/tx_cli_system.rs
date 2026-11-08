mod tx_cli_system_esdt;

use anyhow::Result;

use crate::cli::cli_args_tx_system::{SystemCliAction, SystemCliArgs};

pub async fn tx_system(args: &SystemCliArgs) -> Result<()> {
    match &args.command {
        SystemCliAction::Esdt(esdt_args) => tx_cli_system_esdt::tx_system_esdt(esdt_args).await,
    }
}
