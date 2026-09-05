use anyhow::Result;
use multiversx_sc_snippets::{Interactor, TxOutputFile};

use super::tx_cli_common::load_transaction_from_file;
use crate::cli::cli_args_tx::SendArgs;

pub async fn tx_send(args: &SendArgs) {
    if let Err(e) = tx_send_inner(args).await {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

async fn tx_send_inner(args: &SendArgs) -> Result<()> {
    let tx = load_transaction_from_file(&args.infile)?;

    let output = TxOutputFile::from_transaction(tx)?;
    let interactor = Interactor::empty()
        .with_connection(&args.proxy)
        .await
        .use_chain_simulator_auto();
    interactor
        .broadcast_and_save(output, args.outfile.as_deref(), args.wait_result)
        .await
}
