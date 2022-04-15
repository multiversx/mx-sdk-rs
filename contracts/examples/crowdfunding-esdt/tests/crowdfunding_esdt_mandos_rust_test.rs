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
    let ctx = world.interpreter_context();

    let owner_addr = AddressValue::interpret_from("address:owner", &ctx);
    let first_user_addr = AddressValue::interpret_from("address:user1", &ctx);
    let first_user_balance = BigUintValue::interpret_from(1_000u64, &ctx);
    let second_user_addr = AddressValue::interpret_from("address:user2", &ctx);
    let second_user_balance = BigUintValue::interpret_from(1_000u64, &ctx);

    let cf_token_id = BytesKey::interpret_from("str:CROWD-123456", &ctx);
    let cf_address = AddressValue::interpret_from("sc:crowdfunding", &ctx);
    let mut cf_sc = ContractInfo::<crowdfunding_esdt::Proxy<DebugApi>>::new(&cf_address, &ctx);

    world
        .mandos_set_state(
            SetStateStep::new()
                .put_account(&owner_addr, Account::new())
                .new_address(&owner_addr, 0, &cf_sc),
        )
        .mandos_set_state(SetStateStep::new().put_account(
            &first_user_addr,
            Account::new().esdt_balance(&cf_token_id, first_user_balance),
        ))
        .mandos_set_state(SetStateStep::new().put_account(
            &second_user_addr,
            Account::new().esdt_balance(&cf_token_id, second_user_balance),
        ));
}
