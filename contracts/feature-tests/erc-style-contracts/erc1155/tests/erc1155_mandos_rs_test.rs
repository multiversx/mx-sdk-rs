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
fn deploy_test_rs() {
    multiversx_sc_scenario::run_rs("scenarios/deploy.scen.json", world());
}

// Create token tests

#[test]
fn create_token_fungible_test_rs() {
    multiversx_sc_scenario::run_rs("scenarios/create_token_fungible.scen.json", world());
}

#[test]
fn create_token_non_fungible_test_rs() {
    multiversx_sc_scenario::run_rs("scenarios/create_token_non_fungible.scen.json", world());
}

#[test]
fn create_two_fungible_same_creator_test_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/create_two_tokens_both_fungible_same_creator.scen.json",
        world(),
    );
}

#[test]
fn create_two_fungible_different_creator_test_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/create_two_tokens_both_fungible_different_creator.scen.json",
        world(),
    );
}

#[test]
fn create_two_non_fungible_same_creator_test_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/create_two_tokens_both_non_fungible_same_creator.scen.json",
        world(),
    );
}

#[test]
fn create_one_fungible_one_non_fungible_test_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/create_one_fungible_one_non_fungible.scen.json",
        world(),
    );
}

// transfer tests -  to account
#[test]
fn transfer_fungible_ok_test_rs() {
    multiversx_sc_scenario::run_rs("scenarios/transfer_fungible_ok.scen.json", world());
}

#[test]
fn transfer_fungible_not_enough_balance_test_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transfer_fungible_not_enough_balance.scen.json",
        world(),
    );
}

#[test]
fn transfer_non_fungible_ok_test_rs() {
    multiversx_sc_scenario::run_rs("scenarios/transfer_non_fungible_ok.scen.json", world());
}

#[test]
fn batch_transfer_fungible_test_rs() {
    multiversx_sc_scenario::run_rs("scenarios/batch_transfer_fungible.scen.json", world());
}

#[test]
fn batch_transfer_non_fungible_test_rs() {
    multiversx_sc_scenario::run_rs("scenarios/batch_transfer_non_fungible.scen.json", world());
}

#[test]
fn batch_transfer_both_types_test_rs() {
    multiversx_sc_scenario::run_rs("scenarios/batch_transfer_both_types.scen.json", world());
}

#[test]
fn transfer_fungible_ok_to_sc_test_rs() {
    multiversx_sc_scenario::run_rs("scenarios/transfer_fungible_ok_to_sc.scen.json", world());
}

#[test]
fn transfer_non_fungible_ok_to_sc_test_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/transfer_non_fungible_ok_to_sc.scen.json",
        world(),
    );
}

#[test]
fn batch_transfer_fungible_to_sc_test_rs() {
    multiversx_sc_scenario::run_rs("scenarios/batch_transfer_fungible_to_sc.scen.json", world());
}

#[test]
fn batch_transfer_non_fungible_to_sc_test_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/batch_transfer_non_fungible_to_sc.scen.json",
        world(),
    );
}

#[test]
fn batch_transfer_both_types_to_sc_test_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/batch_transfer_both_types_to_sc.scen.json",
        world(),
    );
}

// mint tests

#[test]
fn mint_fungible_test_rs() {
    multiversx_sc_scenario::run_rs("scenarios/mint_fungible.scen.json", world());
}

#[test]
fn mint_non_fungible_test_rs() {
    multiversx_sc_scenario::run_rs("scenarios/mint_non_fungible.scen.json", world());
}

#[test]
fn mint_not_creator_test_rs() {
    multiversx_sc_scenario::run_rs("scenarios/mint_not_creator.scen.json", world());
}

// burn tests

#[test]
fn burn_fungible_test_rs() {
    multiversx_sc_scenario::run_rs("scenarios/burn_fungible.scen.json", world());
}

#[test]
fn burn_non_fungible_test_rs() {
    multiversx_sc_scenario::run_rs("scenarios/burn_non_fungible.scen.json", world());
}
