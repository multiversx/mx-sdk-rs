use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<DebugApi> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../../output/multisig.wasm",
        Box::new(|context| Box::new(multisig::contract_obj(context))),
    );
    contract_map
}

#[test]
fn test_change_board_rs() {
    elrond_wasm_debug::mandos_rs("mandos/changeBoard.scen.json", contract_map());
}

#[test]
fn test_change_quorum_rs() {
    elrond_wasm_debug::mandos_rs("mandos/changeQuorum.scen.json", contract_map());
}

#[test]
fn test_change_quorum_too_big_rs() {
    elrond_wasm_debug::mandos_rs("mandos/changeQuorum_tooBig.scen.json", contract_map());
}
