use crowdfunding_esdt::*;
use multiversx_sc::types::EgldOrEsdtTokenIdentifier;
use multiversx_sc_scenario::{api::StaticApi, scenario_model::*, *};

const CF_PATH_EXPR: &str = "file:output/crowdfunding-esdt.wasm";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crowdfunding-esdt");

    blockchain.register_contract(CF_PATH_EXPR, crowdfunding_esdt::ContractBuilder);
    blockchain
}

#[test]
fn crowdfunding_scenario_rust_test() {
    let mut world = world();
    let cf_code = world.code_expression(CF_PATH_EXPR);

    let owner_addr = "address:owner";
    let first_user_addr = "address:user1";
    let second_user_addr = "address:user2";

    let deadline: u64 = 7 * 24 * 60 * 60; // 1 week in seconds
    let cf_token_id_value = "CROWD-123456"; // when passing as argument
    let cf_token_id = "str:CROWD-123456"; // when specifying the token transfer
    let mut cf_sc = ContractInfo::<crowdfunding_esdt::Proxy<StaticApi>>::new("sc:crowdfunding");

    // setup owner and crowdfunding SC
    world.start_trace().set_state_step(
        SetStateStep::new()
            .put_account(owner_addr, Account::new())
            .new_address(owner_addr, 0, &cf_sc),
    );

    world.sc_deploy(
        ScDeployStep::new()
            .from(owner_addr)
            .code(cf_code)
            .call(cf_sc.init(
                2_000u32,
                deadline,
                EgldOrEsdtTokenIdentifier::esdt(cf_token_id_value),
            )),
    );

    // setup user accounts
    world
        .set_state_step(SetStateStep::new().put_account(
            first_user_addr,
            Account::new().esdt_balance(cf_token_id, 1_000u64),
        ))
        .set_state_step(SetStateStep::new().put_account(
            second_user_addr,
            Account::new().esdt_balance(cf_token_id, 1_000u64),
        ));

    // first user deposit
    world
        .sc_call(
            ScCallStep::new()
                .from(first_user_addr)
                .to(&cf_sc)
                .esdt_transfer(cf_token_id, 0u64, 1_000u64)
                .call(cf_sc.fund()),
        )
        .check_state_step(
            CheckStateStep::new()
                .put_account(
                    first_user_addr,
                    CheckAccount::new().esdt_balance(cf_token_id, 0u64),
                )
                .put_account(
                    &cf_sc,
                    CheckAccount::new().esdt_balance(cf_token_id, 1_000u64),
                ),
        );

    // second user deposit
    world
        .sc_call(
            ScCallStep::new()
                .from(second_user_addr)
                .to(&cf_sc)
                .esdt_transfer(cf_token_id, 0u64, 500u64)
                .call(cf_sc.fund()),
        )
        .check_state_step(
            CheckStateStep::new()
                .put_account(
                    second_user_addr,
                    CheckAccount::new().esdt_balance(cf_token_id, 500u64),
                )
                .put_account(
                    &cf_sc,
                    CheckAccount::new().esdt_balance(cf_token_id, 1_500u64),
                ),
        );

    // get status before
    let status: Status = cf_sc
        .status()
        .into_vm_query()
        .expect(TxExpect::ok().result(""))
        .execute(&mut world);
    assert_eq!(status, Status::FundingPeriod);

    // deadline passed
    world.set_state_step(SetStateStep::new().block_timestamp(deadline));

    // get status after deadline
    let status: Status = cf_sc
        .status()
        .into_vm_query()
        .expect(TxExpect::ok().result("2"))
        .execute(&mut world);
    assert_eq!(status, Status::Failed);

    // test failed campaign

    // owner claim - failed campaign - nothing is transferred
    world
        .sc_call(
            ScCallStep::new()
                .from(owner_addr)
                .to(&cf_sc)
                .call(cf_sc.claim()),
        )
        .check_state_step(
            CheckStateStep::new()
                .put_account(
                    owner_addr,
                    CheckAccount::new().esdt_balance(cf_token_id, 0u64),
                )
                .put_account(
                    &cf_sc,
                    CheckAccount::new().esdt_balance(cf_token_id, 1_500u64),
                ),
        );

    // first user claim - failed campaign
    world
        .sc_call(
            ScCallStep::new()
                .from(first_user_addr)
                .to(&cf_sc)
                .call(cf_sc.claim()),
        )
        .check_state_step(
            CheckStateStep::new()
                .put_account(
                    first_user_addr,
                    CheckAccount::new().esdt_balance(cf_token_id, 1_000u64),
                )
                .put_account(
                    &cf_sc,
                    CheckAccount::new().esdt_balance(cf_token_id, 500u64),
                ),
        );

    // second user claim - failed campaign
    world
        .sc_call(
            ScCallStep::new()
                .from(second_user_addr)
                .to(&cf_sc)
                .call(cf_sc.claim()),
        )
        .check_state_step(
            CheckStateStep::new()
                .put_account(
                    second_user_addr,
                    CheckAccount::new().esdt_balance(cf_token_id, 1_000u64),
                )
                .put_account(&cf_sc, CheckAccount::new().esdt_balance(cf_token_id, 0u64)),
        );

    // test successful campaign

    world.set_state_step(SetStateStep::new().block_timestamp(deadline / 2));

    // first user deposit
    world.sc_call(
        ScCallStep::new()
            .from(first_user_addr)
            .to(&cf_sc)
            .esdt_transfer(cf_token_id, 0u64, 1_000u64)
            .call(cf_sc.fund()),
    );

    // second user deposit
    world.sc_call(
        ScCallStep::new()
            .from(second_user_addr)
            .to(&cf_sc)
            .esdt_transfer(cf_token_id, 0u64, 1_000u64)
            .call(cf_sc.fund()),
    );

    let status: Status = cf_sc
        .status()
        .into_vm_query()
        .expect(TxExpect::ok().result(""))
        .execute(&mut world);
    assert_eq!(status, Status::FundingPeriod);

    world.set_state_step(SetStateStep::new().block_timestamp(deadline));

    let status: Status = cf_sc
        .status()
        .into_vm_query()
        .expect(TxExpect::ok().result("1"))
        .execute(&mut world);
    assert_eq!(status, Status::Successful);

    // first user try claim - successful campaign
    world.sc_call(
        ScCallStep::new()
            .from(first_user_addr)
            .to(&cf_sc)
            .call(cf_sc.claim())
            .expect(TxExpect::err(
                4,
                "str:only owner can claim successful funding",
            )),
    );

    // owner claim successful campaign
    world
        .sc_call(
            ScCallStep::new()
                .from(owner_addr)
                .to(&cf_sc)
                .call(cf_sc.claim()),
        )
        .check_state_step(
            CheckStateStep::new()
                .put_account(
                    owner_addr,
                    CheckAccount::new().esdt_balance(cf_token_id, 2_000u64),
                )
                .put_account(cf_sc, CheckAccount::new().esdt_balance(cf_token_id, 0u64)),
        );

    world.write_scenario_trace("scenarios-gen/crowdfunding_rust.scen.json");
}
