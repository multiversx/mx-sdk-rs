// Auto-generated blackbox tests from scenarios

use multiversx_sc_scenario::imports::*;

use digital_cash::*;

const CODE_PATH: MxscPath = MxscPath::new("output/digital_cash.mxsc.json");

fn world() -> ScenarioWorld {
    todo!()
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
        .from(TestAddress::new("acc3"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .pay_fee_and_fund(
            ScenarioValueRaw::str(
                "0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60",
            ),
            ScenarioValueRaw::str("86400_000"),
        )
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:CASHTOKEN-778899", "3")
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc3|u32:1|nested:str:CASHTOKEN-112233|u64:0|biguint:50|u64:86400_000|0x01|nested:str:CASHTOKEN-778899|biguint:3")
        ;
}

#[test]
fn forward_scen() {
    let mut world = world();
    forward_scen_steps(&mut world);
}

pub fn forward_scen_steps(world: &mut ScenarioWorld) {
    fund_egld_and_esdt_scen_steps(world);

    world.tx().id("forward-fail")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .forward(ScenarioValueRaw::str("0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60"), ScenarioValueRaw::str("0xa40e72cdac3580e7203a4c2565c932f7691c35e624bcfd82718d7f559c88f440"), ScenarioValueRaw::str("0x443c75ceadb9ec42acff7e1b92e0305182279446c1d6c0502959484c147a0430d3f96f0b988e646f6736d5bf8e4a843d8ba7730d6fa7e60f0ef3edd225ce630f"))
        .run();

    world
        .tx()
        .id("deposit-fees-2")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .deposit_fees(ScenarioValueRaw::str(
            "0xa40e72cdac3580e7203a4c2565c932f7691c35e624bcfd82718d7f559c88f440",
        ))
        .run();

    world.tx().id("forward-without-fees-ok")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .forward(ScenarioValueRaw::str("0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60"), ScenarioValueRaw::str("0xa40e72cdac3580e7203a4c2565c932f7691c35e624bcfd82718d7f559c88f440"), ScenarioValueRaw::str("0x443c75ceadb9ec42acff7e1b92e0305182279446c1d6c0502959484c147a0430d3f96f0b988e646f6736d5bf8e4a843d8ba7730d6fa7e60f0ef3edd225ce630f"))
        .run();

    world
        .tx()
        .id("deposit-fees-4")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .deposit_fees(ScenarioValueRaw::str(
            "0x8dc17613990e9b7476401a36d112d1a4d31190dec21e7e9a3c933872a27613ee",
        ))
        .run();

    world.tx().id("forward-with-fees-fail")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .forward(ScenarioValueRaw::str("0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d"), ScenarioValueRaw::str("0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d"), ScenarioValueRaw::str("0x1ac4f6d4d45836d97ffeda83a66aaea7631a3bb3d4063421ccb2b9de9485bdb4c9bd6e44e003f6a9c9eb74379467238204ff579471d203b1878c3f1530592a02"))
        .run();

    world.tx().id("forward-with-fees-ok")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .forward(ScenarioValueRaw::str("0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d"), ScenarioValueRaw::str("0x8dc17613990e9b7476401a36d112d1a4d31190dec21e7e9a3c933872a27613ee"), ScenarioValueRaw::str("0x1ac4f6d4d45836d97ffeda83a66aaea7631a3bb3d4063421ccb2b9de9485bdb4c9bd6e44e003f6a9c9eb74379467238204ff579471d203b1878c3f1530592a02"))
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:collectedFees", "nested:str:EGLD-000000|biguint:40")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0x8dc17613990e9b7476401a36d112d1a4d31190dec21e7e9a3c933872a27613ee", "address:acc2|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1000")
        .check_storage("str:deposit|0xa40e72cdac3580e7203a4c2565c932f7691c35e624bcfd82718d7f559c88f440", "address:acc2|u32:1|nested:str:EGLD-000000|u64:0|biguint:1,000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
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
        .from(TestAddress::new("acc3"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .pay_fee_and_fund(
            ScenarioValueRaw::str(
                "0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60",
            ),
            ScenarioValueRaw::str("86400_000"),
        )
        .run();

    world
        .tx()
        .id("pay-fee-and-fund-esdt-success")
        .from(TestAddress::new("acc3"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .pay_fee_and_fund(
            ScenarioValueRaw::str(
                "0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60",
            ),
            ScenarioValueRaw::str("86400_000"),
        )
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:CASHTOKEN-778899", "3")
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc3|u32:2|nested:str:CASHTOKEN-778899|u64:0|biguint:44|nested:str:CASHTOKEN-112233|u64:0|biguint:50|u64:86400_000|0x01|nested:str:CASHTOKEN-778899|biguint:6")
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
        .from(TestAddress::new("acc1"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .fund(
            ScenarioValueRaw::str(
                "0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60",
            ),
            ScenarioValueRaw::str("86400_000"),
        )
        .run();

    world
        .tx()
        .id("deposit-fees-1")
        .from(TestAddress::new("acc1"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .deposit_fees(ScenarioValueRaw::str(
            "0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60",
        ))
        .run();

    world
        .tx()
        .id("fund-1")
        .from(TestAddress::new("acc1"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .fund(
            ScenarioValueRaw::str(
                "0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60",
            ),
            ScenarioValueRaw::str("86399_000"),
        )
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;

    world
        .tx()
        .id("deposit-fees-2")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .deposit_fees(ScenarioValueRaw::str(
            "0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd",
        ))
        .run();

    world
        .tx()
        .id("fund-2")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .fund(
            ScenarioValueRaw::str(
                "0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd",
            ),
            ScenarioValueRaw::str("86400_000"),
        )
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;

    world
        .tx()
        .id("fund-fail-3")
        .from(TestAddress::new("acc3"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .fund(
            ScenarioValueRaw::str(
                "0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd",
            ),
            ScenarioValueRaw::str("86400_000"),
        )
        .run();

    world
        .tx()
        .id("deposit-fees-3")
        .from(TestAddress::new("acc3"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .deposit_fees(ScenarioValueRaw::str(
            "0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d",
        ))
        .run();

    world
        .tx()
        .id("fund-2")
        .from(TestAddress::new("acc3"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .fund(
            ScenarioValueRaw::str(
                "0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d",
            ),
            ScenarioValueRaw::str("86400_000"),
        )
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
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

    world.tx().id("claim2")
        .from(TestAddress::new("acc1"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(ScenarioValueRaw::str("0x287bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd"), ScenarioValueRaw::str("0xdd092ec3a8d971daede79da4e5c5c90d66af9f2209a6f6541affa00c46a72fc2596e4db1b1bb226ce76e50730733078ff74a79ff7d0d185054375e0989330600"))
        .run();

    // set block

    world.tx().id("claim3-esdt")
        .from(TestAddress::new("acc1"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(ScenarioValueRaw::str("0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd"), ScenarioValueRaw::str("0xdd092ec3a8d971daede79da4e5c5c90d66af9f2209a6f6541affa00c46a72fc2596e4db1b1bb226ce76e50730733078ff74a79ff7d0d185054375e0989330600"))
        .run();

    // set block

    world.tx().id("claim4")
        .from(TestAddress::new("acc1"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(ScenarioValueRaw::str("0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd"), ScenarioValueRaw::str("0x1dd092ec3a8d971daede79da4e5c5c90d66af9f2209a6f6541affa00c46a72fc2596e4db1b1bb226ce76e50730733078ff74a79ff7d0d185054375e0989330600"))
        .run();

    world.tx().id("claim5-esdt")
        .from(TestAddress::new("acc1"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(ScenarioValueRaw::str("0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd"), ScenarioValueRaw::str("0xdd092ec3a8d971daede79da4e5c5c90d66af9f2209a6f6541affa00c46a72fc2596e4db1b1bb226ce76e50730733078ff74a79ff7d0d185054375e0989330600"))
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:collectedFees", "nested:str:EGLD-000000|biguint:10")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1,000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;
}

#[test]
fn claim_egld_scen() {
    let mut world = world();
    claim_egld_scen_steps(&mut world);
}

pub fn claim_egld_scen_steps(world: &mut ScenarioWorld) {
    fund_egld_and_esdt_scen_steps(world);

    world.tx().id("claim2-fail")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(ScenarioValueRaw::str("0xd0474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60"), ScenarioValueRaw::str("0x443c75ceadb9ec42acff7e1b92e0305182279446c1d6c0502959484c147a0430d3f96f0b988e646f6736d5bf8e4a843d8ba7730d6fa7e60f0ef3edd225ce630f"))
        .run();

    // set block

    world.tx().id("claim3-egld-fail-expired")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(ScenarioValueRaw::str("0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60"), ScenarioValueRaw::str("0x443c75ceadb9ec42acff7e1b92e0305182279446c1d6c0502959484c147a0430d3f96f0b988e646f6736d5bf8e4a843d8ba7730d6fa7e60f0ef3edd225ce630f"))
        .run();

    // set block

    world.tx().id("claim4-fail")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(ScenarioValueRaw::str("0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60"), ScenarioValueRaw::str("0x12bb9e58dad361e9dadd0af1021ce53f9ca12b6580f5b3ab4f9c321ee055a38bcdcf35924eb46aef7a80b22387ded0b837734ac8a57e19ea12c33ef808f996c00"))
        .run();

    world.tx().id("claim5-egld")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(ScenarioValueRaw::str("0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60"), ScenarioValueRaw::str("0x443c75ceadb9ec42acff7e1b92e0305182279446c1d6c0502959484c147a0430d3f96f0b988e646f6736d5bf8e4a843d8ba7730d6fa7e60f0ef3edd225ce630f"))
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:collectedFees", "nested:str:EGLD-000000|biguint:10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
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
        .from(TestAddress::new("acc3"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .pay_fee_and_fund(
            ScenarioValueRaw::str(
                "0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60",
            ),
            ScenarioValueRaw::str("86400_000"),
        )
        .run();

    world
        .tx()
        .id("pay-fee-and-fund-egld-success")
        .from(TestAddress::new("acc3"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .pay_fee_and_fund(
            ScenarioValueRaw::str(
                "0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60",
            ),
            ScenarioValueRaw::str("86400_000"),
        )
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:CASHTOKEN-778899", "3")
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc3|u32:1|nested:str:EGLD-000000|u64:0|biguint:990|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:10")
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
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim_fees()
        .run();

    // set block

    world
        .tx()
        .id("claim-fees-ok")
        .from(TestAddress::new("digital_cash_owner_address"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim_fees()
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
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

    world.tx().id("claim2")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(ScenarioValueRaw::str("0x805532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d"), ScenarioValueRaw::str("0x1ac4f6d4d45836d97ffeda83a66aaea7631a3bb3d4063421ccb2b9de9485bdb4c9bd6e44e003f6a9c9eb74379467238204ff579471d203b1878c3f1530592a02"))
        .run();

    // set block

    world.tx().id("claim3-multi")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(ScenarioValueRaw::str("0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d"), ScenarioValueRaw::str("0x1ac4f6d4d45836d97ffeda83a66aaea7631a3bb3d4063421ccb2b9de9485bdb4c9bd6e44e003f6a9c9eb74379467238204ff579471d203b1878c3f1530592a02"))
        .run();

    // set block

    world.tx().id("claim4")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(ScenarioValueRaw::str("0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d"), ScenarioValueRaw::str("0x11ac4f6d4d45836d97ffeda83a66aaea7631a3bb3d4063421ccb2b9de9485bdb4c9bd6e44e003f6a9c9eb74379467238204ff579471d203b1878c3f1530592a02"))
        .run();

    world.tx().id("claim5-multi")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .claim(ScenarioValueRaw::str("0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d"), ScenarioValueRaw::str("0x1ac4f6d4d45836d97ffeda83a66aaea7631a3bb3d4063421ccb2b9de9485bdb4c9bd6e44e003f6a9c9eb74379467238204ff579471d203b1878c3f1530592a02"))
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:collectedFees", "nested:str:EGLD-000000|biguint:30")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1,000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;
}

#[test]
fn set_accounts_scen() {
    let mut world = world();
    set_accounts_scen_steps(&mut world);
}

pub fn set_accounts_scen_steps(world: &mut ScenarioWorld) {
    world
        .account(TestAddress::new("acc1"))
        .nonce(ScenarioValueRaw::str("0"))
        .balance(ScenarioValueRaw::str("1,000,000"));
    world
        .account(TestAddress::new("acc2"))
        .nonce(ScenarioValueRaw::str("0"))
        .balance(ScenarioValueRaw::str("1,000,000"));
    world
        .account(TestAddress::new("acc3"))
        .nonce(ScenarioValueRaw::str("0"))
        .balance(ScenarioValueRaw::str("1,000,000"));
    world
        .account(TestAddress::new("digital_cash_owner_address"))
        .nonce(ScenarioValueRaw::str("0"))
        .balance(ScenarioValueRaw::str("0"));

    world
        .tx()
        .id("deploy")
        .from(TestAddress::new("digital_cash_owner_address"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .init(
            ScenarioValueRaw::str("false"),
            ScenarioValueRaw::str("str:EGLD-000000"), /* , ScenarioValueRaw::str("10") */
        )
        .code(CODE_PATH)
        .new_address(TestSCAddress::new("the_digital_cash_contract"))
        .run();

    world
        .check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:feesDisabled", "false");
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
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(ScenarioValueRaw::str(
            "0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd",
        ))
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1,000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;

    world
        .tx()
        .id("withdraw-esdt-fail-2")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(ScenarioValueRaw::str(
            "0xe808c2baab2a20b612f1351da5945c52c60f5321c6cde572149db90c9e8fbfc7",
        ))
        .run();

    // set block

    world
        .tx()
        .id("withdraw-esdt-success")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(ScenarioValueRaw::str(
            "0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd",
        ))
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1,000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;
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
        .from(TestAddress::new("acc1"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(ScenarioValueRaw::str(
            "0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60",
        ))
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1,000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;

    world
        .tx()
        .id("withdraw-egld-fail-2")
        .from(TestAddress::new("acc1"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(ScenarioValueRaw::str(
            "0x558fd9b0dd9fed2d3bed883d3b92907743362c56b9728392f84b261f1cc5ae0a",
        ))
        .run();

    // set block

    world
        .tx()
        .id("withdraw-egld-success")
        .from(TestAddress::new("acc1"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(ScenarioValueRaw::str(
            "0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60",
        ))
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
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
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(ScenarioValueRaw::str(
            "0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd",
        ))
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d", "address:acc3|u32:3|nested:str:CASHTOKEN-112233|u64:0|biguint:50|nested:str:CASHTOKEN-445566|u64:0|biguint:50|nested:str:CASHTOKEN-778899|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1,000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;

    world
        .tx()
        .id("withdraw-esdt-2")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(ScenarioValueRaw::str(
            "0x805532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d",
        ))
        .run();

    // set block

    world
        .tx()
        .id("withdraw-esdt-3")
        .from(TestAddress::new("acc2"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .withdraw_expired(ScenarioValueRaw::str(
            "0x885532043a061e0c779e4064b85193f72cffd22c5bcc208c209128e60f21bf0d",
        ))
        .run();

    world.check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10")
        .check_storage("str:deposit|0x487bd4010b50c24a02018345fe5171edf4182e6294325382c75ef4c4409f01bd", "address:acc2|u32:1|nested:str:CASHTOKEN-123456|u64:0|biguint:50|u64:86400_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:deposit|0xdb474a3a065d3f0c0a62ae680ef6435e48eb482899d2ae30ff7a3a4b0ef19c60", "address:acc1|u32:1|nested:str:EGLD-000000|u64:0|biguint:1,000|u64:86399_000|0x01|nested:str:EGLD-000000|biguint:1,000")
        .check_storage("str:feesDisabled", "false")
        ;
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
        .from(TestAddress::new("digital_cash_owner_address"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .set_fee(
            ScenarioValueRaw::str("str:CASHTOKEN-778899"),
            ScenarioValueRaw::str("3"),
        )
        .run();

    world
        .tx()
        .id("whitelist-success-2")
        .from(TestAddress::new("digital_cash_owner_address"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .set_fee(
            ScenarioValueRaw::str("str:ESDT-778899"),
            ScenarioValueRaw::str("5"),
        )
        .run();

    world
        .tx()
        .id("blacklist-success")
        .from(TestAddress::new("digital_cash_owner_address"))
        .to(TestSCAddress::new("the_digital_cash_contract"))
        .typed(digital_cash_proxy::DigitalCashProxy)
        .set_fee(
            ScenarioValueRaw::str("str:ESDT-778899"),
            ScenarioValueRaw::str("0"),
        )
        .run();

    world
        .check_account(TestSCAddress::new("the_digital_cash_contract"))
        .check_storage("str:baseFee|nested:str:CASHTOKEN-778899", "3")
        .check_storage("str:baseFee|nested:str:EGLD-000000", "10");
}
