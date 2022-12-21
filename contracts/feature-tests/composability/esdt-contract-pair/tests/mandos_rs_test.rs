use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:first-contract/output/first-contract.wasm",
        first_contract::ContractBuilder,
    );

    blockchain.register_contract(
        "file:second-contract/output/second-contract.wasm",
        second_contract::ContractBuilder,
    );
    blockchain
}

#[test]
fn init_rs() {
    mx_sc_debug::mandos_rs("mandos/init.scen.json", world());
}

#[test]
fn simple_transfer_full_rs() {
    mx_sc_debug::mandos_rs("mandos/simple_transfer_full.scen.json", world());
}

#[test]
fn simple_transfer_half_rs() {
    mx_sc_debug::mandos_rs("mandos/simple_transfer_half.scen.json", world());
}

#[test]
fn simple_transfer_full_wrong_token_rs() {
    mx_sc_debug::mandos_rs("mandos/simple_transfer_full_wrong_token.scen.json", world());
}

// TODO: implement ESDTTransfer + async call
// #[test]
// fn rejected_transfer_rs() {
// 	mx_sc_debug::mandos_rs("mandos/reject_transfer.scen.json", world());
// }
