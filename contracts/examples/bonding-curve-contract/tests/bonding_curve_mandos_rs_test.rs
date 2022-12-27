use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:output/bonding-curve-contract.wasm",
        bonding_curve_contract::ContractBuilder,
    );
    blockchain
}

#[test]
fn deploy_rs() {
    mx_sc_debug::mandos_rs("scenarios/deploy.scen.json", world());
}

#[test]
fn deposit_rs() {
    mx_sc_debug::mandos_rs("scenarios/deposit.scen.json", world());
}

#[test]
fn set_bonding_curve_rs() {
    mx_sc_debug::mandos_rs("scenarios/set_bonding_curve.scen.json", world());
}

#[test]
fn buy_rs() {
    mx_sc_debug::mandos_rs("scenarios/buy.scen.json", world());
}

#[test]
fn sell_rs() {
    mx_sc_debug::mandos_rs("scenarios/sell.scen.json", world());
}

#[test]
fn deposit_more_view_rs() {
    mx_sc_debug::mandos_rs("scenarios/deposit_more_view.scen.json", world());
}

#[test]
fn claim_rs() {
    mx_sc_debug::mandos_rs("scenarios/claim.scen.json", world());
}
