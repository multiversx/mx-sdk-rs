use multiversx_sc::types::{
    Address, EsdtTokenPayment, ManagedArgBuffer, ManagedVec, MultiValueEncoded,
};
use multiversx_sc_modules::transfer_role_proxy::TransferRoleProxyModule;
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_buffer, managed_token_id, rust_biguint,
    scenario_model::{
        Account, AddressValue, CheckAccount, CheckStateStep, ScCallStep, ScDeployStep, SetStateStep,
    },
    ScenarioWorld, WhiteboxContract,
};
use transfer_role_features::TransferRoleFeatures;

const OWNER_ADDRESS_EXPR: &str = "address:owner";
const USER_ADDRESS_EXPR: &str = "address:user";

const TRANSFER_ROLE_FEATURES_ADDRESS_EXPR: &str = "sc:transfer-role-features";
const TRANSFER_ROLE_FEATURES_PATH_EXPR: &str = "file:output/transfer-role-features.wasm";

const TRANSFER_TOKEN_ID_EXPR: &str = "str:TRANSFER-123456";
const TRANSFER_TOKEN_ID: &[u8] = b"TRANSFER-123456";

const ACCEPT_FUNDS_FUNC_NAME: &[u8] = b"accept_funds";
const REJECT_FUNDS_FUNC_NAME: &[u8] = b"reject_funds";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace(
        "contracts/composability/feature-tests/transfer-role-features",
    );

    blockchain.register_contract(
        TRANSFER_ROLE_FEATURES_PATH_EXPR,
        transfer_role_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn test_transfer_role() {
    let mut world = world();

    world.set_state_step(
        SetStateStep::new()
            .put_account(OWNER_ADDRESS_EXPR, Account::new().nonce(1))
            .new_address(OWNER_ADDRESS_EXPR, 1, TRANSFER_ROLE_FEATURES_ADDRESS_EXPR)
            .put_account(
                USER_ADDRESS_EXPR,
                Account::new()
                    .nonce(1)
                    .esdt_balance(TRANSFER_TOKEN_ID_EXPR, 1_000u64),
            ),
    );

    // vault
    let vault_code = world.code_expression(VAULT_PATH_EXPR);

    const VAULT_ADDRESS_EXPR: &str = "sc:vault";
    const VAULT_PATH_EXPR: &str = "file:../vault/output/vault.wasm";

    world.register_contract(VAULT_PATH_EXPR, vault::ContractBuilder);
    world.set_state_step(SetStateStep::new().put_account(
        VAULT_ADDRESS_EXPR,
        Account::new().nonce(1).code(vault_code.clone()),
    ));

    let transfer_role_features_whitebox = WhiteboxContract::new(
        TRANSFER_ROLE_FEATURES_ADDRESS_EXPR,
        transfer_role_features::contract_obj,
    );
    let transfer_role_features_code = world.code_expression(TRANSFER_ROLE_FEATURES_PATH_EXPR);

    // init
    world.whitebox_deploy(
        &transfer_role_features_whitebox,
        ScDeployStep::new()
            .from(OWNER_ADDRESS_EXPR)
            .code(transfer_role_features_code),
        |sc| {
            let mut whitelist = MultiValueEncoded::new();
            whitelist.push(managed_address!(&address_expr_to_address(
                OWNER_ADDRESS_EXPR
            )));
            whitelist.push(managed_address!(&address_expr_to_address(
                VAULT_ADDRESS_EXPR
            )));

            sc.init(whitelist);
        },
    );

    // transfer to user - ok
    world.whitebox_call(
        &transfer_role_features_whitebox,
        ScCallStep::new().from(USER_ADDRESS_EXPR).esdt_transfer(
            TRANSFER_TOKEN_ID,
            0,
            rust_biguint!(100),
        ),
        |sc| {
            let payments = ManagedVec::from_single_item(EsdtTokenPayment::new(
                managed_token_id!(TRANSFER_TOKEN_ID),
                0,
                managed_biguint!(100),
            ));
            sc.transfer_to_user(
                managed_address!(&address_expr_to_address(USER_ADDRESS_EXPR)),
                managed_address!(&address_expr_to_address(OWNER_ADDRESS_EXPR)),
                payments,
                managed_buffer!(b"enjoy"),
            );
        },
    );

    world.check_state_step(CheckStateStep::new().put_account(
        USER_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(TRANSFER_TOKEN_ID_EXPR, &rust_biguint!(900)),
    ));
    world.check_state_step(CheckStateStep::new().put_account(
        OWNER_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(TRANSFER_TOKEN_ID_EXPR, &rust_biguint!(100)),
    ));

    // transfer to user - err, not whitelisted
    world.whitebox_call_check(
        &transfer_role_features_whitebox,
        ScCallStep::new()
            .from(USER_ADDRESS_EXPR)
            .esdt_transfer(TRANSFER_TOKEN_ID, 0, rust_biguint!(100))
            .no_expect(),
        |sc| {
            let payments = ManagedVec::from_single_item(EsdtTokenPayment::new(
                managed_token_id!(TRANSFER_TOKEN_ID),
                0,
                managed_biguint!(100),
            ));
            sc.transfer_to_user(
                managed_address!(&address_expr_to_address(USER_ADDRESS_EXPR)),
                managed_address!(&Address::zero()),
                payments,
                managed_buffer!(b"enjoy"),
            );
        },
        |r| {
            r.assert_user_error("Destination address not whitelisted");
        },
    );

    // transfer to sc - ok
    world.whitebox_call(
        &transfer_role_features_whitebox,
        ScCallStep::new().from(USER_ADDRESS_EXPR).esdt_transfer(
            TRANSFER_TOKEN_ID,
            0,
            rust_biguint!(100),
        ),
        |sc| {
            let payments = ManagedVec::from_single_item(EsdtTokenPayment::new(
                managed_token_id!(TRANSFER_TOKEN_ID),
                0,
                managed_biguint!(100),
            ));
            sc.transfer_to_contract_raw(
                managed_address!(&address_expr_to_address(USER_ADDRESS_EXPR)),
                managed_address!(&address_expr_to_address(VAULT_ADDRESS_EXPR)),
                payments,
                managed_buffer!(ACCEPT_FUNDS_FUNC_NAME),
                ManagedArgBuffer::new(),
                None,
            );
        },
    );

    world.check_state_step(CheckStateStep::new().put_account(
        USER_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(TRANSFER_TOKEN_ID_EXPR, &rust_biguint!(800)),
    ));
    world.check_state_step(CheckStateStep::new().put_account(
        VAULT_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(TRANSFER_TOKEN_ID_EXPR, &rust_biguint!(100)),
    ));

    // transfer to sc - reject
    world.whitebox_call(
        &transfer_role_features_whitebox,
        ScCallStep::new().from(USER_ADDRESS_EXPR).esdt_transfer(
            TRANSFER_TOKEN_ID,
            0,
            rust_biguint!(100),
        ),
        |sc| {
            let payments = ManagedVec::from_single_item(EsdtTokenPayment::new(
                managed_token_id!(TRANSFER_TOKEN_ID),
                0,
                managed_biguint!(100),
            ));
            sc.transfer_to_contract_raw(
                managed_address!(&address_expr_to_address(USER_ADDRESS_EXPR)),
                managed_address!(&address_expr_to_address(VAULT_ADDRESS_EXPR)),
                payments,
                managed_buffer!(REJECT_FUNDS_FUNC_NAME),
                ManagedArgBuffer::new(),
                None,
            );
        },
    );

    world.check_state_step(CheckStateStep::new().put_account(
        USER_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(TRANSFER_TOKEN_ID_EXPR, &rust_biguint!(800)),
    ));
    world.check_state_step(CheckStateStep::new().put_account(
        VAULT_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(TRANSFER_TOKEN_ID_EXPR, &rust_biguint!(100)),
    ));
}

fn address_expr_to_address(address_expr: &str) -> Address {
    AddressValue::from(address_expr).to_address()
}
