// tests/greeter_blackbox_test.rs — a hand-written blackbox test for the
// endpoint added on top of the bare `empty` scaffold. The two
// `greeter_scenario_*_test.rs` files (unmodified from `sc-meta new
// --template empty`) only exercise deploy; this test exercises the
// actual custom logic, following CLAUDE.md §"Blackbox Tests
// (RECOMMENDED)" pattern exactly.

use multiversx_sc_scenario::imports::*;

use greeter::greeter_proxy;

const OWNER: TestAddress = TestAddress::new("owner");
const CONTRACT: TestSCAddress = TestSCAddress::new("greeter-contract");
const CODE_PATH: MxscPath = MxscPath::new("output/greeter.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(CODE_PATH, greeter::ContractBuilder);
    blockchain
}

#[test]
fn set_and_get_greeting() {
    let mut world = world();
    world.account(OWNER).nonce(1);

    // Deploy.
    world
        .tx()
        .from(OWNER)
        .typed(greeter_proxy::GreeterProxy)
        .init()
        .code(CODE_PATH)
        .new_address(CONTRACT)
        .run();

    // Before any call, the greeting for OWNER is empty.
    world
        .query()
        .to(CONTRACT)
        .typed(greeter_proxy::GreeterProxy)
        .greeting(OWNER.to_address())
        .returns(ExpectValue(ManagedBuffer::<StaticApi>::new()))
        .run();

    // Call setGreeting as OWNER.
    world
        .tx()
        .from(OWNER)
        .to(CONTRACT)
        .typed(greeter_proxy::GreeterProxy)
        .set_greeting(ManagedBuffer::<StaticApi>::from(b"hello devnet"))
        .run();

    // getGreeting now returns what we stored, keyed by the caller address.
    world
        .query()
        .to(CONTRACT)
        .typed(greeter_proxy::GreeterProxy)
        .greeting(OWNER.to_address())
        .returns(ExpectValue(ManagedBuffer::<StaticApi>::from(
            b"hello devnet",
        )))
        .run();
}

#[test]
fn greeting_is_keyed_per_caller() {
    let mut world = world();
    world.account(OWNER).nonce(1);
    let other: TestAddress = TestAddress::new("other-caller");
    world.account(other).nonce(1);

    world
        .tx()
        .from(OWNER)
        .typed(greeter_proxy::GreeterProxy)
        .init()
        .code(CODE_PATH)
        .new_address(CONTRACT)
        .run();

    world
        .tx()
        .from(OWNER)
        .to(CONTRACT)
        .typed(greeter_proxy::GreeterProxy)
        .set_greeting("from owner")
        .run();

    // A different caller who never called setGreeting still reads back
    // empty — this is what CLAUDE.md §"Storage Mappers" means by "the key
    // includes the parameter automatically": each address gets its own
    // storage slot, not a shared one.
    world
        .query()
        .to(CONTRACT)
        .typed(greeter_proxy::GreeterProxy)
        .greeting(other.to_address())
        .returns(ExpectValue(""))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(greeter_proxy::GreeterProxy)
        .greeting(OWNER.to_address())
        .returns(ExpectValue("from owner"))
        .run();
}
