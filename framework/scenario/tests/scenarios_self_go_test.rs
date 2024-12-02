use multiversx_sc_scenario::*;

// These tests don't really test any contract, but the testing framework itself.

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

/// Checks that externalSteps work fine.
#[test]
fn external_steps_go() {
    world().run("tests/scenarios-self/external_steps/external_steps.scen.json");
}

#[test]
#[should_panic]
fn set_account_addr_len_err1_go() {
    world().run("tests/scenarios-self/set-check/set-account-addr-len.err1.json");
}

#[test]
#[should_panic]
fn set_account_addr_len_err2_go() {
    world().run("tests/scenarios-self/set-check/set-account-addr-len.err2.json");
}

#[test]
#[should_panic]
fn set_account_sc_addr_err1_go() {
    world().run("tests/scenarios-self/set-check/set-account-sc-addr.err1.json");
}

#[test]
#[should_panic]
fn set_account_sc_addr_err2_go() {
    world().run("tests/scenarios-self/set-check/set-account-sc-addr.err2.json");
}

#[test]
#[should_panic]
fn set_account_sc_addr_err3_go() {
    world().run("tests/scenarios-self/set-check/set-account-sc-addr.err3.json");
}

#[test]
#[should_panic]
fn set_check_balance_err_go() {
    world().run("tests/scenarios-self/set-check/set-check-balance.err.json");
}

#[test]
fn set_check_balance_go() {
    world().run("tests/scenarios-self/set-check/set-check-balance.scen.json");
}

#[test]
#[should_panic]
fn set_check_code_err_go() {
    world().run("tests/scenarios-self/set-check/set-check-code.err.json");
}

#[test]
fn set_check_code() {
    world().run("tests/scenarios-self/set-check/set-check-code.scen.json");
}

#[test]
#[should_panic]
fn set_check_codemetadata_err_go() {
    world().run("tests/scenarios-self/set-check/set-check-codemetadata.err.json");
}

#[test]
fn set_check_codemetadata() {
    world().run("tests/scenarios-self/set-check/set-check-codemetadata.scen.json");
}

#[test]
#[should_panic]
fn set_check_esdt_err_go() {
    world().run("tests/scenarios-self/set-check/set-check-esdt.err1.json");
}

#[test]
fn set_check_esdt_go() {
    world().run("tests/scenarios-self/set-check/set-check-esdt.scen.json");
}

#[test]
#[should_panic]
fn set_check_nonce_err_go() {
    world().run("tests/scenarios-self/set-check/set-check-nonce.err.json");
}

#[test]
fn set_check_nonce_go() {
    world().run("tests/scenarios-self/set-check/set-check-nonce.scen.json");
}

#[test]
#[should_panic]
fn set_check_storage_err1_go() {
    world().run("tests/scenarios-self/set-check/set-check-storage.err1.json");
}

#[test]
#[should_panic]
fn set_check_storage_err2_go() {
    world().run("tests/scenarios-self/set-check/set-check-storage.err2.json");
}

#[test]
#[should_panic]
fn set_check_storage_err3_go() {
    world().run("tests/scenarios-self/set-check/set-check-storage.err3.json");
}

#[test]
#[should_panic]
fn set_check_storage_err4_go() {
    world().run("tests/scenarios-self/set-check/set-check-storage.err4.json");
}

#[test]
#[should_panic]
fn set_check_storage_err5_go() {
    world().run("tests/scenarios-self/set-check/set-check-storage.err5.json");
}

#[test]
fn set_check_storage_go() {
    world().run("tests/scenarios-self/set-check/set-check-storage.scen.json");
}

#[test]
#[should_panic]
fn set_check_username_err_go() {
    world().run("tests/scenarios-self/set-check/set-check-username.err.json");
}

#[test]
fn set_check_username_go() {
    world().run("tests/scenarios-self/set-check/set-check-username.scen.json");
}

#[test]
fn builtin_func_esdt_transfer() {
    world().run("tests/scenarios-self/builtin-func-esdt-transfer.scen.json");
}

#[test]
#[should_panic]
fn esdt_non_zero_balance_check_err_go() {
    world().run("tests/scenarios-self/esdt-non-zero-balance-check-err.scen.json");
}

#[test]
#[should_panic]
fn esdt_zero_balance_check_err_go() {
    world().run("tests/scenarios-self/esdt-zero-balance-check-err.scen.json");
}

#[test]
fn multi_transfer_esdt_go() {
    world().run("tests/scenarios-self/multi-transfer-esdt.scen.json");
}

#[test]
fn transfer_egld_go() {
    world().run("tests/scenarios-self/transfer-egld.scen.json");
}

#[test]
fn transfer_esdt_go() {
    world().run("tests/scenarios-self/transfer-esdt.scen.json");
}

#[test]
fn validator_reward_go() {
    world().run("tests/scenarios-self/validatorReward.scen.json");
}
