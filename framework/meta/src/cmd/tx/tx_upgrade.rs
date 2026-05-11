use std::fs;

use anyhow::{Context, Result};
use multiversx_sc_scenario::imports::NotPayable;
use multiversx_sc_snippets::imports::{Bech32Address, BytesValue, Interactor, InteractorRunAsync};

use super::{
    tx_cli_args::UpgradeArgs,
    tx_common::{build_arg_buffer, build_code_metadata, load_wallet, sign_and_dispatch},
};

pub async fn tx_upgrade(args: &UpgradeArgs) {
    if let Err(e) = tx_upgrade_inner(args).await {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

async fn tx_upgrade_inner(args: &UpgradeArgs) -> Result<()> {
    let wallet = load_wallet(&args.sender)?;

    // Create the interactor – this fetches the network config.
    let mut interactor = Interactor::new(&args.gateway.proxy).await;
    let sender_address = interactor.register_wallet(wallet).await;
    let sender_bech32 = sender_address.to_bech32(interactor.get_hrp());

    // Determine nonce.
    let nonce = if let Some(n) = args.tx.nonce {
        n
    } else {
        interactor.recall_nonce(&sender_address).await
    };

    let contract = Bech32Address::try_from_bech32_string(args.contract.clone())?;

    // Read bytecode.
    let bytecode = fs::read(&args.bytecode)
        .with_context(|| format!("failed to read bytecode from {}", args.bytecode.display()))?;
    let code = BytesValue::from(bytecode);

    let code_metadata = build_code_metadata(&args.metadata);

    // Build upgrade transaction — same layout as deploy but sent to the existing contract address.
    let arg_buffer = build_arg_buffer(&args.arguments)?;
    let tx_builder = interactor
        .tx()
        .from(&sender_bech32)
        .to(&contract)
        .gas(args.tx.gas_limit)
        .payment(NotPayable)
        .raw_upgrade()
        .code(code)
        .code_metadata(code_metadata)
        .arguments_raw(arg_buffer);

    let tx = tx_builder.into_sdk_transaction();

    sign_and_dispatch(wallet, tx, nonce, &args.tx, &args.gateway, None).await
}
