use multiversx_sc_scenario::imports::*;

mod contract_setup;
use contract_setup::*;

#[test]
fn lottery_init() {
    let mut state = LotteryScTestState::new();

    state.deploy();

    state.check_state(
        CheckStateStep::new()
            .put_account(
                address(MY_ADDRESS),
                CheckAccount::new().nonce(2).balance("1,000,000"),
            )
            .put_account(
                address(OTHER_SHARD_ADDRESS),
                CheckAccount::new().nonce(0).balance("1,000,000"),
            )
            .put_account(
                address(ACCOUNT1_ADDRESS),
                CheckAccount::new().nonce(0).balance("1,000,000"),
            )
            .put_account(
                address(ACCOUNT2_ADDRESS),
                CheckAccount::new().nonce(0).balance("1,000,000"),
            )
            .put_account(sc_address(SC_LOTTERY_ADDRESS), CheckAccount::new().nonce(0)),
    );
}

#[test]
fn start_limited_tickets_and_fixes_deadline() {
    let mut state = LotteryScTestState::new();

    state.deploy();

    state.create_lottery_poll(
        None,
        "lottery_name".to_owned(),
        TestTokenIdentifier::new("LOTTERY-123456"),
        rust_biguint!(100),
        Some(2),
        Some(123_456u64),
        None,
        None,
        None,
        OptionalValue::None,
    );
}
