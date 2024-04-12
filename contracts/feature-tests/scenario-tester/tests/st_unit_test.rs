use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::api::SingleTxApi;
use scenario_tester::*;

#[test]
fn st_unit_test() {
    let contract = scenario_tester::contract_obj::<SingleTxApi>();

    contract.init(BigUint::from(5u32));
    assert_eq!(BigUint::from(5u32), contract.sum().get());

    contract.add(BigUint::from(7u32));
    assert_eq!(BigUint::from(12u32), contract.sum().get());

    contract.add(BigUint::from(1u32));
    assert_eq!(BigUint::from(13u32), contract.sum().get());
}
