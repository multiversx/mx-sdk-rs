#[test]
fn balance_of_go() {
    multiversx_sc_scenario::run_go("scenarios/balanceOf.scen.json");
}

#[test]
fn create_go() {
    multiversx_sc_scenario::run_go("scenarios/create.scen.json");
}

#[test]
fn exceptions_go() {
    multiversx_sc_scenario::run_go("scenarios/exceptions.scen.json");
}

#[test]
fn join_game_go() {
    multiversx_sc_scenario::run_go("scenarios/joinGame.scen.json");
}

#[test]
fn reward_and_send_to_wallet_go() {
    multiversx_sc_scenario::run_go("scenarios/rewardAndSendToWallet.scen.json");
}

#[test]
fn reward_winner_go() {
    multiversx_sc_scenario::run_go("scenarios/rewardWinner.scen.json");
}

#[test]
fn reward_winner_last_go() {
    multiversx_sc_scenario::run_go("scenarios/rewardWinner_Last.scen.json");
}

#[test]
fn top_up_ok_go() {
    multiversx_sc_scenario::run_go("scenarios/topUp_ok.scen.json");
}

#[test]
fn top_up_withdraw_go() {
    multiversx_sc_scenario::run_go("scenarios/topUp_withdraw.scen.json");
}

#[test]
fn withdraw_ok_go() {
    multiversx_sc_scenario::run_go("scenarios/withdraw_Ok.scen.json");
}

#[test]
fn withdraw_too_much_go() {
    multiversx_sc_scenario::run_go("scenarios/withdraw_TooMuch.scen.json");
}
