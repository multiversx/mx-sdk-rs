// Auto-generated blackbox tests from scenarios

use multiversx_sc_scenario::imports::*;

use ping_pong_egld::*;

const PING_PONG_EGLD_CODE_PATH: MxscPath = MxscPath::new("output/ping-pong-egld.mxsc.json");
const MY_ADDRESS_ADDRESS: TestAddress = TestAddress::new("my_address");
const PARTICIPANT1_ADDRESS: TestAddress = TestAddress::new("participant1");
const PARTICIPANT2_ADDRESS: TestAddress = TestAddress::new("participant2");
const PING_PONG_ADDRESS: TestSCAddress = TestSCAddress::new("ping-pong");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());

    blockchain.set_current_dir_from_workspace("contracts/examples/ping-pong-egld");
    blockchain.register_contract(PING_PONG_EGLD_CODE_PATH, ping_pong_egld::ContractBuilder);
    blockchain
}

#[test]
fn ping_pong_call_get_user_addresses_scen() {
    let mut world = world();
    ping_pong_call_get_user_addresses_scen_steps(&mut world);
}

pub fn ping_pong_call_get_user_addresses_scen_steps(world: &mut ScenarioWorld) {
    ping_pong_call_ping_second_user_scen_steps(world);

    world
        .tx()
        .from(PARTICIPANT1_ADDRESS)
        .to(PING_PONG_ADDRESS)
        .typed(proxy::PingPongEgldProxy)
        .get_user_addresses()
        .returns(ExpectValue(MultiValueVec::from(vec![
            ScenarioValueRaw::new("address:participant1"),
            ScenarioValueRaw::new("address:participant2"),
        ])))
        .run();
}

#[test]
fn ping_pong_call_ping_after_deadline_scen() {
    let mut world = world();
    ping_pong_call_ping_after_deadline_scen_steps(&mut world);
}

pub fn ping_pong_call_ping_after_deadline_scen_steps(world: &mut ScenarioWorld) {
    ping_pong_init_scen_steps(world);

    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(123_781u64));

    world
        .tx()
        .from(PARTICIPANT1_ADDRESS)
        .to(PING_PONG_ADDRESS)
        .typed(proxy::PingPongEgldProxy)
        .ping(IgnoreValue)
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 500_000_000_000u64).unwrap())
        .with_result(ExpectError(4, "deadline has passed"))
        .run();

    world
        .check_account(PING_PONG_ADDRESS)
        .check_storage("str:activationTimestamp", "780")
        .check_storage("str:deadline", "123,780")
        .check_storage("str:pingAmount", "500,000,000,000");
}

#[test]
fn ping_pong_call_ping_before_activation_scen() {
    let mut world = world();
    ping_pong_call_ping_before_activation_scen_steps(&mut world);
}

pub fn ping_pong_call_ping_before_activation_scen_steps(world: &mut ScenarioWorld) {
    ping_pong_init_scen_steps(world);

    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(779u64));

    world
        .tx()
        .from(PARTICIPANT1_ADDRESS)
        .to(PING_PONG_ADDRESS)
        .typed(proxy::PingPongEgldProxy)
        .ping(IgnoreValue)
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 500_000_000_000u64).unwrap())
        .with_result(ExpectError(4, "smart contract not active yet"))
        .run();

    world
        .check_account(PING_PONG_ADDRESS)
        .check_storage("str:activationTimestamp", "780")
        .check_storage("str:deadline", "123,780")
        .check_storage("str:pingAmount", "500,000,000,000");
}

#[test]
fn ping_pong_call_ping_before_beginning_scen() {
    let mut world = world();
    ping_pong_call_ping_before_beginning_scen_steps(&mut world);
}

pub fn ping_pong_call_ping_before_beginning_scen_steps(world: &mut ScenarioWorld) {
    ping_pong_init_scen_steps(world);

    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(779u64));

    world
        .tx()
        .from(PARTICIPANT1_ADDRESS)
        .to(PING_PONG_ADDRESS)
        .typed(proxy::PingPongEgldProxy)
        .ping(IgnoreValue)
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 500_000_000_000u64).unwrap())
        .with_result(ExpectError(4, "smart contract not active yet"))
        .run();

    world
        .check_account(PING_PONG_ADDRESS)
        .check_storage("str:activationTimestamp", "780")
        .check_storage("str:deadline", "123,780")
        .check_storage("str:pingAmount", "500,000,000,000");
}

