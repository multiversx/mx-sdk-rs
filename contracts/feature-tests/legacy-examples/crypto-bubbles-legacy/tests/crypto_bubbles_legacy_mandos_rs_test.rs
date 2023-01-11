use mx_sc_scenario::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace(
        "contracts/feature-tests/legacy-examples/crypto-bubbles-legacy",
    );

    blockchain.register_contract(
        "file:output/crypto-bubbles-legacy.wasm",
        crypto_bubbles_legacy::ContractBuilder,
    );
    blockchain
}

#[test]
fn balanceof_rs() {
    mx_sc_scenario::scenario_rs("scenarios/balanceOf.scen.json", world());
}

#[test]
fn create_rs() {
    mx_sc_scenario::scenario_rs("scenarios/create.scen.json", world());
}

#[test]
fn exceptions_rs() {
    mx_sc_scenario::scenario_rs("scenarios/exceptions.scen.json", world());
}

#[test]
fn joingame_rs() {
    mx_sc_scenario::scenario_rs("scenarios/joinGame.scen.json", world());
}

#[test]
fn rewardandsendtowallet_rs() {
    mx_sc_scenario::scenario_rs("scenarios/rewardAndSendToWallet.scen.json", world());
}

#[test]
fn rewardwinner_rs() {
    mx_sc_scenario::scenario_rs("scenarios/rewardWinner.scen.json", world());
}

#[test]
fn rewardwinner_last_rs() {
    mx_sc_scenario::scenario_rs("scenarios/rewardWinner_Last.scen.json", world());
}

#[test]
fn topup_ok_rs() {
    mx_sc_scenario::scenario_rs("scenarios/topUp_ok.scen.json", world());
}

#[test]
fn topup_withdraw_rs() {
    mx_sc_scenario::scenario_rs("scenarios/topUp_withdraw.scen.json", world());
}

#[test]
fn withdraw_ok_rs() {
    mx_sc_scenario::scenario_rs("scenarios/withdraw_Ok.scen.json", world());
}

#[test]
fn withdraw_toomuch_rs() {
    mx_sc_scenario::scenario_rs("scenarios/withdraw_TooMuch.scen.json", world());
}
