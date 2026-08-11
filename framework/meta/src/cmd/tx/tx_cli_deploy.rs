use std::fs;

use anyhow::{Context, Result};
use multiversx_sc::chain_core::std::new_address::compute_new_address_bech32;
use multiversx_sc_snippets::imports::{BytesValue, Interactor, InteractorIntoSdkTransaction};

use super::parse_code_metadata::parse_code_metadata;
use super::tx_cli_common::{
    apply_gas_price, build_arg_buffer, load_relayer_for_interactor, load_wallet, sign_and_dispatch,
    validate_chain_id,
};
use crate::cli::cli_args_tx::DeployArgs;

pub async fn tx_deploy(args: &DeployArgs) {
    if let Err(e) = tx_deploy_inner(args).await {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

async fn tx_deploy_inner(args: &DeployArgs) -> Result<()> {
    let sender_wallet = load_wallet(&args.sender)?;

    // Create the interactor – this fetches the network config in the process.
    let mut interactor = Interactor::new(&args.gateway.proxy).await;
    let sender_address = interactor
        .register_wallet_bech32(sender_wallet.clone())
        .await;
    let relayer_address_opt = load_relayer_for_interactor(&mut interactor, &args.relayer).await?;

    // Determine nonce.
    let nonce = if let Some(n) = args.tx.nonce {
        n
    } else {
        interactor.recall_nonce(&sender_address.address).await
    };

    apply_gas_price(&mut interactor, &args.tx);
    validate_chain_id(&interactor, &args.gateway)?;

    // Read bytecode file and wrap in BytesValue so it implements TxCodeValue.
    let bytecode = fs::read(&args.bytecode)
        .with_context(|| format!("failed to read bytecode from {}", args.bytecode.display()))?;
    let code = BytesValue::from(bytecode);

    // Build CodeMetadata from flags.
    let code_metadata = parse_code_metadata(&args.metadata);

    // Build deploy transaction.
    let arg_buffer = build_arg_buffer(&args.arguments)?;
    let tx_builder = interactor
        .tx()
        .from(&sender_address)
        .gas(args.tx.gas_limit)
        .egld(args.payment.value)
        .raw_deploy()
        .code(code)
        .code_metadata(code_metadata)
        .arguments_raw(arg_buffer)
        .opt_relayer(relayer_address_opt);

    let tx = tx_builder.into_sdk_transaction();

    let contract_address = compute_new_address_bech32(&tx.sender, nonce);

    if let Some(ex) = &interactor.explorer_url {
        println!("new contract: {}", ex.address_url(&contract_address));
    } else {
        println!("new contract address: {contract_address}");
    }

    sign_and_dispatch(
        &interactor,
        tx,
        nonce,
        &args.tx,
        &args.gateway,
        Some(contract_address.to_bech32_string()),
    )
    .await
}
