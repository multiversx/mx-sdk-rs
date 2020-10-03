
extern crate crypto_bubbles;
use crypto_bubbles::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/crypto-bubbles.wasm",
        Box::new(|context| Box::new(CryptoBubblesImpl::new(context))));
    contract_map
}

#[test]
fn balanceof() {
    parse_execute_mandos("mandos/balanceOf.scen.json", &contract_map());
}

#[test]
fn create() {
    parse_execute_mandos("mandos/create.scen.json", &contract_map());
}

#[test]
fn exceptions() {
    parse_execute_mandos("mandos/exceptions.scen.json", &contract_map());
}

#[test]
fn joingame() {
    parse_execute_mandos("mandos/joinGame.scen.json", &contract_map());
}

#[test]
fn rewardandsendtowallet() {
    parse_execute_mandos("mandos/rewardAndSendToWallet.scen.json", &contract_map());
}

#[test]
fn rewardwinner_last() {
    parse_execute_mandos("mandos/rewardWinner_Last.scen.json", &contract_map());
}

#[test]
fn rewardwinner() {
    parse_execute_mandos("mandos/rewardWinner.scen.json", &contract_map());
}

#[test]
fn topup_ok() {
    parse_execute_mandos("mandos/topUp_ok.scen.json", &contract_map());
}

#[test]
fn topup_withdraw() {
    parse_execute_mandos("mandos/topUp_withdraw.scen.json", &contract_map());
}

#[test]
fn withdraw_ok() {
    parse_execute_mandos("mandos/withdraw_Ok.scen.json", &contract_map());
}

#[test]
fn withdraw_toomuch() {
    parse_execute_mandos("mandos/withdraw_TooMuch.scen.json", &contract_map());
}
