use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract("file:output/erc1155.wasm", erc1155::ContractBuilder);
    blockchain.register_contract(
        "file:../erc1155-user-mock/output/erc1155-user-mock.wasm",
        erc1155_user_mock::ContractBuilder,
    );

    blockchain
}

#[test]
fn batch_transfer_both_types_rs() {
    world().run("scenarios/batch_transfer_both_types.scen.json");
}

#[test]
fn batch_transfer_both_types_to_sc_rs() {
    world().run("scenarios/batch_transfer_both_types_to_sc.scen.json");
}

#[test]
fn batch_transfer_fungible_rs() {
    world().run("scenarios/batch_transfer_fungible.scen.json");
}

#[test]
fn batch_transfer_fungible_to_sc_rs() {
    world().run("scenarios/batch_transfer_fungible_to_sc.scen.json");
}

#[test]
fn batch_transfer_non_fungible_rs() {
    world().run("scenarios/batch_transfer_non_fungible.scen.json");
}

#[test]
fn batch_transfer_non_fungible_to_sc_rs() {
    world().run("scenarios/batch_transfer_non_fungible_to_sc.scen.json");
}

// burn tests
#[test]
fn burn_fungible_rs() {
    world().run("scenarios/burn_fungible.scen.json");
}

#[test]
fn burn_non_fungible_rs() {
    world().run("scenarios/burn_non_fungible.scen.json");
}

#[test]
fn create_one_fungible_one_non_fungible_rs() {
    world().run("scenarios/create_one_fungible_one_non_fungible.scen.json");
}

// Create token tests
#[test]
fn create_token_fungible_rs() {
    world().run("scenarios/create_token_fungible.scen.json");
}

#[test]
fn create_token_non_fungible_rs() {
    world().run("scenarios/create_token_non_fungible.scen.json");
}

#[test]
fn create_two_tokens_both_fungible_different_creator_rs() {
    world().run("scenarios/create_two_tokens_both_fungible_different_creator.scen.json");
}

#[test]
fn create_two_tokens_both_fungible_same_creator_rs() {
    world().run("scenarios/create_two_tokens_both_fungible_same_creator.scen.json");
}

#[test]
fn create_two_tokens_both_non_fungible_same_creator_rs() {
    world().run("scenarios/create_two_tokens_both_non_fungible_same_creator.scen.json");
}

#[test]
fn deploy_rs() {
    world().run("scenarios/deploy.scen.json");
}

// mint tests
#[test]
fn mint_fungible_rs() {
    world().run("scenarios/mint_fungible.scen.json");
}

#[test]
fn mint_non_fungible_rs() {
    world().run("scenarios/mint_non_fungible.scen.json");
}

#[test]
fn mint_not_creator_rs() {
    world().run("scenarios/mint_not_creator.scen.json");
}

#[test]
fn transfer_fungible_not_enough_balance_rs() {
    world().run("scenarios/transfer_fungible_not_enough_balance.scen.json");
}

// transfer tests -  to account
#[test]
fn transfer_fungible_ok_rs() {
    world().run("scenarios/transfer_fungible_ok.scen.json");
}

#[test]
fn transfer_fungible_ok_to_sc_rs() {
    world().run("scenarios/transfer_fungible_ok_to_sc.scen.json");
}

#[test]
fn transfer_non_fungible_ok_rs() {
    world().run("scenarios/transfer_non_fungible_ok.scen.json");
}

#[test]
fn transfer_non_fungible_ok_to_sc_rs() {
    world().run("scenarios/transfer_non_fungible_ok_to_sc.scen.json");
}
