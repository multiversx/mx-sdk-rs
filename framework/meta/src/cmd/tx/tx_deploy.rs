use std::fs;

use anyhow::{Context, Result, anyhow};
use multiversx_sc_scenario::multiversx_sc::types::CodeMetadata;
use multiversx_sc_snippets::{
    hex,
    imports::{
        BytesValue, GatewayHttpProxy, Interactor, InteractorRunAsync, InterpretableFrom,
        InterpreterContext,
    },
    sdk::{utils::base64_decode, wallet::Wallet},
};
use multiversx_sdk::data::keystore::InsertPassword;

use crate::cmd::tx::tx_send::fetch_tx_on_network;

use super::{
    output::TxOutputFile,
    tx_cli_args::{DeployArgs, MetadataArgs, SenderArgs},
};

pub async fn tx_deploy(args: &DeployArgs) {
    if let Err(e) = tx_deploy_inner(args).await {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

async fn tx_deploy_inner(args: &DeployArgs) -> Result<()> {
    let wallet = load_wallet(&args.sender)?;

    // Create the interactor – this fetches the network config in the process.
    let mut interactor = Interactor::new(&args.gateway.proxy).await;
    let sender_address = interactor.register_wallet(wallet).await;
    let sender_bech32 = sender_address.to_bech32(interactor.get_hrp());

    // Determine nonce.
    let nonce = if let Some(n) = args.tx.nonce {
        n
    } else {
        interactor.recall_nonce(&sender_address).await
    };

    // Read bytecode file and wrap in BytesValue so it implements TxCodeValue.
    let bytecode = fs::read(&args.bytecode)
        .with_context(|| format!("failed to read bytecode from {}", args.bytecode.display()))?;
    let code = BytesValue::from(bytecode);

    // Build CodeMetadata from flags.
    let code_metadata = build_code_metadata(&args.metadata);

    // Interpret constructor arguments (mandos expression format, e.g. "0x1a", "str:hello", "42").
    let context =
        InterpreterContext::new().with_dir(std::env::current_dir().context("failed to get cwd")?);
    let encoded_args: Vec<BytesValue> = args
        .arguments
        .iter()
        .map(|s| BytesValue::interpret_from(s.as_str(), &context))
        .collect();

    // Build the deploy transaction using the interactor Tx builder syntax,
    // then fold in constructor arguments, and finally call into_sdk_transaction().
    let tx_builder = interactor
        .tx()
        .from(&sender_bech32)
        .gas(args.tx.gas_limit)
        .egld(args.tx.value)
        .raw_deploy()
        .code(code)
        .code_metadata(code_metadata);

    let tx_builder = encoded_args
        .iter()
        .fold(tx_builder, |b, arg| b.argument(arg));

    // Convert to SDK transaction (populates chain_id, gas_price, version, data from network config).
    let mut tx = tx_builder.into_sdk_transaction();

    // Apply caller-controlled overrides and set nonce.
    tx.nonce = nonce;
    if let Some(gas_price) = args.tx.gas_price {
        tx.gas_price = gas_price;
    }
    if let Some(chain_id) = &args.gateway.chain {
        tx.chain_id = chain_id.clone();
    }

    // The deploy data field is already base64-encoded inside tx.data by into_sdk_transaction();
    // decode it for the human-readable TxOutputFile field.
    let decoded_data = tx
        .data
        .as_ref()
        .map(|d| String::from_utf8_lossy(&base64_decode(d)).into_owned())
        .unwrap_or_default();

    let sig = wallet.sign_tx(&tx);
    tx.signature = Some(hex::encode(sig));

    let output = TxOutputFile {
        emitted_transaction: tx,
        emitted_transaction_data: decoded_data,
        emitted_transaction_hash: String::new(),
        transaction_on_network: None,
    };

    let json = serde_json::to_string_pretty(&output).context("failed to serialize transaction")?;

    // Write / print signed tx.
    if let Some(outfile) = &args.tx.outfile {
        fs::write(outfile, &json)
            .with_context(|| format!("failed to write to {}", outfile.display()))?;
        println!("Transaction saved to {}", outfile.display());
    } else if !args.tx.send {
        println!("{json}");
    }

    // Optionally broadcast.
    if args.tx.send {
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

        if args.tx.wait_result {
            println!("Waiting for transaction result...");
            let result = fetch_tx_on_network(&args.gateway.proxy, &tx_hash).await?;
            output_with_hash.transaction_on_network = Some(result);
        }

        let json = serde_json::to_string_pretty(&output_with_hash)
            .context("failed to serialize transaction")?;
        if let Some(outfile) = &args.tx.outfile {
            fs::write(outfile, &json)
                .with_context(|| format!("failed to write to {}", outfile.display()))?;
        } else {
            println!("{json}");
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn load_wallet(sender: &SenderArgs) -> Result<Wallet> {
    if let Some(pem) = &sender.pem {
        Wallet::from_pem_file(pem.to_str().context("invalid pem path")?)
            .context("failed to load PEM wallet")
    } else if let Some(keyfile) = &sender.keyfile {
        Wallet::from_keystore_secret(
            keyfile.to_str().context("invalid keyfile path")?,
            InsertPassword::StandardInput,
        )
        .context("failed to load keystore wallet")
    } else {
        Err(anyhow!("a wallet is required: use --pem or --keyfile"))
    }
}

fn build_code_metadata(meta: &MetadataArgs) -> CodeMetadata {
    let mut flags = CodeMetadata::DEFAULT;
    if !meta.metadata_not_upgradeable {
        flags |= CodeMetadata::UPGRADEABLE;
    }
    if !meta.metadata_not_readable {
        flags |= CodeMetadata::READABLE;
    }
    if meta.metadata_payable {
        flags |= CodeMetadata::PAYABLE;
    }
    if meta.metadata_payable_by_sc {
        flags |= CodeMetadata::PAYABLE_BY_SC;
    }
    flags
}
