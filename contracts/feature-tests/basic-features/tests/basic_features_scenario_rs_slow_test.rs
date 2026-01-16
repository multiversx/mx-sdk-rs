//! These tests are too slow when running with wasmer-experimental.
//!
//! They are moved here, where they are only run using the debugger. With the debugger they run fast.

use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::Debugger);

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/basic-features");

    blockchain.register_contract(
        "mxsc:output/basic-features.mxsc.json",
        basic_features::ContractBuilder,
    );

    blockchain
}

#[test]
fn big_num_ops_arith_rs() {
    world().run("scenarios/big_num_ops_arith.scen.json");
}

#[test]
fn big_num_ops_bitwise_rs() {
    world().run("scenarios/big_num_ops_bitwise.scen.json");
}

#[test]
fn big_num_ops_shift_rs() {
    world().run("scenarios/big_num_ops_shift.scen.json");
}
