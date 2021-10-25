use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crypto-bubbles");

    blockchain.register_contract(
        "file:output/crypto-bubbles.wasm",
        Box::new(|context| Box::new(crypto_bubbles::contract_obj(context))),
    );
    blockchain
}

#[test]
fn balanceof_rs() {
    elrond_wasm_debug::mandos_rs("mandos/balanceOf.scen.json", contract_map());
}

#[test]
fn create_rs() {
    elrond_wasm_debug::mandos_rs("mandos/create.scen.json", contract_map());
}

#[test]
fn exceptions_rs() {
    elrond_wasm_debug::mandos_rs("mandos/exceptions.scen.json", contract_map());
}

#[test]
fn joingame_rs() {
    elrond_wasm_debug::mandos_rs("mandos/joinGame.scen.json", contract_map());
}

#[test]
fn rewardandsendtowallet_rs() {
    elrond_wasm_debug::mandos_rs("mandos/rewardAndSendToWallet.scen.json", contract_map());
}

#[test]
fn rewardwinner_rs() {
    elrond_wasm_debug::mandos_rs("mandos/rewardWinner.scen.json", contract_map());
}

#[test]
fn rewardwinner_last_rs() {
    elrond_wasm_debug::mandos_rs("mandos/rewardWinner_Last.scen.json", contract_map());
}

#[test]
fn topup_ok_rs() {
    elrond_wasm_debug::mandos_rs("mandos/topUp_ok.scen.json", contract_map());
}

#[test]
fn topup_withdraw_rs() {
    elrond_wasm_debug::mandos_rs("mandos/topUp_withdraw.scen.json", contract_map());
}

#[test]
fn withdraw_ok_rs() {
    elrond_wasm_debug::mandos_rs("mandos/withdraw_Ok.scen.json", contract_map());
}

#[test]
fn withdraw_toomuch_rs() {
    elrond_wasm_debug::mandos_rs("mandos/withdraw_TooMuch.scen.json", contract_map());
}
