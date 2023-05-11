use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "file:first-contract/output/first-contract.wasm",
        first_contract::ContractBuilder,
    );

    blockchain.register_contract(
        "file:second-contract/output/second-contract.wasm",
        second_contract::ContractBuilder,
    );
    blockchain
}

#[test]
fn init_rs() {
    world().run("scenarios/init.scen.json");
}

// TODO: implement ESDTTransfer + async call
#[test]
#[ignore]
fn reject_transfer_rs() {
    world().run("scenarios/reject_transfer.scen.json");
}

#[test]
fn simple_transfer_full_rs() {
    world().run("scenarios/simple_transfer_full.scen.json");
}

#[test]
fn simple_transfer_full_wrong_token_rs() {
    world().run("scenarios/simple_transfer_full_wrong_token.scen.json");
}

#[test]
fn simple_transfer_half_rs() {
    world().run("scenarios/simple_transfer_half.scen.json");
}
