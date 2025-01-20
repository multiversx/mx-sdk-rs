use adder_proxy::{SovereignConfig, StakeArgs};
use multiversx_sc::imports::*;
use multiversx_sc_scenario::imports::*;

use adder::*;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const ADDER_ADDRESS: TestSCAddress = TestSCAddress::new("adder");
const OTHER_ADDRESS: TestSCAddress = TestSCAddress::new("other");

const CODE_PATH: MxscPath = MxscPath::new("output/adder.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/examples/adder");
    blockchain.register_contract(CODE_PATH, adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_blackbox_sovereign_config_test() {
    let mut world = world();

    let stake_args = StakeArgs::<StaticApi> {
        token_id: TokenIdentifier::from_esdt_bytes(b"TESTTOKEN"),
        amount: BigUint::from(2u64),
    };

    let config = SovereignConfig::<StaticApi> {
        min_validators: 1u64,
        max_validators: 18u64,
        min_stake: BigUint::from(100u64),
        opt_additional_stake_required: Some(ManagedVec::from_single_item(stake_args)),
    };

    world.account(OWNER_ADDRESS).nonce(1);

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(adder_proxy::AdderProxy)
        .init(5u32)
        .code(CODE_PATH)
        .new_address(ADDER_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, ADDER_ADDRESS.to_address());

    let other_contract = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(adder_proxy::AdderProxy)
        .init(5u32)
        .code(CODE_PATH)
        .new_address(OTHER_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(&new_address)
        .typed(adder_proxy::AdderProxy)
        .set_storage(config)
        .run();

    let config_from_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .to(&other_contract)
        .typed(adder_proxy::AdderProxy)
        .get_storage_from_address(new_address)
        .returns(ReturnsResult)
        .run();

    println!("{:?}", config_from_address);
}

#[test]
fn adder_blackbox() {
    let mut world = world();

    world.start_trace();

    world.account(OWNER_ADDRESS).nonce(1);

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(adder_proxy::AdderProxy)
        .init(5u32)
        .code(CODE_PATH)
        .new_address(ADDER_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, ADDER_ADDRESS.to_address());

    world
        .query()
        .to(ADDER_ADDRESS)
        .typed(adder_proxy::AdderProxy)
        .sum()
        .returns(ExpectValue(5u32))
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(ADDER_ADDRESS)
        .typed(adder_proxy::AdderProxy)
        .add(1u32)
        .run();

    world
        .query()
        .to(ADDER_ADDRESS)
        .typed(adder_proxy::AdderProxy)
        .sum()
        .returns(ExpectValue(6u32))
        .run();

    world.check_account(OWNER_ADDRESS);

    world
        .check_account(ADDER_ADDRESS)
        .check_storage("str:sum", "6");

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(ADDER_ADDRESS)
        .typed(adder_proxy::AdderProxy)
        .upgrade(100u64)
        .code(CODE_PATH)
        .run();

    world
        .check_account(ADDER_ADDRESS)
        .check_storage("str:sum", "100");

    world.write_scenario_trace("trace1.scen.json");
}

#[test]
fn adder_blackbox_get_esdt_token_data_test() {
    let mut world = world();

    world.start_trace();

    world
        .account(OWNER_ADDRESS)
        .nonce(1)
        .esdt_nft_all_properties(
            TokenIdentifier::from_esdt_bytes("TESTTOKEN"),
            0u64,
            100_000_000u64,
            ManagedBuffer::from(b"Attributes1"),
            0u64,
            Some(OWNER_ADDRESS),
            ManagedBuffer::new(),
            Vec::<ManagedBuffer<StaticApi>>::new(),
        );

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(adder_proxy::AdderProxy)
        .init(5u32)
        .code(CODE_PATH)
        .new_address(ADDER_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, ADDER_ADDRESS.to_address());

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(&new_address)
        .raw_call("")
        .single_esdt(
            &TokenIdentifier::from_esdt_bytes(b"TESTTOKEN"),
            0u64,
            &BigUint::from(100_000_000u64),
        )
        .run();

    let token_data = world
        .tx()
        .from(OWNER_ADDRESS)
        .to(&new_address)
        .typed(adder_proxy::AdderProxy)
        .test_endpoint()
        .returns(ReturnsResult)
        .run();

    println!("First call token data {:?}\n", token_data);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(&new_address)
        .typed(adder_proxy::AdderProxy)
        .send_back_token()
        .run();

    world
        .check_account(&new_address)
        .esdt_balance(TokenIdentifier::from_esdt_bytes(b"TESTTOKEN"), 0u64);

    let token_data = world
        .tx()
        .from(OWNER_ADDRESS)
        .to(&new_address)
        .typed(adder_proxy::AdderProxy)
        .test_endpoint()
        .returns(ReturnsResult)
        .run();

    println!("Second call token data {:?}", token_data);
}
