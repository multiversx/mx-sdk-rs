
extern crate basic_features;
use basic_features::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

fn set_up_module_to_test() -> BasicFeaturesImpl<ArwenMockRef, RustBigInt, RustBigUint> {
    let mock_ref = ArwenMockState::new_ref();
    mock_ref.add_account(AccountData{
        address: Address::zero(),
        nonce: 0,
        balance: 0.into(),
        storage: HashMap::new(),
        contract: None,
    });
    mock_ref.set_dummy_tx(&Address::zero());

    BasicFeaturesImpl::new(mock_ref.clone())
}

#[test]
fn test_sc_error() {
    let bf = set_up_module_to_test();

    let result = bf.return_error();
    assert_eq!(SCResult::Err(SCError::Static(&b"return_error"[..])), result);

}
