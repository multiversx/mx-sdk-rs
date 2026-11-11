use std::fs;

use anyhow::{Context, Result};
use multiversx_sc_scenario::imports::NotPayable;
use multiversx_sc_snippets::imports::{BytesValue, InteractorIntoSdkTransaction};

use super::parse_code_metadata::parse_code_metadata;
use super::tx_cli_common::{
    build_arg_buffer, create_interactor, load_relayer_for_interactor, load_wallet, parse_address,
};
use crate::cli::cli_args_tx::UpgradeArgs;

pub async fn tx_upgrade(args: &UpgradeArgs) -> Result<()> {
    let contract = parse_address("contract", &args.contract)?;

    // Read bytecode.
    let bytecode = fs::read(&args.bytecode)
        .with_context(|| format!("failed to read bytecode from {}", args.bytecode.display()))?;
    let code = BytesValue::from(bytecode);

    let code_metadata = parse_code_metadata(&args.metadata);

    // Build upgrade transaction — same layout as deploy but sent to the existing contract address.
    let arg_buffer = build_arg_buffer(&args.arguments)?;

    let sender_wallet = load_wallet(&args.sender)?;

    // Create the interactor – this fetches the network config in the process.
    let mut interactor = create_interactor(&args.gateway, &args.tx).await?;
    let sender_address = interactor.register_wallet_bech32(sender_wallet).await;
    let relayer_address_opt = load_relayer_for_interactor(&mut interactor, &args.relayer).await?;

    let mut tx = interactor
        .tx()
        .from(&sender_address)
        .to(&contract)
        .gas(args.tx.gas_limit)
        .payment(NotPayable)
        .raw_upgrade()
        .code(code)
        .code_metadata(code_metadata)
        .arguments_raw(arg_buffer)
        .opt_relayer(relayer_address_opt)
        .into_sdk_transaction();

    interactor.set_tx_nonce_update_sender(&mut tx).await;

    interactor
        .sign_and_dispatch(
            tx,
            args.tx.send,
            args.tx.wait_result,
            args.tx.outfile.as_deref(),
        )
        .await
}
