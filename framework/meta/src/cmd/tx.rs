pub mod tx_cli_args;

use tx_cli_args::{TxCliAction, TxCliArgs};

pub async fn tx_cli(args: &TxCliArgs) {
    match &args.command {
        TxCliAction::Deploy(_deploy_args) => todo!("tx deploy not yet implemented"),
        TxCliAction::Call(_call_args) => todo!("tx call not yet implemented"),
        TxCliAction::Upgrade(_upgrade_args) => todo!("tx upgrade not yet implemented"),
        TxCliAction::Query(_query_args) => todo!("tx query not yet implemented"),
        TxCliAction::New(_new_args) => todo!("tx new not yet implemented"),
        TxCliAction::Send(_send_args) => todo!("tx send not yet implemented"),
        TxCliAction::Sign(_sign_args) => todo!("tx sign not yet implemented"),
    }
}
