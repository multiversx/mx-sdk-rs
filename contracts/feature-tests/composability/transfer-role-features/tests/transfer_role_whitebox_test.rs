use multiversx_sc_modules::transfer_role_proxy::TransferRoleProxyModule;
use multiversx_sc_scenario::imports::*;
use transfer_role_features::TransferRoleFeatures;
use vault::Vault;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const USER_ADDRESS: TestAddress = TestAddress::new("user");

const TRANSFER_ROLE_FEATURES_ADDRESS: TestSCAddress = TestSCAddress::new("transfer-role-features");
const TRANSFER_ROLE_FEATURES_PATH_EXPR: MxscPath =
    MxscPath::new("mxsc:output/transfer-role-features.mxsc.json");

const TRANSFER_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("TRANSFER-123456");
const TRANSFER_TOKEN_ID_EXPR: &[u8] = b"TRANSFER-123456";

const ACCEPT_FUNDS_FUNC_NAME: &[u8] = b"accept_funds";
const REJECT_FUNDS_FUNC_NAME: &[u8] = b"reject_funds";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace(
        "contracts/feature-tests/composability/transfer-role-features",
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

    world.account(OWNER_ADDRESS).nonce(1);
    world
        .account(USER_ADDRESS)
        .nonce(1)
        .esdt_balance(TRANSFER_TOKEN_ID, BigUint::from(1_000u64));

    // vault
    const VAULT_ADDRESS: TestSCAddress = TestSCAddress::new("vault");
    const VAULT_PATH_EXPR: MxscPath = MxscPath::new("mxsc:../vault/output/vault.mxsc.json");

    world.register_contract(VAULT_PATH_EXPR, vault::ContractBuilder);
    world
        .tx()
        .from(OWNER_ADDRESS)
        .raw_deploy()
        .new_address(VAULT_ADDRESS)
        .code(VAULT_PATH_EXPR)
        .whitebox(vault::contract_obj, |sc| {
            let _ = sc.init(OptionalValue::None);
        });

    // init
    world
        .tx()
        .from(OWNER_ADDRESS)
        .raw_deploy()
        .new_address(TRANSFER_ROLE_FEATURES_ADDRESS)
        .code(TRANSFER_ROLE_FEATURES_PATH_EXPR)
        .whitebox(transfer_role_features::contract_obj, |sc| {
            let mut whitelist = MultiValueEncoded::new();
            whitelist.push(OWNER_ADDRESS.to_managed_address());
            whitelist.push(VAULT_ADDRESS.to_managed_address());

            sc.init(whitelist);
        });

    // transfer to user - ok
    world
        .tx()
        .from(USER_ADDRESS)
        .to(TRANSFER_ROLE_FEATURES_ADDRESS)
        .payment(Payment::try_new(TRANSFER_TOKEN_ID, 0, 100u64).unwrap())
        .whitebox(transfer_role_features::contract_obj, |sc| {
            let payments = ManagedVec::from_single_item(EsdtTokenPayment::new(
                managed_token_id!(TRANSFER_TOKEN_ID_EXPR),
                0,
                managed_biguint!(100),
            ));
            sc.transfer_to_user(
                USER_ADDRESS.to_managed_address(),
                OWNER_ADDRESS.to_managed_address(),
                &payments,
                managed_buffer!(b"enjoy"),
            );
        });

    world
        .check_account(USER_ADDRESS)
        .esdt_balance(TRANSFER_TOKEN_ID, BigUint::from(900u64));
    world
        .check_account(OWNER_ADDRESS)
        .esdt_balance(TRANSFER_TOKEN_ID, BigUint::from(100u64));

    // transfer to user - err, not whitelisted
    world
        .tx()
        .from(USER_ADDRESS)
        .to(TRANSFER_ROLE_FEATURES_ADDRESS)
        .payment(Payment::try_new(TRANSFER_TOKEN_ID, 0, 100u64).unwrap())
        .returns(ExpectError(4u64, "Destination address not whitelisted"))
        .whitebox(transfer_role_features::contract_obj, |sc| {
            let payments = ManagedVec::from_single_item(EsdtTokenPayment::new(
                managed_token_id!(TRANSFER_TOKEN_ID_EXPR),
                0,
                managed_biguint!(100),
            ));
            sc.transfer_to_user(
                USER_ADDRESS.to_managed_address(),
                managed_address!(&Address::zero()),
                &payments,
                managed_buffer!(b"enjoy"),
            );
        });

    // transfer to sc - ok
    world
        .tx()
        .from(USER_ADDRESS)
        .to(TRANSFER_ROLE_FEATURES_ADDRESS)
        .payment(Payment::try_new(TRANSFER_TOKEN_ID, 0, 100u64).unwrap())
        .whitebox(transfer_role_features::contract_obj, |sc| {
            let payments = ManagedVec::from_single_item(EsdtTokenPayment::new(
                managed_token_id!(TRANSFER_TOKEN_ID_EXPR),
                0,
                managed_biguint!(100),
            ));
            sc.transfer_to_contract_raw(
                USER_ADDRESS.to_managed_address(),
                VAULT_ADDRESS.to_managed_address(),
                &payments,
                managed_buffer!(ACCEPT_FUNDS_FUNC_NAME),
                ManagedArgBuffer::new(),
                None,
            );
        });

    world
        .check_account(USER_ADDRESS)
        .esdt_balance(TRANSFER_TOKEN_ID, BigUint::from(800u64));
    world
        .check_account(VAULT_ADDRESS)
        .esdt_balance(TRANSFER_TOKEN_ID, BigUint::from(100u64));

    // transfer to sc - reject
    world
        .tx()
        .from(USER_ADDRESS)
        .to(TRANSFER_ROLE_FEATURES_ADDRESS)
        .payment(Payment::try_new(TRANSFER_TOKEN_ID, 0, 100u64).unwrap())
        .whitebox(transfer_role_features::contract_obj, |sc| {
            let payments = ManagedVec::from_single_item(EsdtTokenPayment::new(
                managed_token_id!(TRANSFER_TOKEN_ID_EXPR),
                0,
                managed_biguint!(100),
            ));
            sc.transfer_to_contract_raw(
                USER_ADDRESS.to_managed_address(),
                VAULT_ADDRESS.to_managed_address(),
                &payments,
                managed_buffer!(REJECT_FUNDS_FUNC_NAME),
                ManagedArgBuffer::new(),
                None,
            );
        });

    world
        .check_account(USER_ADDRESS)
        .esdt_balance(TRANSFER_TOKEN_ID, BigUint::from(800u64));
    world
        .check_account(VAULT_ADDRESS)
        .esdt_balance(TRANSFER_TOKEN_ID, BigUint::from(100u64));
}
