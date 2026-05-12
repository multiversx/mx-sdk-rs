use std::fs;

use anyhow::{Context, Result};
use multiversx_sc_snippets::imports::GatewayHttpProxy;

use super::{
    tx_cli_args::SendArgs,
    tx_common::{fetch_tx_on_network, load_transaction_from_file},
};

pub async fn tx_send(args: &SendArgs) {
    if let Err(e) = tx_send_inner(args).await {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

async fn tx_send_inner(args: &SendArgs) -> Result<()> {
    let tx = load_transaction_from_file(&args.infile)?;

    let proxy = GatewayHttpProxy::new(args.proxy.clone());
    let tx_hash = proxy
        .send_transaction(&tx)
        .await
        .context("failed to broadcast transaction")?;
    println!("Transaction hash: {tx_hash}");

    let result_json = if args.wait_result {
        let on_network = fetch_tx_on_network(&args.proxy, &tx_hash).await?;
        serde_json::to_string_pretty(&on_network).context("failed to serialize result")?
    } else {
        serde_json::json!({ "txHash": tx_hash }).to_string()
    };

    if let Some(outfile) = &args.outfile {
        fs::write(outfile, &result_json)
            .with_context(|| format!("failed to write to {}", outfile.display()))?;
        println!("Result saved to {}", outfile.display());
    } else {
        println!("{result_json}");
    }

    Ok(())
}
