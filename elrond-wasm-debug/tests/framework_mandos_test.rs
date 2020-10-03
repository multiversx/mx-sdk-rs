
use elrond_wasm_debug::*;

// These tests don't really test any contract, but the testing framework itslef.

fn contract_map() -> ContractMap<TxContext> {
    ContractMap::new()
}

/// It just checks that externalSteps work fine.
#[test]
fn external_steps() {
    parse_execute_mandos("tests/mandos/external_steps/external_steps.scen.json", &contract_map());    
}

