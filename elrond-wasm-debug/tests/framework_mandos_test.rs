use elrond_wasm_debug::*;

// These tests don't really test any contract, but the testing framework itslef.

fn contract_map() -> ContractMap<TxContext> {
    ContractMap::new()
}

/// Checks that externalSteps work fine.
#[test]
fn external_steps_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/external_steps/external_steps.scen.json",
        &contract_map(),
    );
}

#[test]
fn transfer_rs() {
    elrond_wasm_debug::mandos_rs("tests/mandos/transfer.scen.json", &contract_map());
}

#[test]
fn validator_reward_rs() {
    elrond_wasm_debug::mandos_rs("tests/mandos/validatorReward.scen.json", &contract_map());
}
