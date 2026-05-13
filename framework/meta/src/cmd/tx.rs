mod output;
mod tx_cli_call;
mod tx_cli_common;
mod tx_cli_deploy;
mod tx_cli_new;
mod tx_cli_query;
mod tx_cli_send;
mod tx_cli_sign;
mod tx_cli_upgrade;

use crate::cli::cli_args_tx::{TxCliAction, TxCliArgs};
use tx_cli_call::tx_call;
use tx_cli_deploy::tx_deploy;
use tx_cli_new::tx_new;
use tx_cli_query::tx_query;
use tx_cli_send::tx_send;
use tx_cli_sign::tx_sign;
use tx_cli_upgrade::tx_upgrade;

pub async fn tx_cli(args: &TxCliArgs) {
    match &args.command {
        TxCliAction::Deploy(deploy_args) => tx_deploy(deploy_args).await,
        TxCliAction::Call(call_args) => tx_call(call_args).await,
        TxCliAction::Upgrade(upgrade_args) => tx_upgrade(upgrade_args).await,
        TxCliAction::Query(query_args) => tx_query(query_args).await,
        TxCliAction::New(new_args) => tx_new(new_args).await,
        TxCliAction::Send(send_args) => tx_send(send_args).await,
        TxCliAction::Sign(sign_args) => tx_sign(sign_args).await,
    }
}
