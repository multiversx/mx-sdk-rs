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

    world
        .mandos_set_state(
            SetStateStep::new()
                .put_account(&owner_address, Account::new().nonce(1))
                .new_address(&owner_address, 1, &adder_contract),
        )
        .mandos_sc_deploy(
            ScDeployStep::new()
                .from(&owner_address)
                .contract_code("file:output/adder.wasm", &ic)
                .argument("5")
                .gas_limit("5,000,000")
                .expect(TxExpect::ok().no_result()),
        );

    let result: SingleValue<BigUint> = world.sc_query(adder_contract.sum());
    assert_eq!(result.into(), BigUint::from(5u32));

    let () = world.sc_call(&owner_address, adder_contract.add(3u32));

    world.mandos_check_state(
        CheckStateStep::new()
            .put_account(&owner_address, CheckAccount::new())
            .put_account(
                &adder_contract,
                CheckAccount::new().check_storage("str:sum", "8"),
            ),
    );
}
