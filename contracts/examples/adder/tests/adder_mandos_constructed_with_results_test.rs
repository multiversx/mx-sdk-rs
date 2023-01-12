use adder::*;
use multiversx_sc::storage::mappers::SingleValue;
use multiversx_sc_scenario::{num_bigint::BigUint, scenario_model::*, *}; // TODO: clean up imports

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract("file:output/adder.wasm", adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_mandos_constructed() {
    let _ = DebugApi::dummy();
    let mut world = world();
    let ic = world.interpreter_context();

    let owner_address = "address:owner";
    let mut adder_contract = ContractInfo::<adder::Proxy<DebugApi>>::new("sc:adder");

    world.set_state_step(
        SetStateStep::new()
            .put_account(owner_address, Account::new().nonce(1))
            .new_address(owner_address, 1, &adder_contract),
    );

    // deploy
    let (new_address, ()) = adder_contract
        .init(5u32)
        .into_blockchain_call()
        .from(owner_address)
        .contract_code("file:output/adder.wasm", &ic)
        .gas_limit("5,000,000")
        .expect(TxExpect::ok().no_result())
        .execute(&mut world);
    assert_eq!(new_address, adder_contract.to_address());

    // query, gets saved in the trace
    let result: SingleValue<BigUint> = adder_contract.sum().into_vm_query().execute(&mut world);
    assert_eq!(result.into(), BigUint::from(5u32));

    let () = adder_contract
        .add(3u32)
        .into_blockchain_call()
        .from(owner_address)
        .gas_limit(5000000)
        .expect(TxExpect::ok().no_result())
        .execute(&mut world);

    world.check_state_step(
        CheckStateStep::new()
            .put_account(owner_address, CheckAccount::new())
            .put_account(
                &adder_contract,
                CheckAccount::new().check_storage("str:sum", "8"),
            ),
    );

    world.write_scenario_trace("trace2.scen.json");
}
