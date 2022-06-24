use adder::*;
use elrond_wasm::storage::mappers::SingleValue;
use elrond_wasm_debug::{mandos_system::model::*, num_bigint::BigUint, *}; // TODO: clean up imports

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract_builder("file:output/adder.wasm", adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_mandos_constructed() {
    let _ = DebugApi::dummy();
    let mut world = world();
    let ic = world.interpreter_context();

    let owner_address = "address:owner";
    let mut adder_contract = ContractInfo::<adder::Proxy<DebugApi>>::new("sc:adder");

    world.mandos_set_state(
        SetStateStep::new()
            .put_account(owner_address, Account::new().nonce(1))
            .new_address(owner_address, 1, &adder_contract),
    );

    // deploy
    let (new_address, ()) = world.mandos_sc_deploy_get_result(
        adder_contract.init(5u32),
        ScDeployStep::new()
            .from(owner_address)
            .contract_code("file:output/adder.wasm", &ic)
            .gas_limit("5,000,000")
            .expect(TxExpect::ok().no_result()),
    );
    assert_eq!(new_address, adder_contract.to_address());

    // mandos query, gets saved in the trace
    let result: SingleValue<BigUint> =
        world.mandos_sc_query_expect_result(adder_contract.sum(), ScQueryStep::new());
    assert_eq!(result.into(), BigUint::from(5u32));

    let () = world.mandos_sc_call_get_result(
        adder_contract.add(3u32),
        ScCallStep::new()
            .from(owner_address)
            .gas_limit(5000000)
            .expect(TxExpect::ok().no_result()),
    );

    world.mandos_check_state(
        CheckStateStep::new()
            .put_account(owner_address, CheckAccount::new())
            .put_account(
                &adder_contract,
                CheckAccount::new().check_storage("str:sum", "8"),
            ),
    );

    world.write_mandos_trace("trace2.scen.json");
}
