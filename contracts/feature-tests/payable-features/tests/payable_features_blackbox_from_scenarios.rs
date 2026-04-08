#[rustfmt::skip]
mod generated {
// Auto-generated blackbox tests from scenarios

use multiversx_sc_scenario::imports::*;

use payable_features::*;

const PAYABLE_FEATURES_CODE_PATH: MxscPath = MxscPath::new("output/payable-features.mxsc.json");
const AN_ACCOUNT_ADDRESS: TestAddress = TestAddress::new("an-account");
const A_USER_ADDRESS: TestAddress = TestAddress::new("a_user");
const PAYABLE_ADDRESS: TestSCAddress = TestSCAddress::new("payable");
const PAYABLE_FEATURES_ADDRESS: TestSCAddress = TestSCAddress::new("payable-features");
const EGLD: TestTokenId = TestTokenId::new("EGLD");
const OTHERTOK_123456: TestTokenId = TestTokenId::new("OTHERTOK-123456");
const OTHER_TOKEN: TestTokenId = TestTokenId::new("OTHER-TOKEN");
const PAYABLE_FEATURES_TOKEN: TestTokenId = TestTokenId::new("PAYABLE-FEATURES-TOKEN");
const SFT_123: TestTokenId = TestTokenId::new("SFT-123");
const TOK_000001: TestTokenId = TestTokenId::new("TOK-000001");
const TOK_000002: TestTokenId = TestTokenId::new("TOK-000002");
const TOK_000003: TestTokenId = TestTokenId::new("TOK-000003");
const TOK_123456: TestTokenId = TestTokenId::new("TOK-123456");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/payable-features");
    blockchain.register_contract(
        PAYABLE_FEATURES_CODE_PATH,
        payable_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn payable_all_1_scen() {
    let mut world = world();
    payable_all_1_scen_steps(&mut world);
}

pub fn payable_all_1_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(10_000u64)
        .esdt_balance(OTHERTOK_123456, 500u64)
        .esdt_nft_balance(SFT_123, 5, 20u64, ())
        .esdt_balance(TOK_123456, 1_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_all-ZERO")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_all()
        .returns(ExpectValue(ScenarioValueRaw::new("")))
        .run();

    world
        .tx()
        .id("payable_all-EGLD")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_all()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1_000u64).unwrap())
        .returns(ExpectValue(ScenarioValueRaw::new(ValueSubTree::List(vec![ValueSubTree::Str("nested:str:EGLD-000000|u64:0|biguint:1000".to_string())]))))
        .run();

    world
        .tx()
        .id("payable_all-multi-EGLD-2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_all()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1_001u64).unwrap())
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1_002u64).unwrap())
        .returns(ExpectValue(ScenarioValueRaw::new(ValueSubTree::List(vec![ValueSubTree::Str("nested:str:EGLD-000000|u64:0|biguint:1001".to_string()), ValueSubTree::Str("nested:str:EGLD-000000|u64:0|biguint:1002".to_string())]))))
        .run();

    world
        .tx()
        .id("payable_all-EGLD-with-ESDT")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_all()
        .payment(Payment::try_new(TOK_123456, 0, 100u64).unwrap())
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1_005u64).unwrap())
        .payment(Payment::try_new(OTHERTOK_123456, 0, 400u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1_006u64).unwrap())
        .returns(ExpectValue(ScenarioValueRaw::new(ValueSubTree::List(vec![ValueSubTree::Str("nested:str:TOK-123456|u64:0|biguint:100|".to_string()), ValueSubTree::Str("nested:str:EGLD-000000|u64:0|biguint:1005".to_string()), ValueSubTree::Str("nested:str:OTHERTOK-123456|u64:0|biguint:400".to_string()), ValueSubTree::Str("nested:str:SFT-123|u64:5|biguint:10".to_string()), ValueSubTree::Str("nested:str:EGLD-000000|u64:0|biguint:1006".to_string())]))))
        .run();

}

#[test]
fn payable_all_2_scen() {
    let mut world = world();
    payable_all_2_scen_steps(&mut world);
}

