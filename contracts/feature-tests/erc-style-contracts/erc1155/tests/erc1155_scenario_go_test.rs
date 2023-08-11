use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn batch_transfer_both_types_go() {
    world().run("scenarios/batch_transfer_both_types.scen.json");
}

#[test]
fn batch_transfer_both_types_to_sc_go() {
    world().run("scenarios/batch_transfer_both_types_to_sc.scen.json");
}

#[test]
fn batch_transfer_fungible_go() {
    world().run("scenarios/batch_transfer_fungible.scen.json");
}

#[test]
fn batch_transfer_fungible_to_sc_go() {
    world().run("scenarios/batch_transfer_fungible_to_sc.scen.json");
}

#[test]
fn batch_transfer_non_fungible_go() {
    world().run("scenarios/batch_transfer_non_fungible.scen.json");
}

#[test]
fn batch_transfer_non_fungible_to_sc_go() {
    world().run("scenarios/batch_transfer_non_fungible_to_sc.scen.json");
}

#[test]
fn burn_fungible_go() {
    world().run("scenarios/burn_fungible.scen.json");
}

#[test]
fn burn_non_fungible_go() {
    world().run("scenarios/burn_non_fungible.scen.json");
}

#[test]
fn create_one_fungible_one_non_fungible_go() {
    world().run("scenarios/create_one_fungible_one_non_fungible.scen.json");
}

#[test]
fn create_token_fungible_go() {
    world().run("scenarios/create_token_fungible.scen.json");
}

#[test]
fn create_token_non_fungible_go() {
    world().run("scenarios/create_token_non_fungible.scen.json");
}

#[test]
fn create_two_tokens_both_fungible_different_creator_go() {
    world().run("scenarios/create_two_tokens_both_fungible_different_creator.scen.json");
}

#[test]
fn create_two_tokens_both_fungible_same_creator_go() {
    world().run("scenarios/create_two_tokens_both_fungible_same_creator.scen.json");
}

#[test]
fn create_two_tokens_both_non_fungible_same_creator_go() {
    world().run("scenarios/create_two_tokens_both_non_fungible_same_creator.scen.json");
}

#[test]
fn deploy_go() {
    world().run("scenarios/deploy.scen.json");
}

#[test]
fn mint_fungible_go() {
    world().run("scenarios/mint_fungible.scen.json");
}

#[test]
fn mint_non_fungible_go() {
    world().run("scenarios/mint_non_fungible.scen.json");
}

#[test]
fn mint_not_creator_go() {
    world().run("scenarios/mint_not_creator.scen.json");
}

#[test]
fn transfer_fungible_not_enough_balance_go() {
    world().run("scenarios/transfer_fungible_not_enough_balance.scen.json");
}

#[test]
fn transfer_fungible_ok_go() {
    world().run("scenarios/transfer_fungible_ok.scen.json");
}

#[test]
fn transfer_fungible_ok_to_sc_go() {
    world().run("scenarios/transfer_fungible_ok_to_sc.scen.json");
}

#[test]
fn transfer_non_fungible_ok_go() {
    world().run("scenarios/transfer_non_fungible_ok.scen.json");
}

#[test]
fn transfer_non_fungible_ok_to_sc_go() {
    world().run("scenarios/transfer_non_fungible_ok_to_sc.scen.json");
}
