// use elrond_wasm_debug::*;

// fn world() -> BlockchainMock {
//     let mut blockchain = BlockchainMock::new();
//     blockchain.set_current_dir_from_workspace("contracts/examples/digital-cash");

//     blockchain.register_contract_builder(
//         "file:output/digital-cash.wasm",
//         digital_cash::ContractBuilder,
//     );
//     blockchain
// }

// verify_ed25519 not implemented
// #[test]
// fn claim_egld_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/claim-egld.scen.json", world());
// }

// verify_ed25519 not implemented
// #[test]
// fn claim_esdt_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/claim-esdt.scen.json", world());
// }

// #[test]
// fn fund_egld_and_esdt_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/fund-egld-and-esdt.scen.json", world());
// }

// #[test]
// fn set_accounts_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/set-accounts.scen.json", world());
// }

// #[test]
// fn withdraw_egld_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/withdraw-egld.scen.json", world());
// }

// #[test]
// fn withdraw_esdt_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/withdraw-esdt.scen.json", world());
// }
