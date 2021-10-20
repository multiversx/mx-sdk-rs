use elrond_wasm_debug::*;

// These tests don't really test any contract, but the testing framework itslef.

fn contract_map() -> ContractMap<DebugApi> {
    ContractMap::new()
}

/// Checks that externalSteps work fine.
#[test]
fn external_steps_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/external_steps/external_steps.scen.json",
        contract_map(),
    );
}

#[test]
#[should_panic]
fn set_account_addr_len_err1_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-account-addr-len.err1.json",
        contract_map(),
    );
}

#[test]
#[should_panic]
fn set_account_addr_len_err2_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-account-addr-len.err2.json",
        contract_map(),
    );
}

#[test]
#[should_panic]
fn set_account_sc_addr_err1_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-account-sc-addr.err1.json",
        contract_map(),
    );
}

#[test]
#[should_panic]
fn set_account_sc_addr_err2_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-account-sc-addr.err2.json",
        contract_map(),
    );
}

#[test]
#[should_panic]
fn set_account_sc_addr_err3_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-account-sc-addr.err3.json",
        contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_balance_err_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-check-balance.err.json",
        contract_map(),
    );
}

#[test]
fn set_check_balance_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-check-balance.scen.json",
        contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_code_err_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-check-code.err.json",
        contract_map(),
    );
}

#[test]
fn set_check_code() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-check-code.scen.json",
        contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_esdt_err_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-check-esdt.err.json",
        contract_map(),
    );
}

#[test]
fn set_check_esdt_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-check-esdt.scen.json",
        contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_nonce_err_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-check-nonce.err.json",
        contract_map(),
    );
}

#[test]
fn set_check_nonce_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-check-nonce.scen.json",
        contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err1_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-check-storage.err1.json",
        contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err2_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-check-storage.err2.json",
        contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err3_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-check-storage.err3.json",
        contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err4_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-check-storage.err4.json",
        contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err5_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-check-storage.err5.json",
        contract_map(),
    );
}

#[test]
fn set_check_storage_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-check-storage.scen.json",
        contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_username_err_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-check-username.err.json",
        contract_map(),
    );
}

#[test]
fn set_check_username_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/set-check/set-check-username.scen.json",
        contract_map(),
    );
}
/*  TODO: implement & test
#[test]
fn builtin_func_esdt_transfer() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/builtin-func-esdt-transfer.scen.json",
        contract_map(),
    );
}
*/
#[test]
#[should_panic]
fn esdt_non_zero_balance_check_err_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/esdt-non-zero-balance-check-err.scen.json",
        contract_map(),
    );
}

#[test]
#[should_panic]
fn esdt_zero_balance_check_err_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/esdt-zero-balance-check-err.scen.json",
        contract_map(),
    );
}
/* TODO: implement & test
#[test]
fn multi_transfer_esdt_rs() {
    elrond_wasm_debug::mandos_rs(
        "tests/mandos/multi-transfer-esdt.scen.json",
        contract_map(),
    );
}
*/
#[test]
fn transfer_egld_rs() {
    elrond_wasm_debug::mandos_rs("tests/mandos/transfer-egld.scen.json", contract_map());
}
/* TODO: implement & test
#[test]
fn transfer_esdt_rs() {
    elrond_wasm_debug::mandos_rs("tests/mandos/transfer-esdt.scen.json", contract_map());
}
*/
#[test]
fn validator_reward_rs() {
    elrond_wasm_debug::mandos_rs("tests/mandos/validatorReward.scen.json", contract_map());
}
