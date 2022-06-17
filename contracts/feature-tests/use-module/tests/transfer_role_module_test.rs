use elrond_wasm::types::{
    EsdtLocalRole, EsdtTokenPayment, ManagedArgBuffer, ManagedVec, MultiValueEncoded,
};
use elrond_wasm_debug::{
    managed_address, managed_biguint, managed_buffer, managed_token_id, rust_biguint,
    testing_framework::BlockchainStateWrapper,
};
use elrond_wasm_modules::transfer_role::{
    transfer_destination::TransferDestinationModule, transfer_proxy::TransferProxyModule,
};

static TRANSFER_TOKEN_ID: &[u8] = b"TRANSFER-123456";
static WASM_PATH: &'static str = "wasm path";
static RECEIVE_FUNDS_FUNC_NAME: &[u8] = b"receiveFunds";

#[test]
fn test_transfer_role_module() {
    let rust_zero = rust_biguint!(0);
    let mut b_mock = BlockchainStateWrapper::new();
    let user = b_mock.create_user_account(&rust_zero);
    b_mock.set_esdt_balance(&user, TRANSFER_TOKEN_ID, &rust_biguint!(1_000));

    let owner = b_mock.create_user_account(&rust_zero);
    let sc_with_transfer_role = b_mock.create_sc_account(
        &rust_zero,
        Some(&owner),
        use_module::contract_obj,
        WASM_PATH,
    );
    let sc_dest = b_mock.create_sc_account(
        &rust_zero,
        Some(&owner),
        use_module::contract_obj,
        WASM_PATH,
    );

    // Note: The role restrictions are not currently implemented in the mock
    b_mock.set_esdt_local_roles(
        sc_with_transfer_role.address_ref(),
        TRANSFER_TOKEN_ID,
        &[EsdtLocalRole::Transfer],
    );

    // add sc to whitelist
    b_mock
        .execute_tx(&owner, &sc_dest, &rust_zero, |sc| {
            let mut args = MultiValueEncoded::new();
            args.push(managed_address!(sc_with_transfer_role.address_ref()));
            sc.add_contract_to_whitelist(args);
        })
        .assert_ok();

    // transfer to user
    b_mock
        .execute_esdt_transfer(
            &user,
            &sc_with_transfer_role,
            TRANSFER_TOKEN_ID,
            0,
            &rust_biguint!(100),
            |sc| {
                let payments = ManagedVec::from_single_item(EsdtTokenPayment::new(
                    managed_token_id!(TRANSFER_TOKEN_ID),
                    0,
                    managed_biguint!(100),
                ));
                sc.transfer_to_user(
                    managed_address!(&user),
                    managed_address!(&owner),
                    payments,
                    managed_buffer!(b"enjoy"),
                );
            },
        )
        .assert_ok();

    b_mock.check_esdt_balance(&user, TRANSFER_TOKEN_ID, &rust_biguint!(900));
    b_mock.check_esdt_balance(&owner, TRANSFER_TOKEN_ID, &rust_biguint!(100));

    // transfer to sc - ok
    b_mock
        .execute_esdt_transfer(
            &user,
            &sc_with_transfer_role,
            TRANSFER_TOKEN_ID,
            0,
            &rust_biguint!(100),
            |sc| {
                let payments = ManagedVec::from_single_item(EsdtTokenPayment::new(
                    managed_token_id!(TRANSFER_TOKEN_ID),
                    0,
                    managed_biguint!(100),
                ));
                sc.transfer_to_contract_raw(
                    managed_address!(&user),
                    managed_address!(sc_dest.address_ref()),
                    payments,
                    managed_buffer!(RECEIVE_FUNDS_FUNC_NAME),
                    ManagedArgBuffer::new(),
                );
            },
        )
        .assert_ok();

    b_mock.check_esdt_balance(&user, TRANSFER_TOKEN_ID, &rust_biguint!(800));
    b_mock.check_esdt_balance(
        sc_dest.address_ref(),
        TRANSFER_TOKEN_ID,
        &rust_biguint!(100),
    );

    // transfer to sc - err, wrong number of args
    b_mock
        .execute_esdt_transfer(
            &user,
            &sc_with_transfer_role,
            TRANSFER_TOKEN_ID,
            0,
            &rust_biguint!(100),
            |sc| {
                let payments = ManagedVec::from_single_item(EsdtTokenPayment::new(
                    managed_token_id!(TRANSFER_TOKEN_ID),
                    0,
                    managed_biguint!(100),
                ));

                let mut args = ManagedArgBuffer::new();
                args.push_arg(b"EVIL ARGUMENT");
                sc.transfer_to_contract_raw(
                    managed_address!(&user),
                    managed_address!(sc_dest.address_ref()),
                    payments,
                    managed_buffer!(RECEIVE_FUNDS_FUNC_NAME),
                    args,
                );
            },
        )
        .assert_ok();

    b_mock.check_esdt_balance(&user, TRANSFER_TOKEN_ID, &rust_biguint!(800));
    b_mock.check_esdt_balance(
        sc_dest.address_ref(),
        TRANSFER_TOKEN_ID,
        &rust_biguint!(100),
    );
}
