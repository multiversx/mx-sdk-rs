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

	#[endpoint]
	fn multi_result_4(&self) -> MultiResult4<i32, [u8; 3], BoxedBytes, OnlyShowsUpAsNested3> {
		(
			1,
			[2; 3],
			BoxedBytes::empty(),
			OnlyShowsUpAsNested3 { something: () },
		)
			.into()
	}

	#[endpoint]
	fn var_args(
		&self,
		_simple_arg: u32,
		#[var_args] _var_args: VarArgs<MultiArg2<OnlyShowsUpAsNested4, i32>>,
	) {
	}

	#[endpoint]
	fn multi_result_vec(&self) -> MultiResultVec<OnlyShowsUpAsNested5> {
		MultiResultVec::new()
	}

	#[endpoint]
	fn optional_arg(
		&self,
		_simple_arg: u32,
		#[var_args] _opt_args: OptionalArg<OnlyShowsUpAsNested6>,
	) {
	}

	#[endpoint]
	fn optional_result(&self) -> OptionalResult<OnlyShowsUpAsNested7> {
		OptionalResult::None
	}
}
