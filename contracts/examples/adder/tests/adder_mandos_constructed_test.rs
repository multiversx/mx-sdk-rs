use adder::*;
use elrond_wasm::storage::mappers::SingleValue;
use elrond_wasm_debug::{
    mandos::{interpret_trait::InterpretableFrom, model::*},
    num_bigint::BigUint,
    *,
}; // TODO: clean up imports

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

    let owner_address = AddressValue::interpret_from("address:owner", &ic);
    let mut adder_contract = ContractInfo::<adder::Proxy<DebugApi>>::new("sc:adder", &ic);

    world.mandos_set_state(
        SetStateStep::new()
            .put_account(&owner_address, Account::new().nonce(1))
            .new_address(&owner_address, 1, &adder_contract),
    );

    // deploy
    let (new_address, result) = world.sc_deploy(
        &owner_address,
        BytesValue::interpret_from("file:output/adder.wasm", &ic),
        adder_contract.init(5u32),
    );
    assert_eq!(
        new_address.as_bytes(),
        adder_contract.mandos_address_expr.value
    );
    assert!(result.is_empty());

    // query
    let result: SingleValue<BigUint> = world.sc_query(adder_contract.sum());
    assert_eq!(result.into(), BigUint::from(5u32));

    // call
    let () = world.sc_call(&owner_address, adder_contract.add(3u32));

    world.mandos_check_state(
        CheckStateStep::new()
            .put_account(&owner_address, CheckAccount::new())
            .put_account(
                &adder_contract,
                CheckAccount::new().check_storage("str:sum", "8"),
            ),
    );

    world.write_mandos_trace("trace.scen.json");
}
