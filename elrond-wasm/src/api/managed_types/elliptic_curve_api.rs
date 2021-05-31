use crate::abi;
use crate::types::BoxedBytes;

/// Definition of the EllipticCurve type required by the API
pub trait EllipticCurveApi:
	Sized
	+ elrond_codec::NestedEncode
	+ elrond_codec::TopEncode
	+ elrond_codec::NestedDecode
	+ elrond_codec::TopDecode
	+ abi::TypeAbi
{
	type BigUint;

	fn new_elliptic_curve(
		field_order: Self::BigUint,
		base_point_order: Self::BigUint,
		eq_constant: Self::BigUint,
		x_base_point: Self::BigUint,
		y_base_point: Self::BigUint,
		size_of_field: i32,
	) -> Self;

	fn p224_ec() -> Self;

	fn p256_ec() -> Self;

	fn p384_ec() -> Self;

	fn p521_ec() -> Self;

	fn add_ec(
		&self,
		x_first_point: Self::BigUint,
		y_first_point: Self::BigUint,
		x_second_point: Self::BigUint,
		y_second_point: Self::BigUint,
	) -> (Self::BigUint, Self::BigUint);

	fn double_ec(
		&self,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
	) -> (Self::BigUint, Self::BigUint);

	fn is_on_curve_ec(&self, x_point: Self::BigUint, y_point: Self::BigUint) -> bool;

	fn scalar_mult(
		&self,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
		data: BoxedBytes,
	) -> (Self::BigUint, Self::BigUint);

	fn scalar_base_mult(&self, data: BoxedBytes) -> (Self::BigUint, Self::BigUint);

	fn marshal_ec(&self, x_pair: Self::BigUint, y_pair: Self::BigUint) -> BoxedBytes;

	fn marshal_compressed_ec(&self, x_pair: Self::BigUint, y_pair: Self::BigUint) -> BoxedBytes;

	fn unmarshal_ec(&self, data: BoxedBytes) -> (Self::BigUint, Self::BigUint);

	fn unmarshal_compressed_ec(&self, data: BoxedBytes) -> (Self::BigUint, Self::BigUint);

	fn generate_key_ec(&self) -> (Self::BigUint, Self::BigUint, BoxedBytes);
}
