use crowdfunding_esdt::*;
use elrond_wasm::storage::mappers::SingleValue;
use elrond_wasm_debug::{
    mandos::{interpret_trait::InterpretableFrom, model::*},
    num_bigint::BigUint,
    *,
};

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crowdfunding-esdt");

    blockchain.register_contract_builder(
        "file:output/crowdfunding-esdt.wasm",
        crowdfunding_esdt::ContractBuilder,
    );
    blockchain
}

#[test]
fn crowdfunding_mandos_rust_test() {
    let _ = DebugApi::dummy();
    let mut world = world();
    let ic = world.interpreter_context();

    let owner_addr = AddressValue::interpret_from("address:owner", &ic);
    let mut cf_sc = ContractInfo::<crowdfunding_esdt::Proxy<DebugApi>>::new("sc:crowdfunding", &ic);

    world.mandos_set_state(
        SetStateStep::new()
            .put_account(&owner_addr, Account::new())
            .new_address(&owner_addr, 0, &cf_sc),
    );

    
}
