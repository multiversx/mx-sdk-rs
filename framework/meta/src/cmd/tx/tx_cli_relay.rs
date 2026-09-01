use anyhow::{Result, anyhow};
use multiversx_sc_snippets::{Interactor, TxOutputFile};

use super::tx_cli_common::{load_relayer_wallet, load_transaction_from_file};
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

    let output = TxOutputFile::from_transaction(tx, None)?;

    if args.send {
        let interactor = Interactor::empty()
            .with_connection(&args.gateway.proxy)
            .await
            .use_chain_simulator_auto();
        interactor
            .broadcast_and_save(output, args.outfile.as_deref(), args.wait_result)
            .await?;
    } else {
        output.save_output(args.outfile.as_deref())?;
    }
    Ok(())
}
