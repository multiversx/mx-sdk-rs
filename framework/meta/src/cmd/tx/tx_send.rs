use std::fs;

use anyhow::{Context, Result, anyhow};
use multiversx_sc_snippets::{
    imports::GatewayHttpProxy,
    sdk::data::transaction::{ApiTransactionResult, Transaction},
};
use serde_json::Value;

use super::tx_cli_args::SendArgs;

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

// ---------------------------------------------------------------------------
// Load transaction from the mxpy-compatible JSON file
// ---------------------------------------------------------------------------

fn load_transaction_from_file(path: &std::path::Path) -> Result<Transaction> {
    let content =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let v: Value = serde_json::from_str(&content)
        .with_context(|| format!("invalid JSON in {}", path.display()))?;

    // Accept either the mxpy "emittedTransaction" wrapper or a raw "tx" key.
    let tx_value = v
        .get("emittedTransaction")
        .or_else(|| v.get("tx"))
        .ok_or_else(|| {
            anyhow!(
                "file {} must contain an \"emittedTransaction\" or \"tx\" key",
                path.display()
            )
        })?;

    serde_json::from_value(tx_value.clone())
        .with_context(|| format!("failed to deserialize transaction from {}", path.display()))
}

// ---------------------------------------------------------------------------
// Wait for result
// ---------------------------------------------------------------------------

pub(super) async fn fetch_tx_on_network(
    gateway: &str,
    tx_hash: &str,
) -> Result<ApiTransactionResult> {
    let proxy = multiversx_sc_snippets::imports::GatewayHttpProxy::new(gateway.to_string());
    let (tx_on_network, _return_code) =
        multiversx_sdk::retrieve_tx_on_network(&proxy, tx_hash.to_string()).await;
    Ok(tx_on_network)
}
