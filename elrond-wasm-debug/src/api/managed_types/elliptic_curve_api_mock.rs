use core::panic;
use elrond_wasm::types::BoxedBytes;

use super::RustBigUint;
pub struct EllipticCurveMock;

use elrond_wasm::elrond_codec::*;

impl NestedEncode for EllipticCurveMock {
	const TYPE_INFO: TypeInfo = TypeInfo::EllipticCurve;

	fn dep_encode<O: NestedEncodeOutput>(&self, _dest: &mut O) -> Result<(), EncodeError> {
		panic!("not implemented")
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		_dest: &mut O,
		_c: ExitCtx,
		_exit: fn(ExitCtx, EncodeError) -> !,
	) {
		panic!("not implemented")
	}
}

impl TopEncode for EllipticCurveMock {
	const TYPE_INFO: TypeInfo = TypeInfo::EllipticCurve;

	fn top_encode<O: TopEncodeOutput>(&self, _output: O) -> Result<(), EncodeError> {
		panic!("not implemented")
	}

	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		_output: O,
		_c: ExitCtx,
		_exit: fn(ExitCtx, EncodeError) -> !,
	) {
		panic!("not implemented")
	}
}

impl NestedDecode for EllipticCurveMock {
	const TYPE_INFO: TypeInfo = TypeInfo::EllipticCurve;

	fn dep_decode<I: NestedDecodeInput>(_input: &mut I) -> Result<Self, DecodeError> {
		panic!("not implemented")
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		_input: &mut I,
		_c: ExitCtx,
		_exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		panic!("not implemented")
	}
}

impl TopDecode for EllipticCurveMock {
	const TYPE_INFO: TypeInfo = TypeInfo::EllipticCurve;

	fn top_decode<I: TopDecodeInput>(_input: I) -> Result<Self, DecodeError> {
		panic!("not implemented")
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		_input: I,
		_: ExitCtx,
		_: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		panic!("not implemented")
	}
}

impl elrond_wasm::abi::TypeAbi for EllipticCurveMock {
	fn type_name() -> String {
		String::from("EllipticCurve")
	}
}

impl elrond_wasm::api::EllipticCurveApi for EllipticCurveMock {
	type BigUint = RustBigUint;

	fn new_elliptic_curve(
		_field_order: Self::BigUint,
		_base_point_order: Self::BigUint,
		_eq_constant: Self::BigUint,
		_x_base_point: Self::BigUint,
		_y_base_point: Self::BigUint,
		_size_of_field: i32,
	) -> Self {
		panic!("new_elliptic_curve not implemented yet!")
	}

	fn p224_ec() -> Self {
		panic!("p224_ec not implemented yet!")
	}

	fn p256_ec() -> Self {
		panic!("p256_ec not implemented yet!")
	}

	fn p384_ec() -> Self {
		panic!("p384_ec not implemented yet!")
	}

	fn p521_ec() -> Self {
		panic!("p521_ec not implemented yet!")
	}

	fn add_ec(
		&self,
		_x_first_point: Self::BigUint,
		_y_first_point: Self::BigUint,
		_x_second_point: Self::BigUint,
		_y_second_point: Self::BigUint,
	) -> (Self::BigUint, Self::BigUint) {
		panic!("add_ec not implemented yet!")
	}

	fn double_ec(
		&self,
		_x_point: Self::BigUint,
		_y_point: Self::BigUint,
	) -> (Self::BigUint, Self::BigUint) {
		panic!("double_ec not implemented yet!")
	}

	fn is_on_curve_ec(&self, _x_point: Self::BigUint, _y_point: Self::BigUint) -> bool {
		panic!("is_on_curve_ec not implemented yet!")
	}

	fn scalar_mult(
		&self,
		_x_point: Self::BigUint,
		_y_point: Self::BigUint,
		_data: BoxedBytes,
	) -> (Self::BigUint, Self::BigUint) {
		panic!("scalar_mult not implemented yet")
	}

	fn scalar_base_mult(&self, _data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
		panic!("scalar_base_mult not implemented yet!")
	}

	fn marshal_ec(&self, _x_pair: Self::BigUint, _y_pair: Self::BigUint) -> BoxedBytes {
		panic!("marshal_ec not implemented yet!")
	}

	fn marshal_compressed_ec(&self, _x_pair: Self::BigUint, _y_pair: Self::BigUint) -> BoxedBytes {
		panic!("marshal_compressed_ec not implemented yet!")
	}

	fn unmarshal_ec(&self, _data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
		panic!("unmarshal_ec not implemented yet!")
	}

	fn unmarshal_compressed_ec(&self, _data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
		panic!("unmarshal_compressed_ec not implemented yet!")
	}

	fn generate_key_ec(&self) -> (Self::BigUint, Self::BigUint, BoxedBytes) {
		panic!("generate_key_ec not implemented yet!")
	}
}
