#![allow(deprecated)] // TODO: migrate tests

use crowdfunding_esdt::*;
use multiversx_sc::types::{Address, EgldOrEsdtTokenIdentifier};
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_token_id, rust_biguint, testing_framework::*,
    DebugApi,
};

const CF_TOKEN_ID: &[u8] = b"CROWD-123456";
const CF_DEADLINE: u64 = 7 * 24 * 60 * 60; // 1 week in seconds
const WASM_PATH: &str = "output/crowdfunding-esdt.wasm";

struct CrowdfundingSetup<CrowdfundingObjBuilder>
where
    CrowdfundingObjBuilder: 'static + Copy + Fn() -> crowdfunding_esdt::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub first_user_address: Address,
    pub second_user_address: Address,
    pub cf_wrapper:
        ContractObjWrapper<crowdfunding_esdt::ContractObj<DebugApi>, CrowdfundingObjBuilder>,
}

fn setup_crowdfunding<CrowdfundingObjBuilder>(
    cf_builder: CrowdfundingObjBuilder,
) -> CrowdfundingSetup<CrowdfundingObjBuilder>
where
    CrowdfundingObjBuilder: 'static + Copy + Fn() -> crowdfunding_esdt::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let first_user_address = blockchain_wrapper.create_user_account(&rust_zero);
    let second_user_address = blockchain_wrapper.create_user_account(&rust_zero);
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );

    blockchain_wrapper.set_esdt_balance(&first_user_address, CF_TOKEN_ID, &rust_biguint!(1_000));
    blockchain_wrapper.set_esdt_balance(&second_user_address, CF_TOKEN_ID, &rust_biguint!(1_000));

    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            let target = managed_biguint!(2_000);
            let token_id = managed_token_id!(CF_TOKEN_ID);

            sc.init(
                target,
                CF_DEADLINE,
                EgldOrEsdtTokenIdentifier::esdt(token_id),
            );
        })
        .assert_ok();

    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    CrowdfundingSetup {
        blockchain_wrapper,
        owner_address,
        first_user_address,
        second_user_address,
        cf_wrapper,
    }
}

#[test]
fn init_test() {
    let cf_setup = setup_crowdfunding(crowdfunding_esdt::contract_obj);
    cf_setup
        .blockchain_wrapper
        .write_mandos_output("_generated_init.scen.json");
}

#[test]
fn fund_test() {
    let mut cf_setup = setup_crowdfunding(crowdfunding_esdt::contract_obj);
    let b_wrapper = &mut cf_setup.blockchain_wrapper;
    let user_addr = &cf_setup.first_user_address;

    b_wrapper
        .execute_esdt_transfer(
            user_addr,
            &cf_setup.cf_wrapper,
            CF_TOKEN_ID,
            0,
            &rust_biguint!(1_000),
            |sc| {
                sc.fund();

                let user_deposit = sc.deposit(&managed_address!(user_addr)).get();
                let expected_deposit = managed_biguint!(1_000);
                assert_eq!(user_deposit, expected_deposit);
            },
        )
        .assert_ok();

    let mut sc_call = ScCallMandos::new(user_addr, cf_setup.cf_wrapper.address_ref(), "fund");
    sc_call.add_esdt_transfer(CF_TOKEN_ID, 0, &rust_biguint!(1_000));

    let expect = TxExpectMandos::new(0);
    b_wrapper.add_mandos_sc_call(sc_call, Some(expect));

    cf_setup
        .blockchain_wrapper
        .write_mandos_output("_generated_fund.scen.json");
}

#[test]
fn status_test() {
    let mut cf_setup = setup_crowdfunding(crowdfunding_esdt::contract_obj);
    let b_wrapper = &mut cf_setup.blockchain_wrapper;

    b_wrapper
        .execute_query(&cf_setup.cf_wrapper, |sc| {
            let status = sc.status();
            assert_eq!(status, Status::FundingPeriod);
        })
        .assert_ok();

    let sc_query = ScQueryMandos::new(cf_setup.cf_wrapper.address_ref(), "status");
    let mut expect = TxExpectMandos::new(0);
    expect.add_out_value(&Status::FundingPeriod);

    b_wrapper.add_mandos_sc_query(sc_query, Some(expect));

    cf_setup
        .blockchain_wrapper
        .write_mandos_output("_generated_query_status.scen.json");
}

#[test]
fn test_sc_error() {
    let mut cf_setup = setup_crowdfunding(crowdfunding_esdt::contract_obj);
    let b_wrapper = &mut cf_setup.blockchain_wrapper;
    let user_addr = &cf_setup.first_user_address;

    b_wrapper.set_egld_balance(user_addr, &rust_biguint!(1_000));

    b_wrapper
        .execute_tx(
            user_addr,
            &cf_setup.cf_wrapper,
            &rust_biguint!(1_000),
            |sc| {
                sc.fund();
            },
        )
        .assert_user_error("wrong token");

    b_wrapper
        .execute_tx(user_addr, &cf_setup.cf_wrapper, &rust_biguint!(0), |sc| {
            let user_deposit = sc.deposit(&managed_address!(user_addr)).get();
            let expected_deposit = managed_biguint!(0);
            assert_eq!(user_deposit, expected_deposit);
        })
        .assert_ok();

    let mut sc_call = ScCallMandos::new(user_addr, cf_setup.cf_wrapper.address_ref(), "fund");
    sc_call.add_egld_value(&rust_biguint!(1_000));

    let mut expect = TxExpectMandos::new(4);
    expect.set_message("wrong token");

    b_wrapper.add_mandos_sc_call(sc_call, Some(expect));

    cf_setup
        .blockchain_wrapper
        .write_mandos_output("_generated_sc_err.scen.json");
}