pub fn payable_all_2_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(10_000u64)
        .esdt_balance(OTHERTOK_123456, 500u64)
        .esdt_nft_balance(SFT_123, 5, 20u64, ())
        .esdt_balance(TOK_123456, 1_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_all")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_all()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 15u64).unwrap())
        .payment(Payment::try_new(TOK_123456, 0, 100u64).unwrap())
        .payment(Payment::try_new(OTHERTOK_123456, 0, 400u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .returns(ExpectValue(ScenarioValueRaw::new(ValueSubTree::List(vec![ValueSubTree::Str("nested:str:EGLD-000000|u64:0|biguint:15".to_string()), ValueSubTree::Str("nested:str:TOK-123456|u64:0|biguint:100".to_string()), ValueSubTree::Str("nested:str:OTHERTOK-123456|u64:0|biguint:400".to_string()), ValueSubTree::Str("nested:str:SFT-123|u64:5|biguint:10".to_string())]))))
        .run();

}

#[test]
fn payable_all_3_scen() {
    let mut world = world();
    payable_all_3_scen_steps(&mut world);
}

pub fn payable_all_3_scen_steps(world: &mut ScenarioWorld) {
    world.account(A_USER_ADDRESS).nonce(0u64)
        .balance(100u64)
        .esdt_balance(TOK_123456, 100u64)
        ;
    world.account(PAYABLE_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("")
        .from(A_USER_ADDRESS)
        .to(PAYABLE_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_all()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 10u64).unwrap())
        .returns(ExpectValue(ScenarioValueRaw::new(ValueSubTree::List(vec![ValueSubTree::Str("nested:str:EGLD-000000|u64:0|biguint:10".to_string())]))))
        .run();


}

#[test]
fn payable_all_transfers_1_scen() {
    let mut world = world();
    payable_all_transfers_1_scen_steps(&mut world);
}

pub fn payable_all_transfers_1_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(10_000u64)
        .esdt_balance(OTHERTOK_123456, 500u64)
        .esdt_nft_balance(SFT_123, 5, 20u64, ())
        .esdt_balance(TOK_123456, 1_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_all_transfers-ZERO")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_all_transfers()
        .returns(ExpectValue(ScenarioValueRaw::new("")))
        .run();

    world
        .tx()
        .id("payable_all_transfers-EGLD")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_all_transfers()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1_000u64).unwrap())
        .returns(ExpectValue(ScenarioValueRaw::new(ValueSubTree::List(vec![ValueSubTree::Str("nested:str:EGLD|u64:0|biguint:1000".to_string())]))))
        .run();

    world
        .tx()
        .id("payable_all_transfers-multi-EGLD-2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_all_transfers()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1_001u64).unwrap())
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1_002u64).unwrap())
        .returns(ExpectValue(ScenarioValueRaw::new(ValueSubTree::List(vec![ValueSubTree::Str("nested:str:EGLD|u64:0|biguint:1001".to_string()), ValueSubTree::Str("nested:str:EGLD|u64:0|biguint:1002".to_string())]))))
        .run();

    world
        .tx()
        .id("payable_all_transfers-EGLD-with-ESDT")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_all_transfers()
        .payment(Payment::try_new(TOK_123456, 0, 100u64).unwrap())
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1_005u64).unwrap())
        .payment(Payment::try_new(OTHERTOK_123456, 0, 400u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1_006u64).unwrap())
        .returns(ExpectValue(ScenarioValueRaw::new(ValueSubTree::List(vec![ValueSubTree::Str("nested:str:TOK-123456|u64:0|biguint:100|".to_string()), ValueSubTree::Str("nested:str:EGLD|u64:0|biguint:1005".to_string()), ValueSubTree::Str("nested:str:OTHERTOK-123456|u64:0|biguint:400".to_string()), ValueSubTree::Str("nested:str:SFT-123|u64:5|biguint:10".to_string()), ValueSubTree::Str("nested:str:EGLD|u64:0|biguint:1006".to_string())]))))
        .run();

}

#[test]
fn payable_all_transfers_2_scen() {
    let mut world = world();
    payable_all_transfers_2_scen_steps(&mut world);
}

pub fn payable_all_transfers_2_scen_steps(world: &mut ScenarioWorld) {
    world.account(A_USER_ADDRESS).nonce(0u64)
        .balance(100u64)
        .esdt_balance(TOK_123456, 100u64)
        ;
    world.account(PAYABLE_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("")
        .from(A_USER_ADDRESS)
        .to(PAYABLE_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_all_transfers()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 10u64).unwrap())
        .returns(ExpectValue(ScenarioValueRaw::new(ValueSubTree::List(vec![ValueSubTree::Str("nested:str:EGLD|u64:0|biguint:10".to_string())]))))
        .run();


}

