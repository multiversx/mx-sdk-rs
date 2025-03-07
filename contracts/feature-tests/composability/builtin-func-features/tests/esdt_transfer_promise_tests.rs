use builtin_func_features::{
    esdt_features::{EsdtFeaturesModule, TransferResult},
    BuiltinFuncFeatures,
};
use multiversx_sc::{codec::Empty, types::Address};
use multiversx_sc_scenario::{
    imports::{BlockchainStateWrapper, ContractObjWrapper},
    managed_address, managed_biguint, managed_token_id, rust_biguint, DebugApi,
};

pub static FUNGIBLE_TOKEN_ID: &[u8] = b"FUNG-123456";
pub static NON_FUNGIBLE_TOKEN_ID: &[u8] = b"NONFUNG-123456";
pub const INIT_BALANCE: u64 = 100_000;
pub const INIT_NONCE: u64 = 1;

pub struct BuiltInFuncFeaturesSetup<BuiltInFuncBuilder>
where
    BuiltInFuncBuilder: 'static + Copy + Fn() -> builtin_func_features::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub user: Address,
    pub sc_wrapper:
        ContractObjWrapper<builtin_func_features::ContractObj<DebugApi>, BuiltInFuncBuilder>,
}

impl<BuiltInFuncBuilder> BuiltInFuncFeaturesSetup<BuiltInFuncBuilder>
where
    BuiltInFuncBuilder: 'static + Copy + Fn() -> builtin_func_features::ContractObj<DebugApi>,
{
    pub fn new(built_in_func_builder: BuiltInFuncBuilder) -> Self {
        let mut b_mock = BlockchainStateWrapper::new();
        let user = b_mock.create_user_account(&rust_biguint!(0));
        let sc_wrapper = b_mock.create_sc_account(
            &rust_biguint!(0),
            Some(&user),
            built_in_func_builder,
            "built in func features",
        );
        b_mock
            .execute_tx(&user, &sc_wrapper, &rust_biguint!(0), |sc| {
                sc.init(
                    managed_token_id!(FUNGIBLE_TOKEN_ID),
                    managed_token_id!(NON_FUNGIBLE_TOKEN_ID),
                );
            })
            .assert_ok();

        b_mock.set_esdt_balance(
            sc_wrapper.address_ref(),
            FUNGIBLE_TOKEN_ID,
            &rust_biguint!(INIT_BALANCE),
        );
        b_mock.set_nft_balance(
            sc_wrapper.address_ref(),
            NON_FUNGIBLE_TOKEN_ID,
            INIT_NONCE,
            &rust_biguint!(INIT_BALANCE),
            &Empty,
        );

        BuiltInFuncFeaturesSetup {
            b_mock,
            user,
            sc_wrapper,
        }
    }
}

#[test]
fn transfer_fungible_promise_no_callback_test() {
    let mut setup = BuiltInFuncFeaturesSetup::new(builtin_func_features::contract_obj);
    let user_addr = setup.user.clone();
    setup
        .b_mock
        .execute_tx(&setup.user, &setup.sc_wrapper, &rust_biguint!(0), |sc| {
            sc.transfer_fungible_promise_no_callback(
                managed_address!(&user_addr),
                managed_biguint!(INIT_BALANCE),
            );
        })
        .assert_ok();

    setup
        .b_mock
        .check_esdt_balance(&setup.user, FUNGIBLE_TOKEN_ID, &rust_biguint!(INIT_BALANCE));
    setup.b_mock.check_esdt_balance(
        setup.sc_wrapper.address_ref(),
        FUNGIBLE_TOKEN_ID,
        &rust_biguint!(0),
    );
}

#[test]
fn transfer_fungible_promise_with_callback_test() {
    let mut setup = BuiltInFuncFeaturesSetup::new(builtin_func_features::contract_obj);
    let user_addr = setup.user.clone();
    setup
        .b_mock
        .execute_tx(&setup.user, &setup.sc_wrapper, &rust_biguint!(0), |sc| {
            sc.transfer_fungible_promise_with_callback(
                managed_address!(&user_addr),
                managed_biguint!(INIT_BALANCE),
            );
        })
        .assert_ok();

    setup
        .b_mock
        .check_esdt_balance(&setup.user, FUNGIBLE_TOKEN_ID, &rust_biguint!(INIT_BALANCE));
    setup.b_mock.check_esdt_balance(
        setup.sc_wrapper.address_ref(),
        FUNGIBLE_TOKEN_ID,
        &rust_biguint!(0),
    );

    setup
        .b_mock
        .execute_query(&setup.sc_wrapper, |sc| {
            assert!(matches!(
                sc.latest_transfer_result().get(),
                TransferResult::Success
            ));
        })
        .assert_ok();
}
