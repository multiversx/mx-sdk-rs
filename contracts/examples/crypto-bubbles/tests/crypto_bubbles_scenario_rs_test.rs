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
    world().run("scenarios/balanceOf.scen.json");
}

#[test]
fn create_rs() {
    world().run("scenarios/create.scen.json");
}

#[test]
fn exceptions_rs() {
    world().run("scenarios/exceptions.scen.json");
}

#[test]
fn join_game_rs() {
    world().run("scenarios/joinGame.scen.json");
}

#[test]
fn reward_and_send_to_wallet_rs() {
    world().run("scenarios/rewardAndSendToWallet.scen.json");
}

#[test]
fn reward_winner_rs() {
    world().run("scenarios/rewardWinner.scen.json");
}

#[test]
fn reward_winner_last_rs() {
    world().run("scenarios/rewardWinner_Last.scen.json");
}

#[test]
fn top_up_ok_rs() {
    world().run("scenarios/topUp_ok.scen.json");
}

#[test]
fn top_up_withdraw_rs() {
    world().run("scenarios/topUp_withdraw.scen.json");
}

#[test]
fn withdraw_ok_rs() {
    world().run("scenarios/withdraw_Ok.scen.json");
}

#[test]
fn withdraw_too_much_rs() {
    world().run("scenarios/withdraw_TooMuch.scen.json");
}
