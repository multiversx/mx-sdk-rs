use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/big-float-features");

    blockchain.register_contract(
        "file:output/big-float-features.wasm",
        big_float_features::ContractBuilder,
    );
    blockchain.register_contract(
        "file:../esdt-system-sc-mock/output/esdt-system-sc-mock.wasm",
        esdt_system_sc_mock::ContractBuilder,
    );

    blockchain
}

#[test]
fn big_float_new_from_big_int_rs() {
    elrond_wasm_debug::mandos_rs("mandos/big_float_new_from_big_int.scen.json", world());
}

#[test]
fn big_float_new_from_big_uint_rs() {
    elrond_wasm_debug::mandos_rs("mandos/big_float_new_from_big_uint.scen.json", world());
}

#[test]
fn big_float_new_from_frac_rs() {
    elrond_wasm_debug::mandos_rs("mandos/big_float_new_from_frac.scen.json", world());
}

#[test]
fn big_float_new_from_int_rs() {
    elrond_wasm_debug::mandos_rs("mandos/big_float_new_from_int.scen.json", world());
}

// #[test]
// fn big_float_new_from_managed_buffer_rs() {
//     elrond_wasm_debug::mandos_rs(
//         "mandos/big_float_new_from_managed_buffer.scen.json",
//         world(),
//     );
// }

#[test]
fn big_float_new_from_parts_rs() {
    elrond_wasm_debug::mandos_rs("mandos/big_float_new_from_parts.scen.json", world());
}

#[test]
fn big_float_new_from_sci_rs() {
    elrond_wasm_debug::mandos_rs("mandos/big_float_new_from_sci.scen.json", world());
}

#[test]
fn big_float_operators_rs() {
    elrond_wasm_debug::mandos_rs("mandos/big_float_operators.scen.json", world());
}

// #[test]
// fn big_float_operator_checks_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/big_float_operator_checks.scen.json", world());
// }
