
extern crate adder;
use adder::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

fn set_up_module_to_test() -> AdderImpl<ArwenMockRef, RustBigInt, RustBigUint> {
    let mock_ref = ArwenMockState::new_ref();
    mock_ref.add_account(AccountData{
        address: Address::zero(),
        nonce: 0,
        balance: 0u32.into(),
        storage: HashMap::new(),
        contract: None,
    });
    mock_ref.set_dummy_tx(&Address::zero());

    AdderImpl::new(mock_ref.clone())
}

#[test]
fn test_add() {
    let adder = set_up_module_to_test();

    adder.init(&RustBigInt::from(5));
    assert_eq!(RustBigInt::from(5), *adder.get_mut_sum());

    let _ = adder.add(&RustBigInt::from(7));
    assert_eq!(RustBigInt::from(12), *adder.get_mut_sum());

    let _ = adder.add(&RustBigInt::from(1));
    assert_eq!(RustBigInt::from(13), *adder.get_mut_sum());
}
