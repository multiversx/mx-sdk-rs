use anyhow::Result;
use multiversx_sc_snippets::imports::{Bech32Address, InteractorIntoSdkTransaction};

use super::parse_payments::parse_all_payment_args;
use super::tx_cli_common::{
    build_arg_buffer, create_interactor, load_relayer_for_interactor, load_wallet,
};
use crate::cli::cli_args_tx::CallArgs;

pub async fn tx_call(args: &CallArgs) -> Result<()> {
    let sender_wallet = load_wallet(&args.sender)?;

    // Create the interactor – fetches network config.
    let mut interactor = create_interactor(&args.gateway, &args.tx).await?;
    let sender_address = interactor.register_wallet(sender_wallet).await;
    let sender_bech32 = sender_address.to_bech32(interactor.get_hrp());
    let relayer_address_opt = load_relayer_for_interactor(&mut interactor, &args.relayer).await?;

    let contract = Bech32Address::try_from_bech32_string(args.contract.clone())?;

    // Build call transaction.
    let arg_buffer = build_arg_buffer(&args.arguments)?;
    let payments = parse_all_payment_args(&args.payment)?;

    interactor
        .tx()
        .from(&sender_bech32)
        .to(&contract)
        .gas(args.tx.gas_limit)
        .payment(payments)
        .raw_call(args.function.as_str())
        .arguments_raw(arg_buffer)
        .opt_relayer(relayer_address_opt)
        .run_with_settings(&args.tx.dispatch_settings())
        .await
}
