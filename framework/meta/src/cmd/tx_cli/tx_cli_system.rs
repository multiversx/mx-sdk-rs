mod system_common;
mod tx_cli_system_delegation;
mod tx_cli_system_esdt;
mod tx_cli_system_governance;
mod tx_cli_system_validator;

use anyhow::Result;

use crate::cli::cli_args_tx_system::{SystemCliAction, SystemCliArgs};

pub async fn tx_system(args: &SystemCliArgs) -> Result<()> {
    match &args.command {
        SystemCliAction::Esdt(esdt_args) => tx_cli_system_esdt::tx_system_esdt(esdt_args).await,
        SystemCliAction::DelegationManager(manager_args) => {
            tx_cli_system_delegation::tx_system_delegation_manager(manager_args).await
        }
        SystemCliAction::Delegation(delegation_args) => {
            tx_cli_system_delegation::tx_system_delegation(delegation_args).await
        }
        SystemCliAction::Validator(validator_args) => {
            tx_cli_system_validator::tx_system_validator(validator_args).await
        }
        SystemCliAction::Governance(governance_args) => {
            tx_cli_system_governance::tx_system_governance(governance_args).await
        }
    }
}
