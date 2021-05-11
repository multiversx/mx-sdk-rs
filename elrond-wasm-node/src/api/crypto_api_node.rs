use super::ArwenBigUint;
use crate::ArwenApiImpl;
use elrond_wasm::api::BigUintApi;
use elrond_wasm::api::CryptoApi;
use elrond_wasm::types::{BoxedBytes, EllipticCurve, H256};

extern "C" {
	fn bigIntNew(value: i64) -> i32;
	
    fn sha256(dataOffset: *const u8, length: i32, resultOffset: *mut u8) -> i32;
	
    fn keccak256(dataOffset: *const u8, length: i32, resultOffset: *mut u8) -> i32;
	
    fn verifyBLS(
		keyOffset: *const u8,
		messageOffset: *const u8,
		messageLength: i32,
		sigOffset: *const u8,
	) -> i32;
	
    fn verifyEd25519(
		keyOffset: *const u8,
		messageOffset: *const u8,
		messageLength: i32,
		sigOffset: *const u8,
	) -> i32;

	fn verifySecp256k1(
		keyOffset: *const u8,
		keyLength: i32,
		messageOffset: *const u8,
		messageLength: i32,
		sigOffset: *const u8,
	) -> i32;

	fn addEC(
		xResultHandle: i32,
		yResultHandle: i32,
		fieldOrder: i32,
		basePointOrder: i32,
		eqConstant: i32,
		xBasePoint: i32,
		yBasePoint: i32,
		sizeOfField: i32,
		fstPointX: i32,
		fstPointY: i32,
		sndPointX: i32,
		sndPointY: i32,
	);

	fn doubleEC(
		xResultHandle: i32,
		yResultHandle: i32,
		fieldOrder: i32,
		basePointOrder: i32,
		eqConstant: i32,
		xBasePoint: i32,
		yBasePoint: i32,
		sizeOfField: i32,
		pointX: i32,
		pointY: i32,
	);

	fn isOnCurveEC(
		fieldOrder: i32,
		basePointOrder: i32,
		eqConstant: i32,
		xBasePoint: i32,
		yBasePoint: i32,
		sizeOfField: i32,
		pointX: i32,
		pointY: i32,
	) -> i32;

	fn scalarMult(
		xResultHandle: i32,
		yResultHandle: i32,
		fieldOrder: i32,
		basePointOrder: i32,
		eqConstant: i32,
		xBasePoint: i32,
		yBasePoint: i32,
		sizeOfField: i32,
		pointX: i32,
		pointY: i32,
		dataOffset: *const u8,
		length: i32,
	);

	fn scalarBaseMult(
		xResultHandle: i32,
		yResultHandle: i32,
		fieldOrder: i32,
		basePointOrder: i32,
		eqConstant: i32,
		xBasePoint: i32,
		yBasePoint: i32,
		sizeOfField: i32,
		dataOffset: *const u8,
		length: i32,
	);

	fn marshalEC(
		xPairHandle: i32,
		yPairHandle: i32,
		fieldOrder: i32,
		basePointOrder: i32,
		eqConstant: i32,
		xBasePoint: i32,
		yBasePoint: i32,
		sizeOfField: i32,
		resultOffset: *mut u8,
	) -> i32;

	fn marshalCompressedEC(
		xPairHandle: i32,
		yPairHandle: i32,
		fieldOrder: i32,
		basePointOrder: i32,
		eqConstant: i32,
		xBasePoint: i32,
		yBasePoint: i32,
		sizeOfField: i32,
		resultOffset: *mut u8,
	) -> i32;

	fn unmarshalEC(
		xPairHandle: i32,
		yPairHandle: i32,
		fieldOrder: i32,
		basePointOrder: i32,
		eqConstant: i32,
		xBasePoint: i32,
		yBasePoint: i32,
		sizeOfField: i32,
		dataOffset: *const u8,
		length: i32,
	) -> i32;

	fn unmarshalCompressedEC(
		xPairHandle: i32,
		yPairHandle: i32,
		fieldOrder: i32,
		basePointOrder: i32,
		eqConstant: i32,
		xBasePoint: i32,
		yBasePoint: i32,
		sizeOfField: i32,
		dataOffset: *const u8,
		length: i32,
	) -> i32;

	fn generateKeyEC(
		xPubKeyHandle: i32,
		yPubKeyHandle: i32,
		fieldOrder: i32,
		basePointOrder: i32,
		eqConstant: i32,
		xBasePoint: i32,
		yBasePoint: i32,
		sizeOfField: i32,
		resultOffset: *mut u8,
	) -> i32;
}

impl CryptoApi for ArwenApiImpl {
	type BigUint = ArwenBigUint;

