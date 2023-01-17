use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crowdfunding-esdt");

    blockchain.register_contract(
        "file:output/crowdfunding-esdt.wasm",
        crowdfunding_esdt::ContractBuilder,
    );
    blockchain
}

#[test]
fn crowdfunding_claim_failed_rs() {
    multiversx_sc_scenario::run_rs("scenarios/crowdfunding-claim-failed.scen.json", world());
}

#[test]
fn crowdfunding_claim_successful_rs() {
    multiversx_sc_scenario::run_rs("scenarios/crowdfunding-claim-successful.scen.json", world());
}

#[test]
fn crowdfunding_claim_too_early_rs() {
    multiversx_sc_scenario::run_rs("scenarios/crowdfunding-claim-too-early.scen.json", world());
}

#[test]
fn crowdfunding_fund_rs() {
    multiversx_sc_scenario::run_rs("scenarios/crowdfunding-fund.scen.json", world());
}

#[test]
fn crowdfunding_fund_too_late_rs() {
    multiversx_sc_scenario::run_rs("scenarios/crowdfunding-fund-too-late.scen.json", world());
}

#[test]
fn crowdfunding_init_rs() {
    multiversx_sc_scenario::run_rs("scenarios/crowdfunding-init.scen.json", world());
}

#[test]
fn egld_crowdfunding_claim_failed_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/egld-crowdfunding-claim-failed.scen.json",
        world(),
    );
}

#[test]
fn egld_crowdfunding_claim_successful_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/egld-crowdfunding-claim-successful.scen.json",
        world(),
    );
}

#[test]
fn egld_crowdfunding_claim_too_early_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/egld-crowdfunding-claim-too-early.scen.json",
        world(),
    );
}

#[test]
fn egld_crowdfunding_fund_rs() {
    multiversx_sc_scenario::run_rs("scenarios/egld-crowdfunding-fund.scen.json", world());
}

#[test]
fn egld_crowdfunding_fund_too_late_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/egld-crowdfunding-fund-too-late.scen.json",
        world(),
    );
}

#[test]
fn egld_crowdfunding_init_rs() {
    multiversx_sc_scenario::run_rs("scenarios/egld-crowdfunding-init.scen.json", world());
}
