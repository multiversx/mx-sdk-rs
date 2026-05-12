use std::fs;

use anyhow::{Context, Result, anyhow};
use multiversx_sc_snippets::{hex, imports::GatewayHttpProxy};

use super::{
    output::TxOutputFile,
    tx_cli_args::SignArgs,
    tx_common::{fetch_tx_on_network, load_transaction_from_file, load_wallet, to_json_pretty},
};

pub async fn tx_sign(args: &SignArgs) {
    if let Err(e) = tx_sign_inner(args).await {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

async fn tx_sign_inner(args: &SignArgs) -> Result<()> {
    let wallet = load_wallet(&args.sender)?;
    let mut tx = load_transaction_from_file(&args.infile)?;

    // Validate that the wallet address matches the transaction sender.
    let wallet_address = wallet.to_address().to_bech32_default();
    if wallet_address != tx.sender {
        return Err(anyhow!(
            "wallet address {} does not match transaction sender {}",
            wallet_address.to_bech32_str(),
            tx.sender.to_bech32_str(),
        ));
    }

    // Apply chain ID override if provided.
    if let Some(chain_id) = &args.gateway.chain {
        tx.chain_id = chain_id.clone();
    }

    // Sign.
    let sig = wallet.sign_tx(&tx);
    tx.signature = Some(hex::encode(sig));

    let decoded_data = tx
        .data
        .as_ref()
        .map(multiversx_sc_snippets::sdk::utils::base64_decode)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default();

    let output = TxOutputFile {
        emitted_transaction: tx,
        emitted_transaction_data: decoded_data,
        emitted_transaction_hash: String::new(),
        contract_address: None,
        transaction_on_network: None,
    };

    let json = to_json_pretty(&output)?;

    // Write / print the signed tx.
    if let Some(outfile) = &args.outfile {
        fs::write(outfile, &json)
            .with_context(|| format!("failed to write to {}", outfile.display()))?;
        println!("Transaction saved to {}", outfile.display());
    } else if !args.send {
        println!("{json}");
    }

    // Optionally broadcast.
    if args.send {
        let proxy = GatewayHttpProxy::new(args.gateway.proxy.clone());
        let tx_hash = proxy
            .send_transaction(&output.emitted_transaction)
            .await
            .context("failed to broadcast transaction")?;
        println!("Transaction hash: {tx_hash}");

        let mut output_with_hash = TxOutputFile {
            emitted_transaction_hash: tx_hash.clone(),
            ..output
        };

        if args.wait_result {
            println!("Waiting for transaction result...");
            let result = fetch_tx_on_network(&args.gateway.proxy, &tx_hash).await?;
            output_with_hash.transaction_on_network = Some(result);
        }

        let json = to_json_pretty(&output_with_hash)?;
        if let Some(outfile) = &args.outfile {
            fs::write(outfile, &json)
                .with_context(|| format!("failed to write to {}", outfile.display()))?;
        } else {
            println!("{json}");
        }
    }

    Ok(())
}
