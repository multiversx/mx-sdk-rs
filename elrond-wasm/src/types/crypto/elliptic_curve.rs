use crate::elrond_codec::elrond_codec_derive::{TopDecode, TopEncode};
use crate::{abi::TypeAbi, api::BigUintApi};
use alloc::string::String;

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

impl<BigUint: BigUintApi> TypeAbi for EllipticCurve<BigUint> {
	fn type_name() -> String {
		"EllipticCurve".into()
	}

	/// This was auto-generated using the TypeAbi derive outside this crate,
	/// and then copied here.
	/// The framework does not usually provide full ABI type decriptions for its types,
	/// but in this case there are more fields thna usual and it might be of help for developers.
	fn provide_type_descriptions<TDC: crate::abi::TypeDescriptionContainer>(accumulator: &mut TDC) {
		let type_name = Self::type_name();
		if !accumulator.contains_type(&type_name) {
			let mut field_descriptions = crate::Vec::new();
			field_descriptions.push(crate::abi::StructFieldDescription {
				docs: &[],
				name: "field_order",
				field_type: <BigUint>::type_name(),
			});
			field_descriptions.push(crate::abi::StructFieldDescription {
				docs: &[],
				name: "base_point_order",
				field_type: <BigUint>::type_name(),
			});
			field_descriptions.push(crate::abi::StructFieldDescription {
				docs: &[],
				name: "eq_constant",
				field_type: <BigUint>::type_name(),
			});
			field_descriptions.push(crate::abi::StructFieldDescription {
				docs: &[],
				name: "x_base_point",
				field_type: <BigUint>::type_name(),
			});
			field_descriptions.push(crate::abi::StructFieldDescription {
				docs: &[],
				name: "y_base_point",
				field_type: <BigUint>::type_name(),
			});
			field_descriptions.push(crate::abi::StructFieldDescription {
				docs: &[],
				name: "size_of_field",
				field_type: <i32>::type_name(),
			});
			accumulator.insert(
				type_name.clone(),
				crate::abi::TypeDescription {
					docs: &[],
					name: type_name,
					contents: crate::abi::TypeContents::Struct(field_descriptions),
				},
			);
		}
	}
}
