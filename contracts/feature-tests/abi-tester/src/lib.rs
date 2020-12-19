#![no_std]

imports!();

mod abi_test_type;

use abi_test_type::*;

#[elrond_wasm_derive::contract(AbiTesterImpl)]
pub trait AbiTester {
	#[endpoint]
	fn echo_abi_test_type(&self, att: AbiTestType) -> AbiTestType {
		att
	}

	#[endpoint]
	fn multi_result_3(&self) -> MultiResult3<i32, [u8; 3], BoxedBytes> {
		(1, [2; 3], BoxedBytes::empty()).into()
	}
}
