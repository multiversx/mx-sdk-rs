
extern crate basic_features;
use basic_features::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

#[test]
fn test_sc_error() {
    let bf = BasicFeaturesImpl::new(TxContext::dummy());
    let result = bf.return_error();
    assert_eq!(SCResult::Err(SCError::Static(&b"return_error"[..])), result);

}