	fn sha256(&self, data: &[u8]) -> H256 {
		unsafe {
			let mut res = H256::zero();
			sha256(data.as_ptr(), data.len() as i32, res.as_mut_ptr());
			res
		}
	}

	fn keccak256(&self, data: &[u8]) -> H256 {
		unsafe {
			let mut res = H256::zero();
			keccak256(data.as_ptr(), data.len() as i32, res.as_mut_ptr());
			res
		}
	}

	// the verify functions return 0 if valid signature, -1 if invalid

	fn verify_bls(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
		unsafe {
			verifyBLS(
				key.as_ptr(),
				message.as_ptr(),
				message.len() as i32,
				signature.as_ptr(),
			) == 0
		}
	}

	fn verify_ed25519(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
		unsafe {
			verifyEd25519(
				key.as_ptr(),
				message.as_ptr(),
				message.len() as i32,
				signature.as_ptr(),
			) == 0
		}
	}

	fn verify_secp256k1(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
		unsafe {
			verifySecp256k1(
				key.as_ptr(),
				key.len() as i32,
				message.as_ptr(),
				message.len() as i32,
				signature.as_ptr(),
			) == 0
		}
	}

	// ELLIPTIC CURVE FUNCTIONALITIES

	fn add_ec(
		&self,
		curve: &EllipticCurve<ArwenBigUint>,
		x_first_point: ArwenBigUint,
		y_first_point: ArwenBigUint,
		x_second_point: ArwenBigUint,
		y_second_point: ArwenBigUint,
	) -> (ArwenBigUint, ArwenBigUint) {
		unsafe {
			let x_result_handle = bigIntNew(0);
			let y_result_handle = bigIntNew(0);
			addEC(
				x_result_handle,
				y_result_handle,
				curve.field_order.handle,
				curve.base_point_order.handle,
				curve.eq_constant.handle,
				curve.x_base_point.handle,
				curve.y_base_point.handle,
				curve.size_of_field,
				x_first_point.handle,
				y_first_point.handle,
				x_second_point.handle,
				y_second_point.handle,
			);
			(
				ArwenBigUint {
					handle: x_result_handle,
				},
				ArwenBigUint {
					handle: y_result_handle,
				},
			)
		}
	}

	fn double_ec(
		&self,
		curve: &EllipticCurve<ArwenBigUint>,
		x_point: ArwenBigUint,
		y_point: ArwenBigUint,
	) -> (ArwenBigUint, ArwenBigUint) {
		unsafe {
			let x_result_handle = bigIntNew(0);
			let y_result_handle = bigIntNew(0);
			doubleEC(
				x_result_handle,
				y_result_handle,
				curve.field_order.handle,
				curve.base_point_order.handle,
				curve.eq_constant.handle,
				curve.x_base_point.handle,
				curve.y_base_point.handle,
				curve.size_of_field,
				x_point.handle,
				y_point.handle,
			);
			(
				ArwenBigUint {
					handle: x_result_handle,
				},
				ArwenBigUint {
					handle: y_result_handle,
				},
			)
		}
	}

	fn is_on_curve_ec(
		&self,
		curve: &EllipticCurve<ArwenBigUint>,
		x_point: ArwenBigUint,
		y_point: ArwenBigUint,
	) -> bool {
		unsafe {
			isOnCurveEC(
				curve.field_order.handle,
				curve.base_point_order.handle,
				curve.eq_constant.handle,
				curve.x_base_point.handle,
				curve.y_base_point.handle,
				curve.size_of_field,
				x_point.handle,
				y_point.handle,
			) == 1
		}
	}

	fn scalar_mult(
		&self,
		curve: &EllipticCurve<ArwenBigUint>,
		x_point: ArwenBigUint,
		y_point: ArwenBigUint,
		data: BoxedBytes,
	) -> (ArwenBigUint, ArwenBigUint) {
		unsafe {
			let x_result_handle = bigIntNew(0);
			let y_result_handle = bigIntNew(0);
			scalarMult(
				x_result_handle,
				y_result_handle,
				curve.field_order.handle,
				curve.base_point_order.handle,
				curve.eq_constant.handle,
				curve.x_base_point.handle,
				curve.y_base_point.handle,
				curve.size_of_field,
				x_point.handle,
				y_point.handle,
				data.as_ptr(),
				data.len() as i32,
			);
			(
				ArwenBigUint {
					handle: x_result_handle,
				},
				ArwenBigUint {
					handle: y_result_handle,
				},
			)
		}
	}

