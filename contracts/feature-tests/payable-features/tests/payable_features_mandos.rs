use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<DebugApi> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/payable-features.wasm",
        Box::new(|context| Box::new(payable_features::contract_obj(context))),
    );
    contract_map
}

#[test]
fn call_value_check_rs() {
    elrond_wasm_debug::mandos_rs("mandos/call-value-check.scen.json", &contract_map());
}

#[test]
fn payable_multiple_rs() {
    elrond_wasm_debug::mandos_rs("mandos/payable_multiple.scen.json", &contract_map());
}

#[test]
fn payable_any_1_rs() {
    elrond_wasm_debug::mandos_rs("mandos/payable_any_1.scen.json", &contract_map());
}

#[test]
fn payable_any_2_rs() {
    elrond_wasm_debug::mandos_rs("mandos/payable_any_2.scen.json", &contract_map());
}

#[test]
fn payable_any_3_rs() {
    elrond_wasm_debug::mandos_rs("mandos/payable_any_3.scen.json", &contract_map());
}

#[test]
fn payable_any_4_rs() {
    elrond_wasm_debug::mandos_rs("mandos/payable_any_4.scen.json", &contract_map());
}

#[test]
fn payable_egld_1_rs() {
    elrond_wasm_debug::mandos_rs("mandos/payable_egld_1.scen.json", &contract_map());
}

#[test]
fn payable_egld_2_rs() {
    elrond_wasm_debug::mandos_rs("mandos/payable_egld_2.scen.json", &contract_map());
}

#[test]
fn payable_egld_3_rs() {
    elrond_wasm_debug::mandos_rs("mandos/payable_egld_3.scen.json", &contract_map());
}

#[test]
fn payable_egld_4_rs() {
    elrond_wasm_debug::mandos_rs("mandos/payable_egld_4.scen.json", &contract_map());
}

#[test]
fn payable_token_1_rs() {
    elrond_wasm_debug::mandos_rs("mandos/payable_token_1.scen.json", &contract_map());
}

#[test]
fn payable_token_2_rs() {
    elrond_wasm_debug::mandos_rs("mandos/payable_token_2.scen.json", &contract_map());
}

#[test]
fn payable_token_3_rs() {
    elrond_wasm_debug::mandos_rs("mandos/payable_token_3.scen.json", &contract_map());
}

#[test]
fn payable_token_4_rs() {
    elrond_wasm_debug::mandos_rs("mandos/payable_token_4.scen.json", &contract_map());
}
