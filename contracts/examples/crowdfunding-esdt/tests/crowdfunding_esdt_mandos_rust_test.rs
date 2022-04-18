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
    let second_user_addr = AddressValue::interpret_from("address:user2", &ctx);

    let deadline: u64 = 7 * 24 * 60 * 60; // 1 week in seconds
    let cf_token_id = BytesKey::interpret_from("str:CROWD-123456", &ctx);
    let cf_address = AddressValue::interpret_from("sc:crowdfunding", &ctx);
    let mut cf_sc = ContractInfo::<crowdfunding_esdt::Proxy<DebugApi>>::new(&cf_address, &ctx);

    // setup owner and crowdfunding SC
    world.mandos_set_state(
        SetStateStep::new()
            .put_account(&owner_addr, Account::new())
            .new_address(&owner_addr, 0, &cf_sc),
    );
    let (_, ()) = world.mandos_sc_deploy_get_result(
        cf_sc.init(2_000u32, deadline, cf_token_id.value.clone()),
        ScDeployStep::new()
            .from(&owner_addr)
            .contract_code("file:output/crowdfunding-esdt.wasm", &ctx)
            .gas_limit("5,000,000")
            .expect(TxExpect::ok().no_result()),
    );

    // setup user accounts
    world
        .mandos_set_state(SetStateStep::new().put_account(
            &first_user_addr,
            Account::new().esdt_balance(&cf_token_id, 1_000u64),
        ))
        .mandos_set_state(SetStateStep::new().put_account(
            &second_user_addr,
            Account::new().esdt_balance(&cf_token_id, 1_000u64),
        ));

    // first user deposit
    world
        .mandos_sc_call(
            ScCallStep::new()
                .from(&first_user_addr)
                .to(&cf_sc)
                .esdt_transfer(&cf_token_id, 0u64, 1_000u64)
                .call(cf_sc.fund())
                .expect(TxExpect::ok().no_result()),
        )
        .mandos_check_state(
            CheckStateStep::new()
                .put_account(
                    &first_user_addr,
                    CheckAccount::new().esdt_balance(&cf_token_id, 0u64),
                )
                .put_account(
                    &cf_address,
                    CheckAccount::new().esdt_balance(&cf_token_id, 1_000u64),
                ),
        );

    // second user deposit
    world
        .mandos_sc_call(
            ScCallStep::new()
                .from(&second_user_addr)
                .to(&cf_sc)
                .esdt_transfer(&cf_token_id, 0u64, 500u64)
                .call(cf_sc.fund())
                .expect(TxExpect::ok().no_result()),
        )
        .mandos_check_state(
            CheckStateStep::new()
                .put_account(
                    &second_user_addr,
                    CheckAccount::new().esdt_balance(&cf_token_id, 500u64),
                )
                .put_account(
                    &cf_address,
                    CheckAccount::new().esdt_balance(&cf_token_id, 1_500u64),
                ),
        );

    // get status before & after

    // test failed campaign

    // test successful campaign

    world.write_mandos_trace("mandos/crowdfunding_rust.scen.json");
}
