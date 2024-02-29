use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace(
        "contracts/feature-tests/erc-style-contracts/crowdfunding-erc20",
    );

    blockchain.register_contract(
        "file:output/crowdfunding-erc20.wasm",
        crowdfunding_erc20::ContractBuilder,
    );

    blockchain.register_contract("file:../erc20/output/erc20.wasm", erc20::ContractBuilder);

    blockchain
}

#[test]
fn deploy_erc_20_and_crowdfunding_rs() {
    world().run("scenarios/deploy_erc20_and_crowdfunding.scen.json");
}

#[test]
fn fund_with_insufficient_allowance_rs() {
    world().run("scenarios/fund_with_insufficient_allowance.scen.json");
}

#[test]
fn fund_with_sufficient_allowance_rs() {
    world().run("scenarios/fund_with_sufficient_allowance.scen.json");
}

#[test]
fn fund_without_allowance_rs() {
    world().run("scenarios/fund_without_allowance.scen.json");
}
