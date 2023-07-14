use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn balance_of_go() {
    world().run("scenarios/balanceOf.scen.json");
}

#[test]
fn create_go() {
    world().run("scenarios/create.scen.json");
}

#[test]
fn exceptions_go() {
    world().run("scenarios/exceptions.scen.json");
}

#[test]
fn join_game_go() {
    world().run("scenarios/joinGame.scen.json");
}

#[test]
fn reward_and_send_to_wallet_go() {
    world().run("scenarios/rewardAndSendToWallet.scen.json");
}

#[test]
fn reward_winner_go() {
    world().run("scenarios/rewardWinner.scen.json");
}

#[test]
fn reward_winner_last_go() {
    world().run("scenarios/rewardWinner_Last.scen.json");
}

#[test]
fn top_up_ok_go() {
    world().run("scenarios/topUp_ok.scen.json");
}

#[test]
fn top_up_withdraw_go() {
    world().run("scenarios/topUp_withdraw.scen.json");
}

#[test]
fn withdraw_ok_go() {
    world().run("scenarios/withdraw_Ok.scen.json");
}

#[test]
fn withdraw_too_much_go() {
    world().run("scenarios/withdraw_TooMuch.scen.json");
}
