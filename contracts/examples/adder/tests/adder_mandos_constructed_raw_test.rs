use elrond_wasm_debug::{mandos::model::*, *};

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract_builder("file:output/adder.wasm", adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_mandos_constructed_raw() {
    let world = world();
    let intp_context = world.interpreter_context();
    let _ = world
        .mandos_set_state(
            SetStateStep::new()
                .put_account("address:owner", Account::new().nonce(1))
                .new_address("address:owner", 1, "sc:adder"),
        )
        .mandos_sc_deploy(
            ScDeployStep::new()
                .from("address:owner")
                .contract_code("file:output/adder.wasm", &intp_context)
                .argument("5")
                .gas_limit("5,000,000")
                .expect(TxExpect::ok().no_result()),
        )
        .mandos_sc_query(
            ScQueryStep::new()
                .to("sc:adder")
                .function("getSum")
                .expect(TxExpect::ok().result("5")),
        )
        .mandos_sc_call(
            ScCallStep::new()
                .from("address:owner")
                .to("sc:adder")
                .function("add")
                .argument("3")
                .expect(TxExpect::ok().no_result()),
        )
        .mandos_check_state(
            CheckStateStep::new()
                .put_account("address:owner", CheckAccount::new())
                .put_account(
                    "sc:adder",
                    CheckAccount::new().check_storage("str:sum", "8"),
                ),
        );
}