#[test]
fn ping_pong_call_ping_second_user_scen() {
    let mut world = world();
    ping_pong_call_ping_second_user_scen_steps(&mut world);
}

pub fn ping_pong_call_ping_second_user_scen_steps(world: &mut ScenarioWorld) {
    ping_pong_call_ping_scen_steps(world);

    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(781u64));

    world
        .tx()
        .from(PARTICIPANT2_ADDRESS)
        .to(PING_PONG_ADDRESS)
        .typed(proxy::PingPongEgldProxy)
        .ping(IgnoreValue)
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 500_000_000_000u64).unwrap())
        .run();

    world
        .check_account(PING_PONG_ADDRESS)
        .check_storage("str:activationTimestamp", "780")
        .check_storage("str:deadline", "123,780")
        .check_storage("str:pingAmount", "500,000,000,000")
        .check_storage("str:userStatus|0x0000001", "1")
        .check_storage("str:userStatus|0x0000002", "1")
        .check_storage("str:user_address_to_id|address:participant1", "1")
        .check_storage("str:user_address_to_id|address:participant2", "2")
        .check_storage("str:user_count", "2")
        .check_storage("str:user_id_to_address|0x0000001", "address:participant1")
        .check_storage("str:user_id_to_address|0x0000002", "address:participant2");
}

#[test]
fn ping_pong_call_ping_twice_scen() {
    let mut world = world();
    ping_pong_call_ping_twice_scen_steps(&mut world);
}

pub fn ping_pong_call_ping_twice_scen_steps(world: &mut ScenarioWorld) {
    ping_pong_call_ping_scen_steps(world);

    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(123_001u64));

    world
        .tx()
        .from(PARTICIPANT1_ADDRESS)
        .to(PING_PONG_ADDRESS)
        .typed(proxy::PingPongEgldProxy)
        .ping(IgnoreValue)
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 500_000_000_000u64).unwrap())
        .with_result(ExpectError(4, "can only ping once"))
        .run();

    world
        .check_account(PING_PONG_ADDRESS)
        .check_storage("str:activationTimestamp", "780")
        .check_storage("str:deadline", "123,780")
        .check_storage("str:pingAmount", "500,000,000,000")
        .check_storage("str:userStatus|0x0000001", "1")
        .check_storage("str:user_address_to_id|address:participant1", "1")
        .check_storage("str:user_count", "1")
        .check_storage("str:user_id_to_address|0x0000001", "address:participant1");
}

#[test]
fn ping_pong_call_ping_wrong_amount_scen() {
    let mut world = world();
    ping_pong_call_ping_wrong_amount_scen_steps(&mut world);
}

pub fn ping_pong_call_ping_wrong_amount_scen_steps(world: &mut ScenarioWorld) {
    ping_pong_init_scen_steps(world);

    world
        .tx()
        .from(PARTICIPANT1_ADDRESS)
        .to(PING_PONG_ADDRESS)
        .typed(proxy::PingPongEgldProxy)
        .ping(IgnoreValue)
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 450_000_000_000u64).unwrap())
        .with_result(ExpectError(4, "the payment must match the fixed sum"))
        .run();

    world
        .check_account(PING_PONG_ADDRESS)
        .check_storage("str:activationTimestamp", "780")
        .check_storage("str:deadline", "123,780")
        .check_storage("str:pingAmount", "500,000,000,000");
}

#[test]
fn ping_pong_call_ping_scen() {
    let mut world = world();
    ping_pong_call_ping_scen_steps(&mut world);
}

pub fn ping_pong_call_ping_scen_steps(world: &mut ScenarioWorld) {
    ping_pong_init_scen_steps(world);

    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(781u64));

    world
        .tx()
        .from(PARTICIPANT1_ADDRESS)
        .to(PING_PONG_ADDRESS)
        .typed(proxy::PingPongEgldProxy)
        .ping(IgnoreValue)
        .payment(Payment::try_new(TestTokenId::EGLD_000000, 0, 500_000_000_000u64).unwrap())
        .run();

    world
        .check_account(PING_PONG_ADDRESS)
        .check_storage("str:activationTimestamp", "780")
        .check_storage("str:deadline", "123,780")
        .check_storage("str:pingAmount", "500,000,000,000")
        .check_storage("str:userStatus|0x0000001", "1")
        .check_storage("str:user_address_to_id|address:participant1", "1")
        .check_storage("str:user_count", "1")
        .check_storage("str:user_id_to_address|0x0000001", "address:participant1");
}

