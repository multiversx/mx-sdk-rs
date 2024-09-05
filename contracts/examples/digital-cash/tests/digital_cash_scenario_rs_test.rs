use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "mxsc:output/digital-cash.mxsc.json",
        digital_cash::ContractBuilder,
    );
    blockchain
}

#[test]
fn claim_egld_rs() {
    world().run("scenarios/claim-egld.scen.json");
}

#[test]
fn claim_esdt_rs() {
    world().run("scenarios/claim-esdt.scen.json");
}

#[test]
fn claim_fees_rs() {
    world().run("scenarios/claim-fees.scen.json");
}

#[test]
fn claim_multi_esdt_rs() {
    world().run("scenarios/claim-multi-esdt.scen.json");
}

#[test]
fn forward_rs() {
    world().run("scenarios/forward.scen.json");
}

#[test]
fn fund_egld_and_esdt_rs() {
    world().run("scenarios/fund-egld-and-esdt.scen.json");
}

#[test]
fn set_accounts_rs() {
    world().run("scenarios/set-accounts.scen.json");
}

#[test]
fn whitelist_blacklist_fee_token_rs() {
    world().run("scenarios/whitelist-blacklist-fee-tokens.scen.json");
}

#[test]
fn pay_fee_and_fund_esdt_rs() {
    world().run("scenarios/pay-fee-and-fund-esdt.scen.json");
}

#[test]
fn pay_fee_and_fund_egld_rs() {
    world().run("scenarios/pay-fee-and-fund-egld.scen.json");
}

#[test]
fn withdraw_egld_rs() {
    world().run("scenarios/withdraw-egld.scen.json");
}

#[test]
fn withdraw_esdt_rs() {
    world().run("scenarios/withdraw-esdt.scen.json");
}

#[test]
fn withdraw_multi_esdt_rs() {
    world().run("scenarios/withdraw-multi-esdt.scen.json");
}
