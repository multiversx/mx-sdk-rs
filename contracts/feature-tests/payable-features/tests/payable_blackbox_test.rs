use multiversx_sc_scenario::imports::*;
use payable_features::payable_features_proxy;

const PF_PATH_EXPR: MxscPath = MxscPath::new("output/payable-features.mxsc.json");
const PAYABLE_FEATURES_ADDRESS: TestSCAddress = TestSCAddress::new("payable-features");
const USER: TestAddress = TestAddress::new("an-account");
const TOKEN_1: TestTokenIdentifier = TestTokenIdentifier::new("TOK-000001");
const TOKEN_2: TestTokenIdentifier = TestTokenIdentifier::new("TOK-000002");
const TOKEN_3: TestTokenIdentifier = TestTokenIdentifier::new("TOK-000003");
const SFT: TestTokenIdentifier = TestTokenIdentifier::new("SFT-123");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/payable-features");
    blockchain.register_contract(PF_PATH_EXPR, payable_features::ContractBuilder);
    blockchain
}

fn init_account(world: &mut ScenarioWorld) {
    world
        .account(USER)
        .balance(10000)
        .esdt_balance(TOKEN_1, 1000)
        .esdt_balance(TOKEN_2, 500)
        .esdt_balance(TOKEN_3, 500)
        .esdt_nft_balance(SFT, 5, 10, ());

    world
        .tx()
        .from(USER)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .init()
        .new_address(PAYABLE_FEATURES_ADDRESS)
        .code(PF_PATH_EXPR)
        .run();
}

#[test]
fn payable_all_blackbox_0() {
    let mut world = world();
    init_account(&mut world);

    let result = world
        .tx()
        .from(USER)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_all()
        .returns(ReturnsResultUnmanaged)
        .run();

    assert!(result.is_empty());
}

#[test]
fn payable_all_blackbox_1() {
    let mut world = world();
    init_account(&mut world);

    let result = world
        .tx()
        .from(USER)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_all()
        .payment(Payment::try_new(TOKEN_1, 0, 100u64).unwrap())
        .returns(ReturnsResultUnmanaged)
        .run();

    assert_eq!(result, vec![Payment::try_new(TOKEN_1, 0, 100u64).unwrap(),]);
}

#[test]
fn payable_all_blackbox_2() {
    let mut world = world();
    init_account(&mut world);

    let result = world
        .tx()
        .from(USER)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_all()
        .payment(Payment::try_new(TOKEN_1, 0, 100u64).unwrap())
        .payment(Payment::try_new(TOKEN_2, 0, 400u64).unwrap())
        .returns(ReturnsResultUnmanaged)
        .run();

    assert_eq!(
        result,
        vec![
            Payment::try_new(TOKEN_1, 0, 100u64).unwrap(),
            Payment::try_new(TOKEN_2, 0, 400u64).unwrap(),
        ]
    );
}

#[test]
fn payable_multi_legacy() {
    let mut world = world();
    init_account(&mut world);

    let result = world
        .tx()
        .from(USER)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_legacy_egld_esdt()
        .payment(Payment::try_new(TOKEN_1, 0, 100u64).unwrap())
        .payment(Payment::try_new(TOKEN_2, 0, 400u64).unwrap())
        .returns(ReturnsResultUnmanaged)
        .run();

    assert_eq!(result.as_tuple().0, RustBigUint::from(0u32));
    assert_eq!(
        result.as_tuple().1,
        vec![
            EsdtTokenPayment::new(TOKEN_1.to_token_identifier(), 0, BigUint::from(100u32)),
            EsdtTokenPayment::new(TOKEN_2.to_token_identifier(), 0, BigUint::from(400u32)),
        ]
    );
}