	fn scalar_base_mult(
		&self,
		curve: &EllipticCurve<ArwenBigUint>,
		data: BoxedBytes,
	) -> (ArwenBigUint, ArwenBigUint) {
		unsafe {
			let x_result_handle = bigIntNew(0);
			let y_result_handle = bigIntNew(0);
			scalarBaseMult(
				x_result_handle,
				y_result_handle,
				curve.field_order.handle,
				curve.base_point_order.handle,
				curve.eq_constant.handle,
				curve.x_base_point.handle,
				curve.y_base_point.handle,
				curve.size_of_field,
				data.as_ptr(),
				data.len() as i32,
			);
			(
				ArwenBigUint {
					handle: x_result_handle,
				},
				ArwenBigUint {
					handle: y_result_handle,
				},
			)
		}
	}

	fn marshal_ec(
		&self,
		curve: &EllipticCurve<ArwenBigUint>,
		x_pair: ArwenBigUint,
		y_pair: ArwenBigUint,
	) -> BoxedBytes {
		unsafe {
			let mut result = BoxedBytes::allocate(1 + 2 * (curve.size_of_field as usize + 7) / 8);
			marshalEC(
				x_pair.handle,
				y_pair.handle,
				curve.field_order.handle,
				curve.base_point_order.handle,
				curve.eq_constant.handle,
				curve.x_base_point.handle,
				curve.y_base_point.handle,
				curve.size_of_field,
				result.as_mut_ptr(),
			);
			result
		}
	}

	fn marshal_compressed_ec(
		&self,
		curve: &EllipticCurve<ArwenBigUint>,
		x_pair: ArwenBigUint,
		y_pair: ArwenBigUint,
	) -> BoxedBytes {
		unsafe {
			let mut result = BoxedBytes::allocate(1 + (curve.size_of_field as usize + 7) / 8);
			marshalCompressedEC(
				x_pair.handle,
				y_pair.handle,
				curve.field_order.handle,
				curve.base_point_order.handle,
				curve.eq_constant.handle,
				curve.x_base_point.handle,
				curve.y_base_point.handle,
				curve.size_of_field,
				result.as_mut_ptr(),
			);
			result
		}
	}

	fn generate_key_ec(
		&self,
		curve: &EllipticCurve<ArwenBigUint>,
	) -> (ArwenBigUint, ArwenBigUint, BoxedBytes) {
		unsafe {
			let x_pub_key_handle = bigIntNew(0);
			let y_pub_key_handle = bigIntNew(0);
			let mut private_key =
				BoxedBytes::allocate(curve.base_point_order.byte_length() as usize);
			generateKeyEC(
				x_pub_key_handle,
				y_pub_key_handle,
				curve.field_order.handle,
				curve.base_point_order.handle,
				curve.eq_constant.handle,
				curve.x_base_point.handle,
				curve.y_base_point.handle,
				curve.size_of_field,
				private_key.as_mut_ptr(),
			);
			(
				ArwenBigUint {
					handle: x_pub_key_handle,
				},
				ArwenBigUint {
					handle: y_pub_key_handle,
				},
				private_key,
			)
		}
	}

	fn unmarshal_ec(
		&self,
		curve: &EllipticCurve<ArwenBigUint>,
		data: BoxedBytes,
	) -> (ArwenBigUint, ArwenBigUint) {
		unsafe {
			let x_pair_handle = bigIntNew(0);
			let y_pair_handle = bigIntNew(0);
			unmarshalEC(
				x_pair_handle,
				y_pair_handle,
				curve.field_order.handle,
				curve.base_point_order.handle,
				curve.eq_constant.handle,
				curve.x_base_point.handle,
				curve.y_base_point.handle,
				curve.size_of_field,
				data.as_ptr(),
				data.len() as i32,
			);
			(
				ArwenBigUint {
					handle: x_pair_handle,
				},
				ArwenBigUint {
					handle: y_pair_handle,
				},
			)
		}
	}

	fn unmarshal_compressed_ec(
		&self,
		curve: &EllipticCurve<ArwenBigUint>,
		data: BoxedBytes,
	) -> (ArwenBigUint, ArwenBigUint) {
		unsafe {
			let x_pair_handle = bigIntNew(0);
			let y_pair_handle = bigIntNew(0);
			unmarshalCompressedEC(
				x_pair_handle,
				y_pair_handle,
				curve.field_order.handle,
				curve.base_point_order.handle,
				curve.eq_constant.handle,
				curve.x_base_point.handle,
				curve.y_base_point.handle,
				curve.size_of_field,
				data.as_ptr(),
				data.len() as i32,
			);
			(
				ArwenBigUint {
					handle: x_pair_handle,
				},
				ArwenBigUint {
					handle: y_pair_handle,
				},
			)
		}
	}
}
