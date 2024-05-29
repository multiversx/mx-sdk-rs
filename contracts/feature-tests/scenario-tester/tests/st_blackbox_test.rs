use multiversx_sc_scenario::imports::*;

use scenario_tester::*;

const SC_SCENARIO_TESTER_PATH_EXPR: &str = "mxsc:output/scenario-tester.mxsc.json";
const FOURTH_ATTRIBUTES: &[u8] = b"FourthhAttributes";
const FOURTH_URIS: &[&[u8]] = &[b"FirstUri", b"SecondUri"];

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const OTHER_ADDRESS: TestAddress = TestAddress::new("other");
const ST_ADDRESS: TestSCAddress = TestSCAddress::new("scenario-tester");
const CODE_PATH: MxscPath = MxscPath::new("output/scenario-tester.mxsc.json");
const TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("TOKEN-123456");
const NFT_ID: TestTokenIdentifier = TestTokenIdentifier::new("NFT-123456");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        SC_SCENARIO_TESTER_PATH_EXPR,
        scenario_tester::ContractBuilder,
    );
    blockchain
}

#[test]
fn st_blackbox() {
    let mut world = world();

    world.start_trace();

    world
        .account(OWNER_ADDRESS)
        .nonce(1)
        .balance(100)
        .account(OTHER_ADDRESS)
        .nonce(2)
        .balance(300)
        .esdt_balance(TOKEN_ID, 500)
        .commit();

    world
        .check_account(OWNER_ADDRESS)
        .nonce(1)
        .balance(100)
        .check_account(OTHER_ADDRESS)
        .nonce(2)
        .balance(300)
        .esdt_balance(TOKEN_ID, 500)
        .commit();

    world.new_address(OWNER_ADDRESS, 1, ST_ADDRESS);

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .init(5u32)
        .code(CODE_PATH)
        .returns(ReturnsNewAddress)
        .run();
    assert_eq!(new_address, ST_ADDRESS.to_address());

    let value = world
        .query()
        .to(ST_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .sum()
        .returns(ReturnsResultUnmanaged)
        .run();
    assert_eq!(value, RustBigUint::from(5u32));

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(ST_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .add(1u32)
        .run();

    world
        .check_account(OWNER_ADDRESS)
        .nonce(3)
        .balance(100)
        .check_account(ST_ADDRESS)
        .check_storage("str:sum", "6")
        .commit();

    world
        .tx()
        .from(OTHER_ADDRESS)
        .to(ST_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .add(1u32)
        .run();

    world
        .tx()
        .from(OTHER_ADDRESS)
        .to(ST_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .multi_param(MultiValue2((1u32, 1u16)))
        .run();

    world
        .tx()
        .from(OTHER_ADDRESS)
        .to(ST_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .multi_return(1u32)
        .returns(ExpectValue(MultiValue2((1u32, 2u32))))
        .run();

    let value = world
        .tx()
        .from(OTHER_ADDRESS)
        .to(ST_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .multi_return(1u32)
        .returns(ReturnsResultUnmanaged)
        .run();
    assert_eq!(
        value,
        MultiValue2((RustBigUint::from(1u32), RustBigUint::from(2u32)))
    );

    world.write_scenario_trace("trace1.scen.json");
}

#[test]
fn set_state_test() {
    let mut world = world();
    let first = TestAddress::new("first");
    let second = TestAddress::new("second");
    let third = TestAddress::new("third");
    let fourth = TestAddress::new("fourth");
    let fifth = TestAddress::new("fifth");
    let sixth = TestAddress::new("sixth");
    let seventh = TestAddress::new("seventh");
    let eighth = TestAddress::new("eighth");

    world.start_trace();

    world
        .account(first)
        .nonce(1)
        .balance(100)
        .account(second)
        .nonce(2)
        .balance(300)
        .esdt_balance(TOKEN_ID, 500)
        .commit();

    world
        .check_account(first)
        .nonce(1)
        .balance(100)
        .check_account(second)
        .nonce(2)
        .balance(300)
        .esdt_balance(TOKEN_ID, 500)
        .commit();

    world
        .account(third)
        .nonce(3)
        .balance(50)
        .esdt_nft_balance(NFT_ID, 2, 1, ())
        .commit();

    world
        .check_account(third)
        .nonce(3)
        .balance(50)
        .esdt_nft_balance_and_attributes(NFT_ID, 2, 1, "")
        .commit();

    let fourth_uris = FOURTH_URIS
        .iter()
        .map(|first_uri| managed_buffer!(first_uri))
        .collect();
    world
        .account(fourth)
        .nonce(3)
        .balance(50)
        .esdt_nft_all_properties(
            NFT_ID,
            2,
            1,
            managed_buffer!(FOURTH_ATTRIBUTES),
            1000,
            None::<Address>,
            (),
            fourth_uris,
        )
        .commit();

    world
        .check_account(fourth)
        .nonce(3)
        .balance(50)
        .esdt_nft_balance_and_attributes(NFT_ID, 2, 1, FOURTH_ATTRIBUTES)
        .commit();

    world
        .account(fifth)
        .nonce(2)
        .balance(30)
        .esdt_nft_last_nonce(NFT_ID, 5);
    world
        .check_account(fifth)
        .nonce(2)
        .balance(30)
        .esdt_nft_balance_and_attributes(NFT_ID, 5, 0, "");

    // using no commit should drop the value naturally
    world
        .account(sixth)
        .nonce(4)
        .balance(400)
        .account(seventh)
        .nonce(5)
        .balance(250)
        .esdt_balance(TOKEN_ID, 2);

    world
        .check_account(sixth)
        .nonce(4)
        .balance(400)
        .check_account(seventh)
        .nonce(5)
        .balance(250)
        .esdt_balance(TOKEN_ID, 2);

    world
        .account(eighth)
        .nonce(6)
        .balance(600)
        .esdt_balance(TOKEN_ID, 60);

    world
        .check_account(eighth)
        .nonce(6)
        .balance(600)
        .esdt_balance(TOKEN_ID, 60);
}
