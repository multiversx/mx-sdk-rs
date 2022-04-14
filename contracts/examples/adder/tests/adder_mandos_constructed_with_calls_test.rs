use adder::*;
use elrond_wasm::storage::mappers::SingleValue;
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
fn adder_mandos_constructed_raw() {
    let _ = DebugApi::dummy();
    let mut world = world();
    let ic = world.interpreter_context();
    let owner_address = AddressValue::interpret_from("address:owner", &ic);
    let mut adder_contract = ContractInfo::<adder::Proxy<DebugApi>>::new("sc:adder", &ic);

    world
        .mandos_set_state(
            SetStateStep::new()
                .put_account("address:owner", Account::new().nonce(1))
                .new_address("address:owner", 1, "sc:adder"),
        )
        .mandos_sc_deploy(
            ScDeployStep::new()
                .from(&owner_address)
                .contract_code("file:output/adder.wasm", &ic)
                .call(adder_contract.init(5u32))
                .gas_limit("5,000,000")
                .expect(TxExpect::ok().no_result()),
        )
        .mandos_sc_query(
            ScQueryStep::new()
                .to(&adder_contract)
                .call_expect(adder_contract.sum(), SingleValue::from(BigUint::from(5u32))),
        )
        .mandos_sc_call(
            ScCallStep::new()
                .from(&owner_address)
                .to(&adder_contract)
                .call(adder_contract.add(3u32))
                .expect(TxExpect::ok().no_result()),
        )
        .mandos_check_state(
            CheckStateStep::new()
                .put_account(&owner_address, CheckAccount::new())
                .put_account(
                    &adder_contract,
                    CheckAccount::new().check_storage("str:sum", "8"),
                ),
        )
        .write_mandos_trace("trace1.scen.json");
}
