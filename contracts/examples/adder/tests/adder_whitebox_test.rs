use adder::*;
use multiversx_sc_scenario::imports::*;

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
