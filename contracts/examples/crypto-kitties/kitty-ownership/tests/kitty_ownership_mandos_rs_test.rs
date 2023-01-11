use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "file:../kitty-genetic-alg/output/kitty-genetic-alg.wasm",
        kitty_genetic_alg::ContractBuilder,
    );
    blockchain.register_contract(
        "file:output/kitty-ownership.wasm",
        kitty_ownership::ContractBuilder,
    );

    blockchain
}

#[test]
fn approve_siring_rs() {
    multiversx_sc_scenario::run_rs("scenarios/approve_siring.scen.json", world());
}

#[test]
fn breed_ok_rs() {
    multiversx_sc_scenario::run_rs("scenarios/breed_ok.scen.json", world());
}

#[test]
fn give_birth_rs() {
    multiversx_sc_scenario::run_rs("scenarios/give_birth.scen.json", world());
}

#[test]
fn init_rs() {
    multiversx_sc_scenario::run_rs("scenarios/init.scen.json", world());
}

#[test]
fn query_rs() {
    multiversx_sc_scenario::run_rs("scenarios/query.scen.json", world());
}

#[test]
fn setup_accounts_rs() {
    multiversx_sc_scenario::run_rs("scenarios/setup_accounts.scen.json", world());
}
