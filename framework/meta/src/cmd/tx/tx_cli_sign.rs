use anyhow::{Result, anyhow};
use multiversx_chain_core::std::Bech32Address;
use multiversx_sc_snippets::{Interactor, TxOutputFile};

use super::tx_cli_common::{load_relayer_wallet, load_transaction_from_file, load_wallet};
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

    if let Some(relayer_str) = &args.relayer.relayer {
        let relayer_addr = Bech32Address::try_from_bech32_string(relayer_str.clone())
            .map_err(|e| anyhow!("invalid --relayer address: {e}"))?;
        tx.relayer = Some(relayer_addr);
    }

    let sig = wallet.sign_tx(&tx)?;
    tx.signature = Some(sig);

    // Optionally sign as relayer.
    if let Some(relayer_w) = load_relayer_wallet(&args.relayer)? {
        let relayer_addr = relayer_w.to_address().to_bech32_default();
        if let Some(tx_relayer) = &tx.relayer {
            if relayer_addr != *tx_relayer {
                return Err(anyhow!(
                    "relayer wallet address {} does not match transaction relayer {}",
                    relayer_addr.to_bech32_str(),
                    tx_relayer.to_bech32_str(),
                ));
            }
        } else {
            // No relayer field in the tx yet — derive it from the wallet.
            tx.relayer = Some(relayer_addr);
        }
        let relayer_sig = relayer_w.sign_tx(&tx)?;
        tx.relayer_signature = Some(relayer_sig);
    }

    let output = TxOutputFile::from_transaction(tx)?;

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
