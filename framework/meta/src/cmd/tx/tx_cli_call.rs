use anyhow::Result;
use multiversx_sc_snippets::imports::{Bech32Address, Interactor, InteractorIntoSdkTransaction};

use super::{
    tx_cli_args::CallArgs,
    tx_cli_common::{build_arg_buffer, build_payments, load_wallet, sign_and_dispatch},
};

pub async fn tx_call(args: &CallArgs) {
    if let Err(e) = tx_call_inner(args).await {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

async fn tx_call_inner(args: &CallArgs) -> Result<()> {
    let wallet = load_wallet(&args.sender)?;

    // Create the interactor – fetches network config.
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

    // Build call transaction.
    let arg_buffer = build_arg_buffer(&args.arguments)?;
    let payments = build_payments(&args.payment)?;

    let tx = interactor
        .tx()
        .from(&sender_bech32)
        .to(&contract)
        .gas(args.tx.gas_limit)
        .payment(payments)
        .raw_call(args.function.as_str())
        .arguments_raw(arg_buffer)
        .into_sdk_transaction();

    sign_and_dispatch(wallet, tx, nonce, &args.tx, &args.gateway, None).await
}
