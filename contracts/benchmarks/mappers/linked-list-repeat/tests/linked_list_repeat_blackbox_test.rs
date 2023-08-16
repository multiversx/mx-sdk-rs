use benchmark_common::ExampleStruct;
use linked_list_repeat::ProxyTrait;
use multiversx_sc::types::{MultiValueEncoded, TokenIdentifier};
use multiversx_sc_scenario::{api::StaticApi, scenario_model::*, *};

const WASM_PATH_EXPR: &str = "file:output/linked-list-repeat.wasm";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/linked-list-repeat");

    blockchain.register_contract(WASM_PATH_EXPR, linked_list_repeat::ContractBuilder);
    blockchain
}

fn setup() -> ScenarioWorld {
    let mut world = world();
    let llr_code = world.code_expression(WASM_PATH_EXPR);

    world
        .set_state_step(
            SetStateStep::new()
                .put_account("address:owner", Account::new().nonce(1))
                .new_address("address:owner", 1, "sc:llr"),
        )
        .sc_deploy(
            ScDeployStep::new()
                .from("address:owner")
                .code(llr_code)
                .expect(TxExpect::ok().no_result()),
        );
    world
}

#[test]
fn linked_list_repeat_blackbox_raw() {
    let mut world = setup();
    let mut contract = ContractInfo::<linked_list_repeat::Proxy<StaticApi>>::new("sc:llr");

    let num_repeats = 5usize;

    world
        .sc_call(
            ScCallStep::new()
                .from("address:owner")
                .to("sc:llr")
                .call(contract.add(num_repeats, "test--"))
                .expect(TxExpect::ok().no_result()),
        )
        .sc_call(
            ScCallStep::new()
                .from("address:owner")
                .to("sc:llr")
                .call(contract.count("test--\x00\x00\x00\x04"))
                .expect(TxExpect::ok().result("1")),
        )
        .sc_query_use_result(
            ScQueryStep::new().to("sc:llr").call(contract.bench()),
            |tr: TypedResponse<MultiValueEncoded<StaticApi, String>>| {
                let result = tr.result.unwrap().into_iter().collect::<Vec<String>>();
                assert_eq!(result.len(), num_repeats);
                for (index, item) in result.iter().enumerate() {
                    let index_str =
                        String::from_utf8((index as u32).to_be_bytes().to_vec()).unwrap();
                    let expected = format!("test--{}", index_str);
                    assert_eq!(item, &expected);
                }
            },
        );
}

#[test]
fn linked_list_repeat_struct_blackbox_raw() {
    let mut world = setup();
    let mut contract = ContractInfo::<linked_list_repeat::Proxy<StaticApi>>::new("sc:llr");

    let mut example = ExampleStruct {
        first_token_id: TokenIdentifier::from_esdt_bytes(b"str:TESTTOK-1234"),
        first_token_nonce: 0,
        first_token_amount: multiversx_sc::types::BigUint::from(1_000_000_000_000_000_000u64),
        second_token_id: TokenIdentifier::from_esdt_bytes(b"str:TESTTOK-2345"),
        second_token_nonce: 0,
        second_token_amount: multiversx_sc::types::BigUint::from(1_000_000_000_000_000_000u64),
    };
    world.sc_call(
        ScCallStep::new()
            .from("address:owner")
            .to("sc:llr")
            .call(contract.add_struct(5u32, example.clone()))
            .expect(TxExpect::ok().no_result()),
    );
    example.first_token_nonce = 3;
    example.second_token_nonce = 3;
    world.sc_call(
        ScCallStep::new()
            .from("address:owner")
            .to("sc:llr")
            .call(contract.count_struct(example))
            .expect(TxExpect::ok().result("1")),
    );
}
