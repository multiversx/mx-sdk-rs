use elrond_wasm_debug::*;

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
    elrond_wasm_debug::mandos_rs("mandos/deploy.scen.json", world());
}

#[test]
fn deposit_rs() {
    elrond_wasm_debug::mandos_rs("mandos/deposit.scen.json", world());
}

#[test]
fn set_bonding_curve_rs() {
    elrond_wasm_debug::mandos_rs("mandos/set_bonding_curve.scen.json", world());
}

#[test]
fn buy_rs() {
    elrond_wasm_debug::mandos_rs("mandos/buy.scen.json", world());
}

#[test]
fn sell_rs() {
    elrond_wasm_debug::mandos_rs("mandos/sell.scen.json", world());
}

#[test]
fn deposit_more_view_rs() {
    elrond_wasm_debug::mandos_rs("mandos/deposit_more_view.scen.json", world());
}

#[test]
fn claim_rs() {
    elrond_wasm_debug::mandos_rs("mandos/claim.scen.json", world());
}
