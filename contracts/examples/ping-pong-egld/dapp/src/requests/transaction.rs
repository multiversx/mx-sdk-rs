use crate::interactor::ContractInteract;
use multiversx_sc_snippets::imports::*;

use super::proxy;

pub async fn deploy_sc() -> Result<Bech32Address, String> {
    let mut contract_interact = ContractInteract::new().await;
    let ping_amount = 1u64;
    let duration_in_seconds = 5u64;
    let opt_activation_timestamp = 2u64;
    let max_funds = 100_000u64;

    let (new_address, status, message) = contract_interact
        .interactor
        .tx()
        .from(&contract_interact.wallet_address)
        .gas(30_000_000u64)
        .typed(proxy::PingPongProxy)
        .init(
            BigUint::from(ping_amount),
            duration_in_seconds,
            Option::Some(opt_activation_timestamp),
            OptionalValue::Some(BigUint::from(max_funds)),
        )
        .code(&contract_interact.contract_code)
        .returns(ReturnsNewBech32Address)
        .returns(ReturnsStatus)
        .returns(ReturnsMessage)
        .run()
        .await;

    match status {
        0u64 => Ok(new_address),
        _ => Err(message),
    }
}

pub async fn ping() -> Result<String, String> {
    let mut contract_interact = ContractInteract::new().await;

    let amount = 1u64;

    let wallet_address = contract_interact.wallet_address.clone();
    let current_address = contract_interact.config.current_address().clone();
    let _data = IgnoreValue;

    contract_interact
        .interactor
        .tx()
        .from(wallet_address)
        .to(Bech32Address::from_bech32_string(current_address))
        .gas(30_000_000u64)
        .typed(proxy::PingPongProxy)
        .ping(_data)
        .egld(BigUint::from(amount))
        .run()
        .await;

    Ok("Ping successful!".to_string())
}
