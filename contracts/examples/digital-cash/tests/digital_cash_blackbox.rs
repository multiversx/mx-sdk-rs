// Auto-generated blackbox tests from scenarios

use multiversx_sc_scenario::imports::*;

use digital_cash::*;

const DIGITAL_CASH_CODE_PATH: MxscPath = MxscPath::new("output/digital-cash.mxsc.json");
const ACC1_ADDRESS: TestAddress = TestAddress::new("acc1");
const ACC2_ADDRESS: TestAddress = TestAddress::new("acc2");
const ACC3_ADDRESS: TestAddress = TestAddress::new("acc3");
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("the_digital_cash_contract");
const CASHTOKEN_112233: TestTokenId = TestTokenId::new("CASHTOKEN-112233");
const CASHTOKEN_123456: TestTokenId = TestTokenId::new("CASHTOKEN-123456");
const CASHTOKEN_445566: TestTokenId = TestTokenId::new("CASHTOKEN-445566");
const CASHTOKEN_778899: TestTokenId = TestTokenId::new("CASHTOKEN-778899");
const ESDT_778899: TestTokenId = TestTokenId::new("ESDT-778899");
const DEPOSIT_KEY_01: [u8; 32] =
    hex!("d0474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60");
const DEPOSIT_KEY_02: [u8; 32] =
    hex!("db474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60");
const DEPOSIT_KEY_03: [u8; 32] =
    hex!("287bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd");
const DEPOSIT_KEY_04: [u8; 32] =
    hex!("487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd");
const DEPOSIT_KEY_05: [u8; 32] =
    hex!("805532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d");
const DEPOSIT_KEY_06: [u8; 32] =
    hex!("885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d");
const DEPOSIT_KEY_07: [u8; 32] =
    hex!("a40e72cdac3580e7203a4c2565c932f7691c35e624bcfd82718d7f559c88f440");
const DEPOSIT_KEY_08: [u8; 32] =
    hex!("8dc17613990e9b7476401a36d112d1a4d31190dec21e7e9a3c933872a27613ee");
const DEPOSIT_KEY_09: [u8; 32] =
    hex!("558fd9b0dd9fed2d3bed883d3b92907743362c56b9728392f84b261f1cc5ae0a");
const DEPOSIT_KEY_10: [u8; 32] =
    hex!("e808c2baab2a20b612f1351da5945c52c60f5321c6cde572149db90c9e8fbfc7");
const SIGNATURE_01: [u8; 64] = hex!(
    "443c75ceadb9ec42acff7e1b92e0305182279446c1d6c0502959484c147a0430d3f96f0b988e646f6736d5bf8e4a843d8ba7730d6fa7e60f0ef3edd225ce630f"
);
const SIGNATURE_02: [u8; 64] = hex!(
    "dd092ec3a8d971daede79da4e5c5c90d66af9f2209a6f6541affa00c46a72fc2596e4db1b1bb226ce76e50730733078ff74a79ff7d0d185054375e0989330600"
);
const SIGNATURE_03: [u8; 64] = hex!(
    "1ac4f6d4d45836d97ffeda83a66aaea7631a3bb3d4063421ccb2b9de9485bdb4c9bd6e44e003f6a9c9eb74379467238204ff579471d203b1878c3f1530592a02"
);

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/examples/digital-cash");
    blockchain.register_contract(
        "mxsc:output/digital-cash.mxsc.json",
        digital_cash::ContractBuilder,
    );
    blockchain
}

#[test]
fn claim_egld_scen() {
    let mut world = world();
    claim_egld_scen_steps(&mut world);
}

pub fn claim_egld_scen_steps(world: &mut ScenarioWorld) {
    fund_egld_and_esdt_scen_steps(world);

    world
        .tx()
        .id("claim2-fail")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(&DEPOSIT_KEY_01, &SIGNATURE_01)
        .with_result(ExpectError(4, "non-existent key"))
        .run();

    // set block
    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(86_400_500u64));

    world
        .tx()
        .id("claim3-egld-fail-expired")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(&DEPOSIT_KEY_02, &SIGNATURE_01)
        .with_result(ExpectError(4, "deposit expired"))
        .run();

    // set block
    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(86_200_000u64));

    world
        .tx()
        .id("claim4-fail")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(&DEPOSIT_KEY_02, ScenarioValueRaw::new("0x12bb9e58dad361e9dadd0af1021ce53f9ca12b6580f5b3ab4f9c321ee055a38bcdcf35924eb46aef7a80b22387ded0b837734ac8a57e19ea12c33ef808f996c00"))
        .with_result(ExpectError(4, "argument decode error (signature): bad array length"))
        .run();

    world
        .tx()
        .id("claim5-egld")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(&DEPOSIT_KEY_02, &SIGNATURE_01)
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:collectedFees", "nested:str:EGLD-000000|biguint:10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;
}

