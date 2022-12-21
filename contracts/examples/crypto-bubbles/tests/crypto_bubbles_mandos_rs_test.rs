use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crypto-bubbles");

    blockchain.register_contract(
        "file:output/crypto-bubbles.wasm",
        crypto_bubbles::ContractBuilder,
    );
    blockchain
}

#[test]
fn balanceof_rs() {
    mx_sc_debug::mandos_rs("mandos/balanceOf.scen.json", world());
}

#[test]
fn create_rs() {
    mx_sc_debug::mandos_rs("mandos/create.scen.json", world());
}

#[test]
fn exceptions_rs() {
    mx_sc_debug::mandos_rs("mandos/exceptions.scen.json", world());
}

#[test]
fn joingame_rs() {
    mx_sc_debug::mandos_rs("mandos/joinGame.scen.json", world());
}

#[test]
fn rewardandsendtowallet_rs() {
    mx_sc_debug::mandos_rs("mandos/rewardAndSendToWallet.scen.json", world());
}

#[test]
fn rewardwinner_rs() {
    mx_sc_debug::mandos_rs("mandos/rewardWinner.scen.json", world());
}

#[test]
fn rewardwinner_last_rs() {
    mx_sc_debug::mandos_rs("mandos/rewardWinner_Last.scen.json", world());
}

#[test]
fn topup_ok_rs() {
    mx_sc_debug::mandos_rs("mandos/topUp_ok.scen.json", world());
}

#[test]
fn topup_withdraw_rs() {
    mx_sc_debug::mandos_rs("mandos/topUp_withdraw.scen.json", world());
}

#[test]
fn withdraw_ok_rs() {
    mx_sc_debug::mandos_rs("mandos/withdraw_Ok.scen.json", world());
}

#[test]
fn withdraw_toomuch_rs() {
    mx_sc_debug::mandos_rs("mandos/withdraw_TooMuch.scen.json", world());
}
