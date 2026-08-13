use anyhow::{Result, anyhow};
use multiversx_chain_core::std::base64_decode;

use super::{
    output::TxOutputFile,
    tx_cli_common::{
        broadcast_and_save, load_relayer_wallet, load_transaction_from_file, save_output,
    },
};
use crate::cli::cli_args_tx::RelayArgs;

pub async fn tx_relay(args: &RelayArgs) {
    if let Err(e) = tx_relay_inner(args).await {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

async fn tx_relay_inner(args: &RelayArgs) -> Result<()> {
    let relayer_wallet = load_relayer_wallet(&args.relayer)?.ok_or_else(|| {
        anyhow!("a relayer wallet is required: use --relayer-pem or --relayer-keyfile")
    })?;

    let mut tx = load_transaction_from_file(&args.infile)?;

    // Validate that the transaction has a relayer field set.
    let tx_relayer = tx
        .relayer
        .clone()
        .ok_or_else(|| anyhow!("transaction does not have a relayer field; set it when building the transaction with --relayer <bech32>"))?;

    // Validate that the sender has already signed.
    if tx.signature.is_none() {
        return Err(anyhow!(
            "transaction is not signed by the sender; sign it first with `tx sign`"
        ));
    }

    // Validate that the relayer wallet matches the transaction's relayer field.
    let relayer_addr = relayer_wallet.to_address().to_bech32_default();
    if relayer_addr != tx_relayer {
        return Err(anyhow!(
            "relayer wallet address {} does not match transaction relayer {}",
            relayer_addr.to_bech32_str(),
            tx_relayer.to_bech32_str(),
        ));
    }

    if let Some(chain_id) = &args.gateway.chain {
        tx.chain_id = chain_id.clone();
    }

    let relayer_sig = relayer_wallet.sign_tx(&tx)?;
    tx.relayer_signature = Some(relayer_sig);

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