#[test]
fn claim_esdt_scen() {
    let mut world = world();
    claim_esdt_scen_steps(&mut world);
}

pub fn claim_esdt_scen_steps(world: &mut ScenarioWorld) {
    fund_egld_and_esdt_scen_steps(world);

    world
        .tx()
        .id("claim2")
        .from(ACC1_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(&DEPOSIT_KEY_03, &SIGNATURE_02)
        .with_result(ExpectError(4, "non-existent key"))
        .run();

    // set block
    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(86_400_500u64));

    world
        .tx()
        .id("claim3-esdt")
        .from(ACC1_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(&DEPOSIT_KEY_04, &SIGNATURE_02)
        .with_result(ExpectError(4, "deposit expired"))
        .run();

    // set block
    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(86_200_000u64));

    world
        .tx()
        .id("claim4")
        .from(ACC1_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(&DEPOSIT_KEY_04, ScenarioValueRaw::new("0x1dd092ec3a8d971daede79da4e5c5c90d66af9f2209a6f6541affa00c46a72fc2596e4db1b1bb226ce76e50730733078ff74a79ff7d0d185054375e0989330600"))
        .with_result(ExpectError(4, "argument decode error (signature): bad array length"))
        .run();

    world
        .tx()
        .id("claim5-esdt")
        .from(ACC1_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(&DEPOSIT_KEY_04, &SIGNATURE_02)
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:collectedFees", "nested:str:EGLD-000000|biguint:10")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1,000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;
}

#[test]
fn claim_fees_scen() {
    let mut world = world();
    claim_fees_scen_steps(&mut world);
}

pub fn claim_fees_scen_steps(world: &mut ScenarioWorld) {
    claim_egld_scen_steps(world);

    world
        .tx()
        .id("claim-fees-fail")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim_fees()
        .with_result(ExpectError(4, "Endpoint can only be called by owner"))
        .run();

    // set block
    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(86_200_000u64));

    world
        .tx()
        .id("claim-fees-ok")
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim_fees()
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;
}

#[test]
fn claim_multi_esdt_scen() {
    let mut world = world();
    claim_multi_esdt_scen_steps(&mut world);
}

pub fn claim_multi_esdt_scen_steps(world: &mut ScenarioWorld) {
    fund_egld_and_esdt_scen_steps(world);

    world
        .tx()
        .id("claim2")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(&DEPOSIT_KEY_05, &SIGNATURE_03)
        .with_result(ExpectError(4, "non-existent key"))
        .run();

    // set block
    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(86_400_500u64));

    world
        .tx()
        .id("claim3-multi")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(&DEPOSIT_KEY_06, &SIGNATURE_03)
        .with_result(ExpectError(4, "deposit expired"))
        .run();

    // set block
    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(86_200_000u64));

    world
        .tx()
        .id("claim4")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(&DEPOSIT_KEY_06, ScenarioValueRaw::new("0x11ac4f6d4d45836d97ffeda83a66aaea7631a3bb3d4063421ccb2b9de9485bdb4c9bd6e44e003f6a9c9eb74379467238204ff579471d203b1878c3f1530592a02"))
        .with_result(ExpectError(4, "argument decode error (signature): bad array length"))
        .run();

    world
        .tx()
        .id("claim5-multi")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(&DEPOSIT_KEY_06, &SIGNATURE_03)
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:collectedFees", "nested:str:EGLD-000000|biguint:30")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1,000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;
}

#[test]
fn forward_scen() {
    let mut world = world();
    forward_scen_steps(&mut world);
}

