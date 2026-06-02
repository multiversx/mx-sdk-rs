use anyhow::{Result, anyhow};
use multiversx_chain_core::std::base64_decode;

use super::{
    output::TxOutputFile,
    tx_cli_common::{broadcast_and_save, load_transaction_from_file, load_wallet, save_output},
};
use crate::cli::cli_args_tx::SignArgs;

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

    if let Some(chain_id) = &args.gateway.chain {
        tx.chain_id = chain_id.clone();
    }

    let sig = wallet.sign_tx(&tx);
    tx.signature = Some(sig);

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

    if args.send {
        broadcast_and_save(
            output,
            &args.gateway.proxy,
            args.outfile.as_deref(),
            args.wait_result,
        )
        .await?;
    } else {
        save_output(&output, args.outfile.as_deref())?;
    }
    Ok(())
}
