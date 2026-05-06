mod output;
pub mod tx_cli_args;
mod tx_deploy;
mod tx_new;
mod tx_send;

use tx_cli_args::{TxCliAction, TxCliArgs};
use tx_deploy::tx_deploy;
use tx_new::tx_new;
use tx_send::tx_send;

pub async fn tx_cli(args: &TxCliArgs) {
    match &args.command {
        TxCliAction::Deploy(deploy_args) => tx_deploy(deploy_args).await,
        TxCliAction::Call(_call_args) => todo!("tx call not yet implemented"),
        TxCliAction::Upgrade(_upgrade_args) => todo!("tx upgrade not yet implemented"),
        TxCliAction::Query(_query_args) => todo!("tx query not yet implemented"),
        TxCliAction::New(new_args) => tx_new(new_args).await,
        TxCliAction::Send(send_args) => tx_send(send_args).await,
        TxCliAction::Sign(_sign_args) => todo!("tx sign not yet implemented"),
    }
}