#[test]
fn payable_any_1_scen() {
    let mut world = world();
    payable_any_1_scen_steps(&mut world);
}

pub fn payable_any_1_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(1_000_000_000_000u64)
        .esdt_balance(OTHER_TOKEN, 1_000_000_000_000u64)
        .esdt_balance(PAYABLE_FEATURES_TOKEN, 1_000_000_000_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_any_1.1")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_any_1()
        .returns(ExpectValue(MultiValue2::new(0u64, EGLD)))
        .run();

    world
        .tx()
        .id("payable_any_1.2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_any_1()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 5u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(5u64, EGLD)))
        .run();

    world
        .tx()
        .id("payable_any_1.3")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_any_1()
        .payment(Payment::try_new(PAYABLE_FEATURES_TOKEN, 0, 100u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(100u64, PAYABLE_FEATURES_TOKEN)))
        .run();

}

#[test]
fn payable_any_2_scen() {
    let mut world = world();
    payable_any_2_scen_steps(&mut world);
}

pub fn payable_any_2_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(1_000_000_000_000u64)
        .esdt_balance(OTHER_TOKEN, 1_000_000_000_000u64)
        .esdt_balance(PAYABLE_FEATURES_TOKEN, 1_000_000_000_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_any_2.1")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_any_2()
        .returns(ExpectValue(MultiValue2::new(0u64, EGLD)))
        .run();

    world
        .tx()
        .id("payable_any_2.2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_any_2()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 5u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(5u64, EGLD)))
        .run();

    world
        .tx()
        .id("payable_any_2.3")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_any_2()
        .payment(Payment::try_new(PAYABLE_FEATURES_TOKEN, 0, 100u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(100u64, PAYABLE_FEATURES_TOKEN)))
        .run();

}

#[test]
fn payable_any_3_scen() {
    let mut world = world();
    payable_any_3_scen_steps(&mut world);
}

pub fn payable_any_3_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(1_000_000_000_000u64)
        .esdt_balance(OTHER_TOKEN, 1_000_000_000_000u64)
        .esdt_balance(PAYABLE_FEATURES_TOKEN, 1_000_000_000_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_any_3.1")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_any_3()
        .returns(ExpectValue(MultiValue2::new(0u64, EGLD)))
        .run();

    world
        .tx()
        .id("payable_any_3.2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_any_3()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 5u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(5u64, EGLD)))
        .run();

    world
        .tx()
        .id("payable_any_3.3")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_any_3()
        .payment(Payment::try_new(PAYABLE_FEATURES_TOKEN, 0, 100u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(100u64, PAYABLE_FEATURES_TOKEN)))
        .run();

}

#[test]
fn payable_any_4_scen() {
    let mut world = world();
    payable_any_4_scen_steps(&mut world);
}

pub fn payable_any_4_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(1_000_000_000_000u64)
        .esdt_balance(OTHER_TOKEN, 1_000_000_000_000u64)
        .esdt_balance(PAYABLE_FEATURES_TOKEN, 1_000_000_000_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_any_4.1")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_any_4()
        .returns(ExpectValue(MultiValue2::new(0u64, EGLD)))
        .run();

    world
        .tx()
        .id("payable_any_4.2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_any_4()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 5u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(5u64, EGLD)))
        .run();

    world
        .tx()
        .id("payable_any_4.3")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_any_4()
        .payment(Payment::try_new(PAYABLE_FEATURES_TOKEN, 0, 100u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(100u64, PAYABLE_FEATURES_TOKEN)))
        .run();

}

#[test]
fn payable_any_5_scen() {
    let mut world = world();
    payable_any_5_scen_steps(&mut world);
}

pub fn payable_any_5_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(1_000_000_000_000u64)
        .esdt_balance(OTHER_TOKEN, 1_000_000_000_000u64)
        .esdt_balance(PAYABLE_FEATURES_TOKEN, 1_000_000_000_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_any_5.1")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_any_5()
        .run();

    world
        .tx()
        .id("payable_any_5.2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_any_5()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 5u64).unwrap())
        .returns(ExpectValue(OptionalValue::Some(MultiValue3::new(TestTokenId::EGLD_000000, 0u64, NonZeroBigUint::try_from(5u64).unwrap()))))
        .run();

    world
        .tx()
        .id("payable_any_5.3")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_any_5()
        .payment(Payment::try_new(PAYABLE_FEATURES_TOKEN, 0, 100u64).unwrap())
        .returns(ExpectValue(OptionalValue::Some(MultiValue3::new(PAYABLE_FEATURES_TOKEN, 0u64, NonZeroBigUint::try_from(100u64).unwrap()))))
        .run();

}

#[test]
fn payable_array_scen() {
    let mut world = world();
    payable_array_scen_steps(&mut world);
}

pub fn payable_array_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(10_000u64)
        .esdt_nft_balance(SFT_123, 5, 20u64, ())
        .esdt_balance(TOK_000001, 1_000u64)
        .esdt_balance(TOK_000002, 500u64)
        .esdt_balance(TOK_000003, 500u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payment_array_3-too-many")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payment_array_3()
        .payment(Payment::try_new(TOK_000001, 0, 100u64).unwrap())
        .payment(Payment::try_new(TOK_000002, 0, 400u64).unwrap())
        .payment(Payment::try_new(TOK_000003, 0, 400u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 103u64).unwrap())
        .with_result(ExpectError(4, "incorrect number of transfers"))
        .run();

    world
        .tx()
        .id("payment_array_3-too-few")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payment_array_3()
        .payment(Payment::try_new(TOK_000001, 0, 100u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .with_result(ExpectError(4, "incorrect number of transfers"))
        .run();

    world
        .tx()
        .id("payment_array_3-ok")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payment_array_3()
        .payment(Payment::try_new(TOK_000001, 0, 100u64).unwrap())
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 400u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .returns(ExpectValue(MultiValue3::new(ScenarioValueRaw::new("nested:str:TOK-000001|u64:0|biguint:100"), ScenarioValueRaw::new("nested:str:EGLD-000000|u64:0|biguint:400"), ScenarioValueRaw::new("nested:str:SFT-123|u64:5|biguint:10"))))
        .run();

}

#[test]
fn payable_array_egld_or_esdt_scen() {
    let mut world = world();
    payable_array_egld_or_esdt_scen_steps(&mut world);
}

pub fn payable_array_egld_or_esdt_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(10_000u64)
        .esdt_nft_balance(SFT_123, 5, 20u64, ())
        .esdt_balance(TOK_000001, 1_000u64)
        .esdt_balance(TOK_000002, 500u64)
        .esdt_balance(TOK_000003, 500u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payment_array_egld_or_esdt_3-too-many")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payment_array_egld_or_esdt_3()
        .payment(Payment::try_new(TOK_000001, 0, 100u64).unwrap())
        .payment(Payment::try_new(TOK_000002, 0, 400u64).unwrap())
        .payment(Payment::try_new(TOK_000003, 0, 400u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 103u64).unwrap())
        .with_result(ExpectError(4, "incorrect number of transfers"))
        .run();

    world
        .tx()
        .id("payment_array_egld_or_esdt_3-too-few")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payment_array_egld_or_esdt_3()
        .payment(Payment::try_new(TOK_000001, 0, 100u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .with_result(ExpectError(4, "incorrect number of transfers"))
        .run();

    world
        .tx()
        .id("payment_array_egld_or_esdt_3-ok")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payment_array_egld_or_esdt_3()
        .payment(Payment::try_new(TOK_000001, 0, 100u64).unwrap())
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 400u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .returns(ExpectValue(MultiValue3::new(ScenarioValueRaw::new("nested:str:TOK-000001|u64:0|biguint:100"), ScenarioValueRaw::new("nested:str:EGLD|u64:0|biguint:400"), ScenarioValueRaw::new("nested:str:SFT-123|u64:5|biguint:10"))))
        .run();

}

#[test]
fn payable_array_esdt_scen() {
    let mut world = world();
    payable_array_esdt_scen_steps(&mut world);
}

pub fn payable_array_esdt_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(10_000u64)
        .esdt_nft_balance(SFT_123, 5, 20u64, ())
        .esdt_balance(TOK_000001, 1_000u64)
        .esdt_balance(TOK_000002, 500u64)
        .esdt_balance(TOK_000003, 500u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payment_array_esdt_3-too-many")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payment_array_esdt_3()
        .payment(Payment::try_new(TOK_000001, 0, 100u64).unwrap())
        .payment(Payment::try_new(TOK_000002, 0, 400u64).unwrap())
        .payment(Payment::try_new(TOK_000003, 0, 400u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .with_result(ExpectError(4, "incorrect number of transfers"))
        .run();

    world
        .tx()
        .id("payment_array_esdt_3-too-few")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payment_array_esdt_3()
        .payment(Payment::try_new(TOK_000001, 0, 100u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .with_result(ExpectError(4, "incorrect number of transfers"))
        .run();

    world
        .tx()
        .id("payment_array_esdt_3-bad-egld")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payment_array_esdt_3()
        .payment(Payment::try_new(TOK_000003, 0, 400u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 103u64).unwrap())
        .with_result(ExpectError(4, "unexpected EGLD transfer"))
        .run();

    world
        .tx()
        .id("payment_array_esdt_3-ok")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payment_array_esdt_3()
        .payment(Payment::try_new(TOK_000001, 0, 100u64).unwrap())
        .payment(Payment::try_new(TOK_000002, 0, 400u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .returns(ExpectValue(MultiValue3::new(ScenarioValueRaw::new("nested:str:TOK-000001|u64:0|biguint:100"), ScenarioValueRaw::new("nested:str:TOK-000002|u64:0|biguint:400"), ScenarioValueRaw::new("nested:str:SFT-123|u64:5|biguint:10"))))
        .run();

}

#[test]
fn payable_egld_1_scen() {
    let mut world = world();
    payable_egld_1_scen_steps(&mut world);
}

pub fn payable_egld_1_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(1_000_000_000_000u64)
        .esdt_balance(OTHER_TOKEN, 1_000_000_000_000u64)
        .esdt_balance(PAYABLE_FEATURES_TOKEN, 1_000_000_000_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_egld_1.1")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_egld_1()
        .returns(ExpectValue(MultiValue2::new(0u64, EGLD)))
        .run();

    world
        .tx()
        .id("payable_egld_1.2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_egld_1()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 5u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(5u64, EGLD)))
        .run();

    world
        .tx()
        .id("payable_egld_1.3")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_egld_1()
        .payment(Payment::try_new(PAYABLE_FEATURES_TOKEN, 0, 100u64).unwrap())
        .with_result(ExpectError(4, "function does not accept ESDT payment"))
        .run();

}

#[test]
fn payable_egld_2_scen() {
    let mut world = world();
    payable_egld_2_scen_steps(&mut world);
}

pub fn payable_egld_2_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(1_000_000_000_000u64)
        .esdt_balance(OTHER_TOKEN, 1_000_000_000_000u64)
        .esdt_balance(PAYABLE_FEATURES_TOKEN, 1_000_000_000_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_egld_2.1")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_egld_2()
        .returns(ExpectValue(MultiValue2::new(0u64, EGLD)))
        .run();

    world
        .tx()
        .id("payable_egld_2.2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_egld_2()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 5u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(5u64, EGLD)))
        .run();

    world
        .tx()
        .id("payable_egld_2.3")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_egld_2()
        .payment(Payment::try_new(PAYABLE_FEATURES_TOKEN, 0, 100u64).unwrap())
        .with_result(ExpectError(4, "function does not accept ESDT payment"))
        .run();

}

#[test]
fn payable_egld_3_scen() {
    let mut world = world();
    payable_egld_3_scen_steps(&mut world);
}

pub fn payable_egld_3_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(1_000_000_000_000u64)
        .esdt_balance(OTHER_TOKEN, 1_000_000_000_000u64)
        .esdt_balance(PAYABLE_FEATURES_TOKEN, 1_000_000_000_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_egld_3.1")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_egld_3()
        .returns(ExpectValue(MultiValue2::new(0u64, EGLD)))
        .run();

    world
        .tx()
        .id("payable_egld_3.2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_egld_3()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 5u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(5u64, EGLD)))
        .run();

    world
        .tx()
        .id("payable_egld_3.3")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_egld_3()
        .payment(Payment::try_new(PAYABLE_FEATURES_TOKEN, 0, 100u64).unwrap())
        .with_result(ExpectError(4, "function does not accept ESDT payment"))
        .run();

}

#[test]
fn payable_egld_4_scen() {
    let mut world = world();
    payable_egld_4_scen_steps(&mut world);
}

pub fn payable_egld_4_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(1_000_000_000_000u64)
        .esdt_balance(OTHER_TOKEN, 1_000_000_000_000u64)
        .esdt_balance(PAYABLE_FEATURES_TOKEN, 1_000_000_000_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_egld_4.1")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_egld_4()
        .returns(ExpectValue(MultiValue2::new(0u64, EGLD)))
        .run();

    world
        .tx()
        .id("payable_egld_4.2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_egld_4()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 5u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(5u64, EGLD)))
        .run();

    world
        .tx()
        .id("payable_egld_4.3")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_egld_4()
        .payment(Payment::try_new(PAYABLE_FEATURES_TOKEN, 0, 100u64).unwrap())
        .with_result(ExpectError(4, "function does not accept ESDT payment"))
        .run();

}

#[test]
fn payable_egld_5_scen() {
    let mut world = world();
    payable_egld_5_scen_steps(&mut world);
}

pub fn payable_egld_5_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(1_000_000_000_000u64)
        .esdt_balance(OTHER_TOKEN, 1_000_000_000_000u64)
        .esdt_balance(PAYABLE_FEATURES_TOKEN, 1_000_000_000_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_egld_5.1")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_egld_5()
        .run();

    world
        .tx()
        .id("payable_egld_5.2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_egld_5()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 5u64).unwrap())
        .returns(ExpectValue(OptionalValue::Some(MultiValue3::new(TestTokenId::EGLD_000000, 0u64, NonZeroBigUint::try_from(5u64).unwrap()))))
        .run();

    world
        .tx()
        .id("payable_egld_5.3")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_egld_5()
        .payment(Payment::try_new(PAYABLE_FEATURES_TOKEN, 0, 100u64).unwrap())
        .with_result(ExpectError(4, "function does not accept ESDT payment"))
        .run();

}

#[test]
fn payable_legacy_egld_esdt_scen() {
    let mut world = world();
    payable_legacy_egld_esdt_scen_steps(&mut world);
}

pub fn payable_legacy_egld_esdt_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(10_000u64)
        .esdt_balance(OTHERTOK_123456, 500u64)
        .esdt_nft_balance(SFT_123, 5, 20u64, ())
        .esdt_balance(TOK_123456, 1_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_legacy_egld_esdt")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_legacy_egld_esdt()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 100u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(100u64, ScenarioValueRaw::new(""))))
        .run();

    world
        .tx()
        .id("call-value-single-esdt")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_legacy_egld_esdt()
        .payment(Payment::try_new(TOK_123456, 0, 100u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(0u64, ScenarioValueRaw::new(ValueSubTree::List(vec![ValueSubTree::Str("nested:str:TOK-123456|u64:0|biguint:100".to_string())])))))
        .run();

    world
        .tx()
        .id("call-value-multi-esdt")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_legacy_egld_esdt()
        .payment(Payment::try_new(TOK_123456, 0, 100u64).unwrap())
        .payment(Payment::try_new(OTHERTOK_123456, 0, 400u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(0u64, ScenarioValueRaw::new(ValueSubTree::List(vec![ValueSubTree::Str("nested:str:TOK-123456|u64:0|biguint:100".to_string()), ValueSubTree::Str("nested:str:OTHERTOK-123456|u64:0|biguint:400".to_string()), ValueSubTree::Str("nested:str:SFT-123|u64:5|biguint:10".to_string())])))))
        .run();

}

#[test]
fn payable_multiple_scen() {
    let mut world = world();
    payable_multiple_scen_steps(&mut world);
}

pub fn payable_multiple_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(10_000u64)
        .esdt_balance(OTHERTOK_123456, 500u64)
        .esdt_nft_balance(SFT_123, 5, 20u64, ())
        .esdt_balance(TOK_123456, 1_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payment-multiple")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payment_multiple()
        .payment(Payment::try_new(TOK_123456, 0, 100u64).unwrap())
        .payment(Payment::try_new(OTHERTOK_123456, 0, 400u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .returns(ExpectValue(ScenarioValueRaw::new(ValueSubTree::List(vec![ValueSubTree::Str("nested:str:TOK-123456|u64:0|biguint:100|".to_string()), ValueSubTree::Str("nested:str:OTHERTOK-123456|u64:0|biguint:400".to_string()), ValueSubTree::Str("nested:str:SFT-123|u64:5|biguint:10".to_string())]))))
        .run();

    world
        .tx()
        .id("payment-multiple-bad-egld")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payment_multiple()
        .payment(Payment::try_new(TOK_123456, 0, 100u64).unwrap())
        .payment(Payment::try_new(OTHERTOK_123456, 0, 100u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 120u64).unwrap())
        .with_result(ExpectError(4, "unexpected EGLD transfer"))
        .run();

}

#[test]
fn payable_multiple_egld_scen() {
    let mut world = world();
    payable_multiple_egld_scen_steps(&mut world);
}

pub fn payable_multiple_egld_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(10_000u64)
        .esdt_balance(OTHERTOK_123456, 500u64)
        .esdt_nft_balance(SFT_123, 5, 20u64, ())
        .esdt_balance(TOK_123456, 1_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payment-multiple-egld")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_all_transfers()
        .payment(Payment::try_new(TOK_123456, 0, 100u64).unwrap())
        .payment(Payment::try_new(OTHERTOK_123456, 0, 400u64).unwrap())
        .payment(Payment::try_new(SFT_123, 5, 10u64).unwrap())
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 120u64).unwrap())
        .returns(ExpectValue(ScenarioValueRaw::new(ValueSubTree::List(vec![ValueSubTree::Str("nested:str:TOK-123456|u64:0|biguint:100|".to_string()), ValueSubTree::Str("nested:str:OTHERTOK-123456|u64:0|biguint:400".to_string()), ValueSubTree::Str("nested:str:SFT-123|u64:5|biguint:10".to_string()), ValueSubTree::Str("nested:str:EGLD|u64:0|biguint:120".to_string())]))))
        .run();

}

#[test]
fn payable_token_1_scen() {
    let mut world = world();
    payable_token_1_scen_steps(&mut world);
}

pub fn payable_token_1_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(1_000_000_000_000u64)
        .esdt_balance(OTHER_TOKEN, 1_000_000_000_000u64)
        .esdt_balance(PAYABLE_FEATURES_TOKEN, 1_000_000_000_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_token_1.1")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_1()
        .with_result(ExpectError(4, "single ESDT payment expected"))
        .run();

    world
        .tx()
        .id("payable_token_1.2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_1()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 5u64).unwrap())
        .with_result(ExpectError(4, "single ESDT payment expected"))
        .run();

    world
        .tx()
        .id("payable_token_1.3")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_1()
        .payment(Payment::try_new(PAYABLE_FEATURES_TOKEN, 0, 100u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(100u64, PAYABLE_FEATURES_TOKEN)))
        .run();

    world
        .tx()
        .id("payable_token_1.4")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_1()
        .payment(Payment::try_new(OTHER_TOKEN, 0, 100u64).unwrap())
        .with_result(ExpectError(4, "bad call value token provided"))
        .run();

}

#[test]
fn payable_token_2_scen() {
    let mut world = world();
    payable_token_2_scen_steps(&mut world);
}

pub fn payable_token_2_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(1_000_000_000_000u64)
        .esdt_balance(OTHER_TOKEN, 1_000_000_000_000u64)
        .esdt_balance(PAYABLE_FEATURES_TOKEN, 1_000_000_000_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_token_2.1")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_2()
        .with_result(ExpectError(4, "single ESDT payment expected"))
        .run();

    world
        .tx()
        .id("payable_token_2.2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_2()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 5u64).unwrap())
        .with_result(ExpectError(4, "single ESDT payment expected"))
        .run();

    world
        .tx()
        .id("payable_token_2.3")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_2()
        .payment(Payment::try_new(PAYABLE_FEATURES_TOKEN, 0, 100u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(100u64, PAYABLE_FEATURES_TOKEN)))
        .run();

    world
        .tx()
        .id("payable_token_2.4")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_2()
        .payment(Payment::try_new(OTHER_TOKEN, 0, 100u64).unwrap())
        .with_result(ExpectError(4, "bad call value token provided"))
        .run();

}

#[test]
fn payable_token_3_scen() {
    let mut world = world();
    payable_token_3_scen_steps(&mut world);
}

pub fn payable_token_3_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(1_000_000_000_000u64)
        .esdt_balance(OTHER_TOKEN, 1_000_000_000_000u64)
        .esdt_balance(PAYABLE_FEATURES_TOKEN, 1_000_000_000_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_token_3.1")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_3()
        .with_result(ExpectError(4, "single ESDT payment expected"))
        .run();

    world
        .tx()
        .id("payable_token_3.2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_3()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 5u64).unwrap())
        .with_result(ExpectError(4, "single ESDT payment expected"))
        .run();

    world
        .tx()
        .id("payable_token_3.3")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_3()
        .payment(Payment::try_new(PAYABLE_FEATURES_TOKEN, 0, 100u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(100u64, PAYABLE_FEATURES_TOKEN)))
        .run();

    world
        .tx()
        .id("payable_token_3.4")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_3()
        .payment(Payment::try_new(OTHER_TOKEN, 0, 100u64).unwrap())
        .with_result(ExpectError(4, "bad call value token provided"))
        .run();

}

#[test]
fn payable_token_4_scen() {
    let mut world = world();
    payable_token_4_scen_steps(&mut world);
}

pub fn payable_token_4_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(1_000_000_000_000u64)
        .esdt_balance(OTHER_TOKEN, 1_000_000_000_000u64)
        .esdt_balance(PAYABLE_FEATURES_TOKEN, 1_000_000_000_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_token_4.1")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_4()
        .with_result(ExpectError(4, "single ESDT payment expected"))
        .run();

    world
        .tx()
        .id("payable_token_4.2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_4()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 5u64).unwrap())
        .with_result(ExpectError(4, "single ESDT payment expected"))
        .run();

    world
        .tx()
        .id("payable_token_4.3")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_4()
        .payment(Payment::try_new(PAYABLE_FEATURES_TOKEN, 0, 100u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(100u64, PAYABLE_FEATURES_TOKEN)))
        .run();

    world
        .tx()
        .id("payable_token_4.4")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_4()
        .payment(Payment::try_new(OTHER_TOKEN, 0, 100u64).unwrap())
        .with_result(ExpectError(4, "bad call value token provided"))
        .run();

}

#[test]
fn payable_token_5_scen() {
    let mut world = world();
    payable_token_5_scen_steps(&mut world);
}

pub fn payable_token_5_scen_steps(world: &mut ScenarioWorld) {
    world.account(AN_ACCOUNT_ADDRESS).nonce(0u64)
        .balance(1_000_000_000_000u64)
        .esdt_balance(OTHER_TOKEN, 1_000_000_000_000u64)
        .esdt_balance(PAYABLE_FEATURES_TOKEN, 1_000_000_000_000u64)
        ;
    world.account(PAYABLE_FEATURES_ADDRESS).nonce(0u64)
        .balance(0u64)
        .code(PAYABLE_FEATURES_CODE_PATH)
        ;

    world
        .tx()
        .id("payable_token_1.1")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_1()
        .with_result(ExpectError(4, "single ESDT payment expected"))
        .run();

    world
        .tx()
        .id("payable_token_1.2")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_1()
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 5u64).unwrap())
        .with_result(ExpectError(4, "single ESDT payment expected"))
        .run();

    world
        .tx()
        .id("payable_token_1.3")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_1()
        .payment(Payment::try_new(PAYABLE_FEATURES_TOKEN, 0, 100u64).unwrap())
        .returns(ExpectValue(MultiValue2::new(100u64, PAYABLE_FEATURES_TOKEN)))
        .run();

    world
        .tx()
        .id("payable_token_1.4")
        .from(AN_ACCOUNT_ADDRESS)
        .to(PAYABLE_FEATURES_ADDRESS)
        .typed(payable_features_proxy::PayableFeaturesProxy)
        .payable_token_1()
        .payment(Payment::try_new(OTHER_TOKEN, 0, 100u64).unwrap())
        .with_result(ExpectError(4, "bad call value token provided"))
        .run();

}

}
