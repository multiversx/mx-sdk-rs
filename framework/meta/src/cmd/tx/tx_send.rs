use anyhow::Result;
use multiversx_sc_snippets::sdk::utils::base64_decode;

use super::{
    output::TxOutputFile,
    tx_cli_args::SendArgs,
    tx_common::{broadcast_and_save, load_transaction_from_file},
};

pub async fn tx_send(args: &SendArgs) {
    if let Err(e) = tx_send_inner(args).await {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

async fn tx_send_inner(args: &SendArgs) -> Result<()> {
    let tx = load_transaction_from_file(&args.infile)?;

    let decoded_data = tx
        .data
        .as_ref()
        .map(|d| String::from_utf8_lossy(&base64_decode(d)).into_owned())
        .unwrap_or_default();

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
