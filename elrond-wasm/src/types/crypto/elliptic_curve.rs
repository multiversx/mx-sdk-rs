use crate::elrond_codec::elrond_codec_derive::{TopDecode, TopEncode};
use crate::{abi::TypeAbi, api::BigUintApi};
use alloc::string::String;
use hex_literal::hex;

// Default Elliptic Curves
// P224
pub const P224_FIELD_ORDER : [u8; 28] = hex!("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF000000000000000000000001");
pub const P224_BASE_POINT_ORDER : [u8; 28] = hex!("FFFFFFFFFFFFFFFFFFFFFFFFFFFF16A2E0B8F03E13DD29455C5C2A3D");
pub const P224_EQ_CONSTANT : [u8; 28] = hex!("b4050a850c04b3abf54132565044b0b7d7bfd8ba270b39432355ffb4");
pub const P224_X_BASE_POINT : [u8; 28] = hex!("b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21");
pub const P224_Y_BASE_POINT : [u8; 28] = hex!("bd376388b5f723fb4c22dfe6cd4375a05a07476444d5819985007e34");
pub const P224_SIZE_OF_FIELD : i32 = 224;

//P256
pub const P256_FIELD_ORDER : [u8; 32] = hex!("FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF");
pub const P256_BASE_POINT_ORDER : [u8; 32] = hex!("FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551");
pub const P256_EQ_CONSTANT : [u8; 32] = hex!("5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b");
pub const P256_X_BASE_POINT : [u8; 32] = hex!("6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296");
pub const P256_Y_BASE_POINT : [u8; 32] = hex!("4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5");
pub const P256_SIZE_OF_FIELD : i32 = 256;

//P384
pub const P384_FIELD_ORDER : [u8; 48] = hex!("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFFFF0000000000000000FFFFFFFF");
pub const P384_BASE_POINT_ORDER : [u8; 48] = hex!("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC7634D81F4372DDF581A0DB248B0A77AECEC196ACCC52973");
pub const P384_EQ_CONSTANT : [u8; 48] = hex!("b3312fa7e23ee7e4988e056be3f82d19181d9c6efe8141120314088f5013875ac656398d8a2ed19d2a85c8edd3ec2aef");
pub const P384_X_BASE_POINT : [u8; 48] = hex!("aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a385502f25dbf55296c3a545e3872760ab7");
pub const P384_Y_BASE_POINT : [u8; 48] = hex!("3617de4a96262c6f5d9e98bf9292dc29f8f41dbd289a147ce9da3113b5f0b8c00a60b1ce1d7e819d7a431d7c90ea0e5f");
pub const P384_SIZE_OF_FIELD : i32 = 384;

//P521 - TO BE FIXED
// pub const P521_FIELD_ORDER : [u8; 66] = hex!("1FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
// pub const P521_BASE_POINT_ORDER : [u8; 66] = hex!("1FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFA51868783BF2F966B7FCC0148F709A5D03BB5C9B8899C47AEBB6FB71E91386409");
// pub const P521_EQ_CONSTANT : [u8; 66] = hex!("051953eb9618e1c9a1f929a21a0b68540eea2da725b99b315f3b8b489918ef109e156193951ec7e937b1652c0bd3bb1bf073573df883d2c34f1ef451fd46b503f00");
// pub const P521_X_BASE_POINT : [u8; 66] = hex!("c6858e06b70404e9cd9e3ecb662395b4429c648139053fb521f828af606b4d3dbaa14b5e77efe75928fe1dc127a2ffa8de3348b3c1856a429bf97e7e31c2e5bd66");
// pub const P521_Y_BASE_POINT : [u8; 66] = hex!("11839296a789a3bc0045c8a5fb42c7d1bd998f54449579b446817afbd17273e662c97ee72995ef42640c550b9013fad0761353c7086a272c24088be94769fd16650");
// pub const P521_SIZE_OF_FIELD : i32 = 521;

#[derive(TopDecode, TopEncode, PartialEq)]
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

    pub fn p224() -> Self {
		Self {
			field_order: BigUint::from_bytes_be(&P224_FIELD_ORDER),
			base_point_order: BigUint::from_bytes_be(&P224_BASE_POINT_ORDER),
			eq_constant: BigUint::from_bytes_be(&P224_EQ_CONSTANT),
			x_base_point: BigUint::from_bytes_be(&P224_X_BASE_POINT),
			y_base_point: BigUint::from_bytes_be(&P224_Y_BASE_POINT),
			size_of_field: P224_SIZE_OF_FIELD,
		}
	}

    pub fn p256() -> Self {
		Self {
			field_order: BigUint::from_bytes_be(&P256_FIELD_ORDER),
			base_point_order: BigUint::from_bytes_be(&P256_BASE_POINT_ORDER),
			eq_constant: BigUint::from_bytes_be(&P256_EQ_CONSTANT),
			x_base_point: BigUint::from_bytes_be(&P256_X_BASE_POINT),
			y_base_point: BigUint::from_bytes_be(&P256_Y_BASE_POINT),
			size_of_field: P256_SIZE_OF_FIELD,
		}
	}

    pub fn p384() -> Self {
		Self {
			field_order: BigUint::from_bytes_be(&P384_FIELD_ORDER),
			base_point_order: BigUint::from_bytes_be(&P384_BASE_POINT_ORDER),
			eq_constant: BigUint::from_bytes_be(&P384_EQ_CONSTANT),
			x_base_point: BigUint::from_bytes_be(&P384_X_BASE_POINT),
			y_base_point: BigUint::from_bytes_be(&P384_Y_BASE_POINT),
			size_of_field: P384_SIZE_OF_FIELD,
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
