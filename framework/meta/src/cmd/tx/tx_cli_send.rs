use anyhow::Result;
use multiversx_chain_core::std::base64_decode;

use super::{
    output::TxOutputFile,
    tx_cli_common::{broadcast_and_save, load_transaction_from_file},
};
use crate::cli::cli_args_tx::SendArgs;

pub async fn tx_send(args: &SendArgs) {
    if let Err(e) = tx_send_inner(args).await {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

async fn tx_send_inner(args: &SendArgs) -> Result<()> {
    let tx = load_transaction_from_file(&args.infile)?;

    let decoded_data = match &tx.data {
        None => String::new(),
        Some(d) => {
            let bytes = base64_decode(d)?;
            String::from_utf8_lossy(&bytes).into_owned()
        }
    };

    let output = TxOutputFile {
        emitted_transaction: tx,
        emitted_transaction_data: decoded_data,
        emitted_transaction_hash: String::new(),
        contract_address: None,
        transaction_on_network: None,
    };

    broadcast_and_save(
        output,
        &args.proxy,
        args.outfile.as_deref(),
        args.wait_result,
    )
    .await
}
