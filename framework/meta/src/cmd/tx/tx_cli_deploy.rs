use std::fs;

use anyhow::{Context, Result};
use multiversx_sc::chain_core::std::new_address::compute_new_address_bech32;
use multiversx_sc_snippets::imports::{BytesValue, Interactor, InteractorIntoSdkTransaction};

use super::tx_cli_common::{build_arg_buffer, build_code_metadata, load_wallet, sign_and_dispatch};
use crate::cli::cli_args_tx::DeployArgs;

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

    // Build deploy transaction.
    let arg_buffer = build_arg_buffer(&args.arguments)?;
    let tx_builder = interactor
        .tx()
        .from(&sender_bech32)
        .gas(args.tx.gas_limit)
        .egld(args.payment.value)
        .raw_deploy()
        .code(code)
        .code_metadata(code_metadata)
        .arguments_raw(arg_buffer);

    let tx = tx_builder.into_sdk_transaction();

    let contract_address = compute_new_address_bech32(&tx.sender, nonce);
    println!("Contract address: {contract_address}");

    sign_and_dispatch(
        wallet,
        tx,
        nonce,
        &args.tx,
        &args.gateway,
        Some(contract_address.to_bech32_string()),
    )
    .await
}
