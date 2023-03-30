use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crypto-bubbles");

    blockchain.register_contract(
        "file:output/crypto-bubbles.wasm",
        crypto_bubbles::ContractBuilder,
    );
    blockchain
}

#[test]
fn balance_of_rs() {
    multiversx_sc_scenario::run_rs("scenarios/balanceOf.scen.json", world());
}

#[test]
fn create_rs() {
    multiversx_sc_scenario::run_rs("scenarios/create.scen.json", world());
}

#[test]
fn exceptions_rs() {
    multiversx_sc_scenario::run_rs("scenarios/exceptions.scen.json", world());
}

#[test]
fn join_game_rs() {
    multiversx_sc_scenario::run_rs("scenarios/joinGame.scen.json", world());
}

#[test]
fn reward_and_send_to_wallet_rs() {
    multiversx_sc_scenario::run_rs("scenarios/rewardAndSendToWallet.scen.json", world());
}

#[test]
fn reward_winner_rs() {
    multiversx_sc_scenario::run_rs("scenarios/rewardWinner.scen.json", world());
}

#[test]
fn reward_winner_last_rs() {
    multiversx_sc_scenario::run_rs("scenarios/rewardWinner_Last.scen.json", world());
}

#[test]
fn top_up_ok_rs() {
    multiversx_sc_scenario::run_rs("scenarios/topUp_ok.scen.json", world());
}

#[test]
fn top_up_withdraw_rs() {
    multiversx_sc_scenario::run_rs("scenarios/topUp_withdraw.scen.json", world());
}

#[test]
fn withdraw_ok_rs() {
    multiversx_sc_scenario::run_rs("scenarios/withdraw_Ok.scen.json", world());
}

#[test]
fn withdraw_too_much_rs() {
    multiversx_sc_scenario::run_rs("scenarios/withdraw_TooMuch.scen.json", world());
}