pub fn forward_scen_steps(world: &mut ScenarioWorld) {
    fund_egld_and_esdt_scen_steps(world);

    world
        .tx()
        .id("forward-fail")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .forward(&DEPOSIT_KEY_02, &DEPOSIT_KEY_07, &SIGNATURE_01)
        .with_result(ExpectError(
            4,
            "forward deposit needs to exist in advance, with fees paid",
        ))
        .run();

    world
        .tx()
        .id("deposit-fees-2")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .deposit_fees(&DEPOSIT_KEY_07)
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1_000u64).unwrap())
        .run();

    world
        .tx()
        .id("forward-without-fees-ok")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .forward(&DEPOSIT_KEY_02, &DEPOSIT_KEY_07, &SIGNATURE_01)
        .run();

    world
        .tx()
        .id("deposit-fees-4")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .deposit_fees(&DEPOSIT_KEY_08)
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 500u64).unwrap())
        .run();

    world
        .tx()
        .id("forward-with-fees-fail")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .forward(&DEPOSIT_KEY_06, &DEPOSIT_KEY_06, &SIGNATURE_03)
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 500u64).unwrap())
        .with_result(ExpectError(
            4,
            "forward deposit needs to exist in advance, with fees paid",
        ))
        .run();

    world
        .tx()
        .id("forward-with-fees-ok")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .forward(&DEPOSIT_KEY_06, &DEPOSIT_KEY_08, &SIGNATURE_03)
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 500u64).unwrap())
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:collectedFees", "nested:str:EGLD-000000|biguint:40")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0x8dc17613990e9b7476401a36d112d1a4d31190dec21e7e9a3c933872a27613ee", "address:acc2|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1000")
        .check_storage("str:deposit|0xa40e72cdac3580e7203a4c2565c932f7691c35e624bcfd82718d7f559c88f440", "address:acc2|u32:1|nested:str:EGLD-000000|u64:0|biguint:1,000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;
}

#[test]
fn fund_egld_and_esdt_scen() {
    let mut world = world();
    fund_egld_and_esdt_scen_steps(&mut world);
}

pub fn fund_egld_and_esdt_scen_steps(world: &mut ScenarioWorld) {
    set_accounts_scen_steps(world);

    world
        .tx()
        .id("fail-fund")
        .from(ACC1_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .fund(&DEPOSIT_KEY_02, TimestampMillis::new(86_400_000u64))
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1_000u64).unwrap())
        .with_result(ExpectError(
            4,
            "deposit needs to exist before funding, with fees paid",
        ))
        .run();

    world
        .tx()
        .id("deposit-fees-1")
        .from(ACC1_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .deposit_fees(&DEPOSIT_KEY_02)
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1_000u64).unwrap())
        .run();

    world
        .tx()
        .id("fund-1")
        .from(ACC1_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .fund(&DEPOSIT_KEY_02, TimestampMillis::new(86_399_000u64))
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1_000u64).unwrap())
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;

    world
        .tx()
        .id("deposit-fees-2")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .deposit_fees(&DEPOSIT_KEY_04)
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1_000u64).unwrap())
        .run();

    world
        .tx()
        .id("fund-2")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .fund(&DEPOSIT_KEY_04, TimestampMillis::new(86_400_000u64))
        .payment(Payment::try_new(CASHTOKEN_123456, 0, 50u64).unwrap())
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;

    world
        .tx()
        .id("fund-fail-3")
        .from(ACC3_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .fund(&DEPOSIT_KEY_04, TimestampMillis::new(86_400_000u64))
        .payment(Payment::try_new(CASHTOKEN_112233, 0, 50u64).unwrap())
        .payment(Payment::try_new(CASHTOKEN_445566, 0, 50u64).unwrap())
        .payment(Payment::try_new(CASHTOKEN_778899, 0, 50u64).unwrap())
        .with_result(ExpectError(4, "invalid depositor"))
        .run();

    world
        .tx()
        .id("deposit-fees-3")
        .from(ACC3_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .deposit_fees(&DEPOSIT_KEY_06)
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1_000u64).unwrap())
        .run();

    world
        .tx()
        .id("fund-2")
        .from(ACC3_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .fund(&DEPOSIT_KEY_06, TimestampMillis::new(86_400_000u64))
        .payment(Payment::try_new(CASHTOKEN_112233, 0, 50u64).unwrap())
        .payment(Payment::try_new(CASHTOKEN_445566, 0, 50u64).unwrap())
        .payment(Payment::try_new(CASHTOKEN_778899, 0, 50u64).unwrap())
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;
}

