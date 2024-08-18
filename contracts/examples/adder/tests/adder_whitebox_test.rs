use adder::*;
use multiversx_sc_scenario::imports::*;

const ADDER_PATH_EXPR: &str = "mxsc:output/adder.mxsc.json";
const OWNER: TestAddress = TestAddress::new("owner");
const ADDER_ADDRESS: TestSCAddress = TestSCAddress::new("adder");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/adder.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(CODE_PATH, adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_whitebox() {
    let mut world = world();
    let adder_whitebox = WhiteboxContract::new("sc:adder", adder::contract_obj);
    let adder_code = world.code_expression(ADDER_PATH_EXPR);

    world
        .set_state_step(
            SetStateStep::new()
                .put_account("address:owner", Account::new().nonce(1))
                .new_address("address:owner", 1, "sc:adder"),
        )
        .whitebox_deploy(
            &adder_whitebox,
            ScDeployStep::new().from("address:owner").code(adder_code),
            |sc| {
                sc.init(5u32.into());
            },
        )
        .whitebox_query(&adder_whitebox, |sc| {
            let sum_value = sc.sum();
            assert_eq!(sum_value.get(), 5u32);
        })
        .whitebox_call(
            &adder_whitebox,
            ScCallStep::new().from("address:owner"),
            |sc| sc.add(3u32.into()),
        )
        .check_state_step(
            CheckStateStep::new()
                .put_account("address:owner", CheckAccount::new())
                .put_account(
                    "sc:adder",
                    CheckAccount::new().check_storage("str:sum", "8"),
                ),
        );
}

#[test]
fn adder_whitebox_unified() {
    let mut world = world();

    world.account(OWNER).nonce(1);

    let new_address = world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(ADDER_ADDRESS)
        .returns(ReturnsNewBech32Address)
        .whitebox(adder::contract_obj, |sc| {
            sc.init(BigUint::from(3u64));
        });

    assert_eq!(new_address, ADDER_ADDRESS.to_address().into());

    world
        .tx()
        .from(OWNER)
        .to(ADDER_ADDRESS)
        .whitebox(adder::contract_obj, |sc| {
            sc.add(BigUint::from(5u64));
        });

    let _raw_response = world
        .query()
        .to(ADDER_ADDRESS)
        .returns(ReturnsRawResult)
        .whitebox(adder::contract_obj, |sc| {
            let sum = sc.sum().get();
            assert_eq!(sum, BigUint::from(8u64));
        });
}