#[test]
fn ping_pong_call_pong_all_after_pong_scen() {
    let mut world = world();
    ping_pong_call_pong_all_after_pong_scen_steps(&mut world);
}

pub fn ping_pong_call_pong_all_after_pong_scen_steps(world: &mut ScenarioWorld) {
    ping_pong_call_pong_scen_steps(world);

    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(123_781u64));

    world
        .tx()
        .from(PARTICIPANT1_ADDRESS)
        .to(PING_PONG_ADDRESS)
        .typed(proxy::PingPongEgldProxy)
        .pong_all()
        .returns(ExpectValue(ScenarioValueRaw::new("str:completed")))
        .run();

    world
        .check_account(PING_PONG_ADDRESS)
        .check_storage("str:activationTimestamp", "780")
        .check_storage("str:deadline", "123,780")
        .check_storage("str:pingAmount", "500,000,000,000")
        .check_storage("str:userStatus|0x0000001", "2")
        .check_storage("str:user_address_to_id|address:participant1", "1")
        .check_storage("str:user_count", "1")
        .check_storage("str:user_id_to_address|0x0000001", "address:participant1");
}

#[test]
fn ping_pong_call_pong_all_scen() {
    let mut world = world();
    ping_pong_call_pong_all_scen_steps(&mut world);
}

pub fn ping_pong_call_pong_all_scen_steps(world: &mut ScenarioWorld) {
    ping_pong_call_ping_second_user_scen_steps(world);

    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(123_781u64));

    ping_pong_call_pong_all_steps_steps(world);
}

pub fn ping_pong_call_pong_all_steps_steps(world: &mut ScenarioWorld) {
    world
        .tx()
        .id("pong-all-complete")
        .from(PARTICIPANT1_ADDRESS)
        .to(PING_PONG_ADDRESS)
        .typed(proxy::PingPongEgldProxy)
        .pong_all()
        .returns(ExpectValue(ScenarioValueRaw::new("str:completed")))
        .run();

    world
        .check_account(PING_PONG_ADDRESS)
        .check_storage("str:activationTimestamp", "780")
        .check_storage("str:deadline", "123,780")
        .check_storage("str:pingAmount", "500,000,000,000")
        .check_storage("str:userStatus|0x0000001", "2")
        .check_storage("str:userStatus|0x0000002", "2")
        .check_storage("str:user_address_to_id|address:participant1", "1")
        .check_storage("str:user_address_to_id|address:participant2", "2")
        .check_storage("str:user_count", "2")
        .check_storage("str:user_id_to_address|0x0000001", "address:participant1")
        .check_storage("str:user_id_to_address|0x0000002", "address:participant2");
}

#[test]
fn ping_pong_call_pong_before_deadline_scen() {
    let mut world = world();
    ping_pong_call_pong_before_deadline_scen_steps(&mut world);
}

pub fn ping_pong_call_pong_before_deadline_scen_steps(world: &mut ScenarioWorld) {
    ping_pong_call_ping_scen_steps(world);

    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(123_000u64));

    world
        .tx()
        .from(PARTICIPANT1_ADDRESS)
        .to(PING_PONG_ADDRESS)
        .typed(proxy::PingPongEgldProxy)
        .pong()
        .with_result(ExpectError(4, "can\'t withdraw before deadline"))
        .run();

    world
        .check_account(PING_PONG_ADDRESS)
        .check_storage("str:activationTimestamp", "780")
        .check_storage("str:deadline", "123,780")
        .check_storage("str:pingAmount", "500,000,000,000")
        .check_storage("str:userStatus|0x0000001", "1")
        .check_storage("str:user_address_to_id|address:participant1", "1")
        .check_storage("str:user_count", "1")
        .check_storage("str:user_id_to_address|0x0000001", "address:participant1");
}

