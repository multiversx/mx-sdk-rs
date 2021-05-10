use crate::api::BigUintApi;
use crate::elrond_codec::elrond_codec_derive::{TopDecode, TopEncode};

#[derive(TopDecode, TopEncode)]
pub struct EllipticCurve<BigUint: BigUintApi> {
	pub field_order: BigUint,
	pub base_point_order: BigUint,
	pub eq_constant: BigUint,
	pub x_base_point: BigUint,
	pub y_base_point: BigUint,
	pub size_of_field: i32,
}

impl<BigUint: BigUintApi> EllipticCurve<BigUint> {
	pub fn new(
		field_order: BigUint,
		base_point_order: BigUint,
		eq_constant: BigUint,
		x_base_point: BigUint,
		y_base_point: BigUint,
		size_of_field: i32,
	) -> Self {
		Self {
			field_order,
			base_point_order,
			eq_constant,
			x_base_point,
			y_base_point,
			size_of_field,
		}
	}
}