#[test]
fn pay_fee_and_fund_egld_scen() {
    let mut world = world();
    pay_fee_and_fund_egld_scen_steps(&mut world);
}

pub fn pay_fee_and_fund_egld_scen_steps(world: &mut ScenarioWorld) {
    whitelist_blacklist_fee_tokens_scen_steps(world);

    world
        .tx()
        .id("pay-fee-and-fund-egld-fail")
        .from(ACC3_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .pay_fee_and_fund(&DEPOSIT_KEY_02, TimestampMillis::new(86_400_000u64))
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 9u64).unwrap())
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 1u64).unwrap())
        .with_result(ExpectError(4, "insufficient fees provided"))
        .run();

    world
        .tx()
        .id("pay-fee-and-fund-egld-success")
        .from(ACC3_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .pay_fee_and_fund(&DEPOSIT_KEY_02, TimestampMillis::new(86_400_000u64))
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 10u64).unwrap())
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 990u64).unwrap())
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:baseFee|nested:str:CASHTOKEN-778899", "3")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc3|u32:1|nested:str:EGLD-000000|u64:0|biguint:990|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:10")
        ;
}

#[test]
fn pay_fee_and_fund_esdt_multiple_scen() {
    let mut world = world();
    pay_fee_and_fund_esdt_multiple_scen_steps(&mut world);
}

