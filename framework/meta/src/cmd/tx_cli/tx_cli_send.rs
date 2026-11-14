use anyhow::Result;
use multiversx_sc_snippets::{Interactor, TxOutputFile, TxOutputTarget};

use super::tx_cli_common::load_transaction_from_file;
use crate::cli::cli_args_tx::SendArgs;

pub async fn tx_send(args: &SendArgs) -> Result<()> {
    let tx = load_transaction_from_file(&args.infile)?;

    let output = TxOutputFile::from_transaction(tx)?;
    let interactor = Interactor::empty()
        .with_connection(&args.proxy)
        .await
        .use_chain_simulator_auto();
    interactor
        .broadcast_and_save(
            output,
            &TxOutputTarget::file_or_stdout(args.outfile.clone()),
            args.wait_result,
        )
        .await
}
