use multiversx_sc_scenario::{scenario_model::*, *};

const PF_PATH_EXPR: &str = "file:output/payable-features.wasm";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/payable-features");

    blockchain.register_contract(PF_PATH_EXPR, payable_features::ContractBuilder);
    blockchain
}

#[test]
fn payable_multi() {
    let mut world = world();
    let pf_code = world.code_expression(PF_PATH_EXPR);

    world
        .set_state_step(
            SetStateStep::new()
                .put_account("sc:payable-features", Account::new().code(pf_code))
                .put_account(
                    "address:an-account",
                    Account::new()
                        .balance("10000")
                        .esdt_balance("str:TOK-000001", "1000")
                        .esdt_balance("str:TOK-000002", "500")
                        .esdt_balance("str:TOK-000003", "500")
                        .esdt_nft_balance("str:SFT-123", 5u32, 10u32, Option::<()>::None) 
                ),
        )
        .sc_call(
            ScCallStep::new()
                .from("address:an-account")
                .to("sc:payable-features")
                .function("echo_call_value")
                .esdt_transfer("str:TOK-000001", 0, "100")
                .esdt_transfer("str:TOK-000002", 0, "400")
                .expect(
                    TxExpect::ok()
                        .result("0")
                        .result("nested:str:TOK-000001|u64:0|biguint:100|nested:str:TOK-000002|u64:0|biguint:400")
                ),
        );
}
