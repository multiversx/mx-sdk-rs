use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crowdfunding-esdt");

    blockchain.register_contract(
        "file:output/crowdfunding-esdt.wasm",
        Box::new(|context| Box::new(crowdfunding_esdt::contract_obj(context))),
    );
    blockchain
}

#[test]
fn crowdfunding_claim_failed_rs() {
    elrond_wasm_debug::mandos_rs("mandos/crowdfunding-claim-failed.scen.json", contract_map());
}

#[test]
fn crowdfunding_claim_successful_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/crowdfunding-claim-successful.scen.json",
        contract_map(),
    );
}

#[test]
fn crowdfunding_claim_too_early_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/crowdfunding-claim-too-early.scen.json",
        contract_map(),
    );
}

#[test]
fn crowdfunding_fund_rs() {
    elrond_wasm_debug::mandos_rs("mandos/crowdfunding-fund.scen.json", contract_map());
}

#[test]
fn crowdfunding_fund_too_late_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/crowdfunding-fund-too-late.scen.json",
        contract_map(),
    );
}

#[test]
fn crowdfunding_init_rs() {
    elrond_wasm_debug::mandos_rs("mandos/crowdfunding-init.scen.json", contract_map());
}

#[test]
fn egld_crowdfunding_claim_failed_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/egld-crowdfunding-claim-failed.scen.json",
        contract_map(),
    );
}

#[test]
fn egld_crowdfunding_claim_successful_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/egld-crowdfunding-claim-successful.scen.json",
        contract_map(),
    );
}

#[test]
fn egld_crowdfunding_claim_too_early_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/egld-crowdfunding-claim-too-early.scen.json",
        contract_map(),
    );
}

#[test]
fn egld_crowdfunding_fund_rs() {
    elrond_wasm_debug::mandos_rs("mandos/egld-crowdfunding-fund.scen.json", contract_map());
}

#[test]
fn egld_crowdfunding_fund_too_late_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/egld-crowdfunding-fund-too-late.scen.json",
        contract_map(),
    );
}

#[test]
fn egld_crowdfunding_init_rs() {
    elrond_wasm_debug::mandos_rs("mandos/egld-crowdfunding-init.scen.json", contract_map());
}
