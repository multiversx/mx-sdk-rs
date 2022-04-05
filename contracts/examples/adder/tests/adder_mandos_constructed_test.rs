use adder::*;
use elrond_wasm::{contract_base::ProxyObjBase, storage::mappers::SingleValue};
use elrond_wasm_debug::{
    mandos::{interpret_trait::InterpretableFrom, model::*},
    num_bigint::BigUint,
    *,
};

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
    let intp_context = world.interpreter_context();

    let owner_address = AddressValue::interpret_from("address:owner", &intp_context);
    let adder_address = AddressValue::interpret_from("sc:adder", &intp_context);

    world
        .mandos_set_state(
            SetStateStep::new()
                .put_account(&owner_address, Account::new().nonce(1))
                .new_address(&owner_address, 1, &adder_address),
        )
        .mandos_sc_deploy(
            ScDeployStep::new()
                .from(&owner_address)
                .contract_code("file:output/adder.wasm", &intp_context)
                .argument("5")
                .gas_limit("5,000,000")
                .expect(TxExpect::ok().no_result()),
        );

    let result: SingleValue<BigUint> = world.sc_query(
        adder::Proxy::new_proxy_obj()
            .contract(adder_address.value.into())
            .sum(),
    );
    assert_eq!(result.into(), BigUint::from(5u32));

    let () = world.sc_call(
        owner_address.value.into(),
        adder::Proxy::new_proxy_obj()
            .contract(adder_address.value.into())
            .add(3u32),
    );

    world.mandos_check_state(
        CheckStateStep::new()
            .put_account(&owner_address, CheckAccount::new())
            .put_account(
                &adder_address,
                CheckAccount::new().check_storage("str:sum", "8"),
            ),
    );
}