#[test]
fn ping_pong_call_pong_twice_scen() {
    let mut world = world();
    ping_pong_call_pong_twice_scen_steps(&mut world);
}

pub fn ping_pong_call_pong_twice_scen_steps(world: &mut ScenarioWorld) {
    ping_pong_call_pong_scen_steps(world);

    world
        .tx()
        .from(PARTICIPANT1_ADDRESS)
        .to(PING_PONG_ADDRESS)
        .typed(proxy::PingPongEgldProxy)
        .pong()
        .with_result(ExpectError(4, "already withdrawn"))
        .run();

    world
        .check_account(PING_PONG_ADDRESS)
        .check_storage("str:activationTimestamp", "780")
        .check_storage("str:deadline", "123,780")
        .check_storage("str:pingAmount", "500,000,000,000")
        .check_storage("str:userStatus|0x0000001", "2")
        .check_storage("str:user_address_to_id|address:participant1", "1")
        .check_storage("str:user_count", "1")
        .check_storage("str:user_id_to_address|0x0000001", "address:participant1");
}

#[test]
fn ping_pong_call_pong_without_ping_scen() {
    let mut world = world();
    ping_pong_call_pong_without_ping_scen_steps(&mut world);
}

pub fn ping_pong_call_pong_without_ping_scen_steps(world: &mut ScenarioWorld) {
    ping_pong_init_scen_steps(world);

    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(123_781u64));

    world
        .tx()
        .from(PARTICIPANT1_ADDRESS)
        .to(PING_PONG_ADDRESS)
        .typed(proxy::PingPongEgldProxy)
        .pong()
        .with_result(ExpectError(4, "can\'t pong, never pinged"))
        .run();

    world
        .check_account(PING_PONG_ADDRESS)
        .check_storage("str:activationTimestamp", "780")
        .check_storage("str:deadline", "123,780")
        .check_storage("str:pingAmount", "500,000,000,000");
}

#[test]
fn ping_pong_call_pong_scen() {
    let mut world = world();
    ping_pong_call_pong_scen_steps(&mut world);
}

pub fn ping_pong_call_pong_scen_steps(world: &mut ScenarioWorld) {
    ping_pong_call_ping_scen_steps(world);

    world
        .current_block()
        .block_timestamp_millis(TimestampMillis::new(123_781u64));

    world
        .tx()
        .from(PARTICIPANT1_ADDRESS)
        .to(PING_PONG_ADDRESS)
        .typed(proxy::PingPongEgldProxy)
        .pong()
        .run();

    world
        .check_account(PING_PONG_ADDRESS)
        .check_storage("str:activationTimestamp", "780")
        .check_storage("str:deadline", "123,780")
        .check_storage("str:pingAmount", "500,000,000,000")
        .check_storage("str:userStatus|0x0000001", "2")
        .check_storage("str:user_address_to_id|address:participant1", "1")
        .check_storage("str:user_count", "1")
        .check_storage("str:user_id_to_address|0x0000001", "address:participant1");
}

#[test]
fn ping_pong_init_scen() {
    let mut world = world();
    ping_pong_init_scen_steps(&mut world);
}

pub fn ping_pong_init_scen_steps(world: &mut ScenarioWorld) {
    world
        .account(MY_ADDRESS_ADDRESS)
        .nonce(0u64)
        .balance(1_000_000u64);
    world
        .account(PARTICIPANT1_ADDRESS)
        .nonce(0u64)
        .balance(1_800_000_000_000u64);
    world
        .account(PARTICIPANT2_ADDRESS)
        .nonce(0u64)
        .balance(2_500_000_000_000u64);

    world
        .tx()
        .id("deploy")
        .from(MY_ADDRESS_ADDRESS)
        .typed(proxy::PingPongEgldProxy)
        .init(
            500_000_000_000u64,
            DurationMillis::new(123_000u64),
            ScenarioValueRaw::new("1|u64:780"),
            IgnoreValue,
        )
        .code(PING_PONG_EGLD_CODE_PATH)
        .new_address(PING_PONG_ADDRESS)
        .run();

    world
        .check_account(PING_PONG_ADDRESS)
        .check_storage("str:activationTimestamp", "780")
        .check_storage("str:deadline", "123,780")
        .check_storage("str:pingAmount", "500,000,000,000");
}
