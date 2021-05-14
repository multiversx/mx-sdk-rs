use elrond_wasm::types::{SCError, SCResult};
use elrond_wasm_debug::*;

use basic_features::macros::Macros;

#[test]
fn test_sc_error() {
	let bf = basic_features::contract_obj(TxContext::dummy());
	let result = bf.return_sc_error();
	assert_eq!(
		SCResult::Err(SCError::from(&b"return_sc_error"[..])),
		result
	);
}
