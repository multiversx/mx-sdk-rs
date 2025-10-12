use benchmark_common::ExampleStruct;
use linked_list_repeat::linked_list_repeat_proxy;
use multiversx_sc_scenario::imports::*;

const WASM_PATH: MxscPath = MxscPath::new("output/linked-list-repeat.mxsc.json");
const LLR_ADDRESS: TestSCAddress = TestSCAddress::new("llr");
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/linked-list-repeat");

    blockchain.register_contract(WASM_PATH, linked_list_repeat::ContractBuilder);
    blockchain
}

fn setup() -> ScenarioWorld {
    let mut world = world();
    world
        .account(OWNER_ADDRESS)
        .nonce(1)
        .new_address(OWNER_ADDRESS, 1, LLR_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(linked_list_repeat_proxy::LinkedListRepeatProxy)
        .init()
        .code(WASM_PATH)
        .run();
    world
}

#[test]
fn linked_list_repeat_blackbox_raw() {
    let mut world = setup();

    let num_repeats = 5usize;

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(LLR_ADDRESS)
        .typed(linked_list_repeat_proxy::LinkedListRepeatProxy)
        .add(num_repeats, "test--")
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(LLR_ADDRESS)
        .typed(linked_list_repeat_proxy::LinkedListRepeatProxy)
        .count("test--\x00\x00\x00\x04")
        .returns(ExpectValue(1u32))
        .run();

    let items = world
        .query()
        .to(LLR_ADDRESS)
        .typed(linked_list_repeat_proxy::LinkedListRepeatProxy)
        .bench()
        .returns(ReturnsResult)
        .run();

    for (index, item) in items.into_iter().enumerate() {
        let index_str = String::from_utf8((index as u32).to_be_bytes().to_vec()).unwrap();
        let expected = format!("test--{}", index_str);
        assert_eq!(item.to_string(), expected);
    }
}

#[test]
fn linked_list_repeat_struct_blackbox_raw() {
    let mut world = setup();

    let mut example = ExampleStruct {
        first_token_id: EsdtTokenIdentifier::from_esdt_bytes(b"str:TESTTOK-1234"),
        first_token_nonce: 0,
        first_token_amount: multiversx_sc::types::BigUint::from(1_000_000_000_000_000_000u64),
        second_token_id: EsdtTokenIdentifier::from_esdt_bytes(b"str:TESTTOK-2345"),
        second_token_nonce: 0,
        second_token_amount: multiversx_sc::types::BigUint::from(1_000_000_000_000_000_000u64),
    };

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(LLR_ADDRESS)
        .typed(linked_list_repeat_proxy::LinkedListRepeatProxy)
        .add_struct(5u32, example.clone())
        .run();
    example.first_token_nonce = 3;
    example.second_token_nonce = 3;
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(LLR_ADDRESS)
        .typed(linked_list_repeat_proxy::LinkedListRepeatProxy)
        .count_struct(example)
        .returns(ExpectValue(1u32))
        .run();
}
