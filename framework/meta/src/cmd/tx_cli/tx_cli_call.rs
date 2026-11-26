use anyhow::Result;
use multiversx_sc_snippets::imports::InteractorIntoSdkTransaction;

use super::parse_payments::parse_all_payment_args;
use super::tx_cli_common::{
    build_arg_buffer, create_interactor, load_relayer_for_interactor, load_wallet, parse_address,
};
use crate::cli::cli_args_tx::CallArgs;

pub async fn tx_call(args: &CallArgs) -> Result<()> {
    let contract = parse_address("contract", &args.contract)?;
    let arg_buffer = build_arg_buffer(&args.arguments)?;
    let payments = parse_all_payment_args(&args.payment)?;

    let sender_wallet = load_wallet(&args.sender)?;

    // Create the interactor – this fetches the network config in the process.
    let mut interactor = create_interactor(&args.gateway, &args.tx).await?;
    let sender_address = interactor.register_wallet_bech32(sender_wallet).await;
    let relayer_address_opt = load_relayer_for_interactor(&mut interactor, &args.relayer).await?;

    // Build call transaction.
    let mut tx = interactor
        .tx()
        .from(&sender_address)
        .to(&contract)
        .gas(args.tx.gas_limit)
        .payment(payments)
        .raw_call(args.function.as_str())
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
