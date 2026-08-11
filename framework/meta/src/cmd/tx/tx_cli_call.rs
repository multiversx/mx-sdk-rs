use anyhow::Result;
use multiversx_sc_snippets::imports::{Bech32Address, Interactor, InteractorIntoSdkTransaction};

use super::parse_payments::parse_all_payment_args;
use super::tx_cli_common::{
    apply_gas_price, build_arg_buffer, load_relayer_wallet, load_wallet, sign_and_dispatch,
    validate_chain_id,
};
use crate::cli::cli_args_tx::CallArgs;

pub async fn tx_call(args: &CallArgs) {
    if let Err(e) = tx_call_inner(args).await {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

async fn tx_call_inner(args: &CallArgs) -> Result<()> {
    let sender_wallet = load_wallet(&args.sender)?;
    let relayer_wallet = load_relayer_wallet(&args.relayer)?;

    // Create the interactor – fetches network config.
    let mut interactor = Interactor::new(&args.gateway.proxy).await;
    let sender_address = interactor.register_wallet(sender_wallet).await;
    let sender_bech32 = sender_address.to_bech32(interactor.get_hrp());
    let relayer_address_opt = interactor.register_wallet_bech32_opt(relayer_wallet).await;

    // Determine nonce.
    let nonce = if let Some(n) = args.tx.nonce {
        n
    } else {
        interactor.recall_nonce(&sender_address).await
    };

    apply_gas_price(&mut interactor, &args.tx);
    validate_chain_id(&interactor, &args.gateway)?;

    let contract = Bech32Address::try_from_bech32_string(args.contract.clone())?;

    // Build call transaction.
    let arg_buffer = build_arg_buffer(&args.arguments)?;
    let payments = parse_all_payment_args(&args.payment)?;

    let tx = interactor
        .tx()
        .from(&sender_bech32)
        .to(&contract)
        .gas(args.tx.gas_limit)
        .payment(payments)
        .raw_call(args.function.as_str())
        .arguments_raw(arg_buffer)
        .opt_relayer(relayer_address_opt)
        .into_sdk_transaction();

    sign_and_dispatch(&interactor, tx, nonce, &args.tx, &args.gateway, None).await
}