pub fn pay_fee_and_fund_esdt_multiple_scen_steps(world: &mut ScenarioWorld) {
    whitelist_blacklist_fee_tokens_scen_steps(world);

    world
        .tx()
        .id("pay-fee-and-fund-esdt-fail")
        .from(ACC3_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .pay_fee_and_fund(&DEPOSIT_KEY_02, TimestampMillis::new(86_400_000u64))
        .payment(Payment::try_new(CASHTOKEN_445566, 0, 50u64).unwrap())
        .payment(Payment::try_new(CASHTOKEN_112233, 0, 50u64).unwrap())
        .with_result(ExpectError(4, "invalid fee token"))
        .run();

    world
        .tx()
        .id("pay-fee-and-fund-esdt-success")
        .from(ACC3_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .pay_fee_and_fund(&DEPOSIT_KEY_02, TimestampMillis::new(86_400_000u64))
        .payment(Payment::try_new(CASHTOKEN_778899, 0, 6u64).unwrap())
        .payment(Payment::try_new(CASHTOKEN_778899, 0, 44u64).unwrap())
        .payment(Payment::try_new(CASHTOKEN_112233, 0, 50u64).unwrap())
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:baseFee|nested:str:CASHTOKEN-778899", "3")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc3|u32:2|nested:str:CASHTOKEN-778899|u64:0|biguint:44|nested:str:CASHTOKEN-112233|u64:0|biguint:50|u64:86400_000|0x01|nested:str:CASHTOKEN-778899|biguint:6")
        ;
}

#[test]
fn pay_fee_and_fund_esdt_single_scen() {
    let mut world = world();
    pay_fee_and_fund_esdt_single_scen_steps(&mut world);
}

pub fn pay_fee_and_fund_esdt_single_scen_steps(world: &mut ScenarioWorld) {
    whitelist_blacklist_fee_tokens_scen_steps(world);

    world
        .tx()
        .id("pay-fee-and-fund-esdt-success")
        .from(ACC3_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .pay_fee_and_fund(&DEPOSIT_KEY_02, TimestampMillis::new(86_400_000u64))
        .payment(Payment::try_new(CASHTOKEN_778899, 0, 3u64).unwrap())
        .payment(Payment::try_new(CASHTOKEN_112233, 0, 50u64).unwrap())
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:baseFee|nested:str:CASHTOKEN-778899", "3")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc3|u32:1|nested:str:CASHTOKEN-112233|u64:0|biguint:50|u64:86400_000|0x01|nested:str:CASHTOKEN-778899|biguint:3")
        ;
}

#[test]
fn set_accounts_scen() {
    let mut world = world();
    set_accounts_scen_steps(&mut world);
}

pub fn set_accounts_scen_steps(world: &mut ScenarioWorld) {
    world
        .account(ACC1_ADDRESS)
        .nonce(0u64)
        .balance(1_000_000u64);
    world
        .account(ACC2_ADDRESS)
        .nonce(0u64)
        .balance(1_000_000u64)
        .esdt_balance(CASHTOKEN_123456, 100u64);
    world
        .account(ACC3_ADDRESS)
        .nonce(0u64)
        .balance(1_000_000u64)
        .esdt_balance(CASHTOKEN_112233, 100u64)
        .esdt_balance(CASHTOKEN_445566, 100u64)
        .esdt_balance(CASHTOKEN_778899, 100u64);
    world.account(OWNER_ADDRESS).nonce(0u64).balance(0u64);

    world
        .tx()
        .id("deploy")
        .from(OWNER_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .init(
            false,
            MultiValueVec::from(vec![MultiValue2::new(TestTokenId::EGLD_000000, 10u64)]),
        )
        .code(DIGITAL_CASH_CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();

    world
        .check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:feesDisabled", "false");
}

#[test]
fn whitelist_blacklist_fee_tokens_scen() {
    let mut world = world();
    whitelist_blacklist_fee_tokens_scen_steps(&mut world);
}

pub fn whitelist_blacklist_fee_tokens_scen_steps(world: &mut ScenarioWorld) {
    set_accounts_scen_steps(world);

    world
        .tx()
        .id("whitelist-success-1")
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .set_fee(CASHTOKEN_778899, 3u64)
        .run();

    world
        .tx()
        .id("whitelist-success-2")
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .set_fee(ESDT_778899, 5u64)
        .run();

    world
        .tx()
        .id("blacklist-success")
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .set_fee(ESDT_778899, 0u64)
        .run();

    world
        .check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:baseFee|nested:str:CASHTOKEN-778899", "3");
}

#[test]
fn withdraw_egld_scen() {
    let mut world = world();
    withdraw_egld_scen_steps(&mut world);
}

pub fn withdraw_egld_scen_steps(world: &mut ScenarioWorld) {
    fund_egld_and_esdt_scen_steps(world);

    world
        .tx()
        .id("withdraw-egld-fail-1")
        .from(ACC1_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(&DEPOSIT_KEY_02)
        .with_result(ExpectError(4, "cannot withdraw, deposit not expired yet"))
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1,000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;

    world
        .tx()
        .id("withdraw-egld-fail-2")
        .from(ACC1_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(&DEPOSIT_KEY_09)
        .with_result(ExpectError(4, "non-existent key"))
        .run();

    // set block
    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(86_400_000u64));

    world
        .tx()
        .id("withdraw-egld-success")
        .from(ACC1_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(&DEPOSIT_KEY_02)
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;
}

#[test]
fn withdraw_esdt_scen() {
    let mut world = world();
    withdraw_esdt_scen_steps(&mut world);
}

pub fn withdraw_esdt_scen_steps(world: &mut ScenarioWorld) {
    fund_egld_and_esdt_scen_steps(world);

    world
        .tx()
        .id("withdraw-esdt-fail-1")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(&DEPOSIT_KEY_04)
        .with_result(ExpectError(4, "cannot withdraw, deposit not expired yet"))
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1,000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;

    world
        .tx()
        .id("withdraw-esdt-fail-2")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(&DEPOSIT_KEY_10)
        .with_result(ExpectError(4, "non-existent key"))
        .run();

    // set block
    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(86_400_500u64));

    world
        .tx()
        .id("withdraw-esdt-success")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(&DEPOSIT_KEY_04)
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1,000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;
}

#[test]
fn withdraw_multi_esdt_scen() {
    let mut world = world();
    withdraw_multi_esdt_scen_steps(&mut world);
}

pub fn withdraw_multi_esdt_scen_steps(world: &mut ScenarioWorld) {
    fund_egld_and_esdt_scen_steps(world);

    world
        .tx()
        .id("withdraw-esdt-1")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(&DEPOSIT_KEY_04)
        .with_result(ExpectError(4, "cannot withdraw, deposit not expired yet"))
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1,000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;

    world
        .tx()
        .id("withdraw-esdt-2")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(&DEPOSIT_KEY_05)
        .with_result(ExpectError(4, "non-existent key"))
        .run();

    // set block
    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(86_400_500u64));

    world
        .tx()
        .id("withdraw-esdt-3")
        .from(ACC2_ADDRESS)
        .to(SC_ADDRESS)
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(&DEPOSIT_KEY_06)
        .run();

    world.check_account(SC_ADDRESS)
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1,000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;
}