#[test]
fn test_successful_cf() {
    let mut cf_setup = setup_crowdfunding(crowdfunding_esdt::contract_obj);
    let b_wrapper = &mut cf_setup.blockchain_wrapper;
    let owner = &cf_setup.owner_address;
    let first_user = &cf_setup.first_user_address;
    let second_user = &cf_setup.second_user_address;

    // first user fund
    b_wrapper
        .execute_esdt_transfer(
            first_user,
            &cf_setup.cf_wrapper,
            CF_TOKEN_ID,
            0,
            &rust_biguint!(1_000),
            |sc| {
                sc.fund();

                let user_deposit = sc.deposit(&managed_address!(first_user)).get();
                let expected_deposit = managed_biguint!(1_000);
                assert_eq!(user_deposit, expected_deposit);
            },
        )
        .assert_ok();

    // second user fund
    b_wrapper
        .execute_esdt_transfer(
            second_user,
            &cf_setup.cf_wrapper,
            CF_TOKEN_ID,
            0,
            &rust_biguint!(1_000),
            |sc| {
                sc.fund();

                let user_deposit = sc.deposit(&managed_address!(second_user)).get();
                let expected_deposit = managed_biguint!(1_000);
                assert_eq!(user_deposit, expected_deposit);
            },
        )
        .assert_ok();

    // set block timestamp after deadline
    b_wrapper.set_block_timestamp(CF_DEADLINE + 1);

    // check status
    b_wrapper
        .execute_query(&cf_setup.cf_wrapper, |sc| {
            let status = sc.status();
            assert_eq!(status, Status::Successful);
        })
        .assert_ok();

    // user try claim
    b_wrapper
        .execute_tx(first_user, &cf_setup.cf_wrapper, &rust_biguint!(0), |sc| {
            sc.claim();
        })
        .assert_user_error("only owner can claim successful funding");

    // owner claim
    b_wrapper
        .execute_tx(owner, &cf_setup.cf_wrapper, &rust_biguint!(0), |sc| {
            sc.claim();
        })
        .assert_ok();

    b_wrapper.check_esdt_balance(owner, CF_TOKEN_ID, &rust_biguint!(2_000));
    b_wrapper.check_esdt_balance(first_user, CF_TOKEN_ID, &rust_biguint!(0));
    b_wrapper.check_esdt_balance(second_user, CF_TOKEN_ID, &rust_biguint!(0));
}

#[test]
fn test_failed_cf() {
    let mut cf_setup = setup_crowdfunding(crowdfunding_esdt::contract_obj);
    let b_wrapper = &mut cf_setup.blockchain_wrapper;
    let owner = &cf_setup.owner_address;
    let first_user = &cf_setup.first_user_address;
    let second_user = &cf_setup.second_user_address;

    // first user fund
    b_wrapper
        .execute_esdt_transfer(
            first_user,
            &cf_setup.cf_wrapper,
            CF_TOKEN_ID,
            0,
            &rust_biguint!(300),
            |sc| {
                sc.fund();

                let user_deposit = sc.deposit(&managed_address!(first_user)).get();
                let expected_deposit = managed_biguint!(300);
                assert_eq!(user_deposit, expected_deposit);
            },
        )
        .assert_ok();

    // second user fund
    b_wrapper
        .execute_esdt_transfer(
            second_user,
            &cf_setup.cf_wrapper,
            CF_TOKEN_ID,
            0,
            &rust_biguint!(600),
            |sc| {
                sc.fund();

                let user_deposit = sc.deposit(&managed_address!(second_user)).get();
                let expected_deposit = managed_biguint!(600);
                assert_eq!(user_deposit, expected_deposit);
            },
        )
        .assert_ok();

    // set block timestamp after deadline
    b_wrapper.set_block_timestamp(CF_DEADLINE + 1);

    // check status
    b_wrapper
        .execute_query(&cf_setup.cf_wrapper, |sc| {
            let status = sc.status();
            assert_eq!(status, Status::Failed);
        })
        .assert_ok();

    // first user claim
    b_wrapper
        .execute_tx(first_user, &cf_setup.cf_wrapper, &rust_biguint!(0), |sc| {
            sc.claim();
        })
        .assert_ok();

    // second user claim
    b_wrapper
        .execute_tx(second_user, &cf_setup.cf_wrapper, &rust_biguint!(0), |sc| {
            sc.claim();
        })
        .assert_ok();

    b_wrapper.check_esdt_balance(owner, CF_TOKEN_ID, &rust_biguint!(0));
    b_wrapper.check_esdt_balance(first_user, CF_TOKEN_ID, &rust_biguint!(1_000));
    b_wrapper.check_esdt_balance(second_user, CF_TOKEN_ID, &rust_biguint!(1_000));
}
