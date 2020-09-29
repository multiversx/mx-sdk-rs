
extern crate factorial;
use factorial::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

fn set_up_module_to_test() -> FactorialImpl<ArwenMockRef, RustBigInt, RustBigUint> {
    let mock_ref = ArwenMockState::new_ref();
    mock_ref.add_account(AccountData{
        address: Address::zero(),
        nonce: 0,
        balance: 0u32.into(),
        storage: HashMap::new(),
        contract: None,
    });
    mock_ref.set_dummy_tx(&Address::zero());

    FactorialImpl::new(mock_ref.clone())
}

#[test]
fn test_add() {
    let factorial = set_up_module_to_test();

    assert_eq!(RustBigUint::from(1u32),   factorial.factorial(RustBigUint::from(0u32)));
    assert_eq!(RustBigUint::from(1u32),   factorial.factorial(RustBigUint::from(1u32)));
    assert_eq!(RustBigUint::from(2u32),   factorial.factorial(RustBigUint::from(2u32)));
    assert_eq!(RustBigUint::from(6u32),   factorial.factorial(RustBigUint::from(3u32)));
    assert_eq!(RustBigUint::from(24u32),  factorial.factorial(RustBigUint::from(4u32)));
    assert_eq!(RustBigUint::from(120u32), factorial.factorial(RustBigUint::from(5u32)));
}
