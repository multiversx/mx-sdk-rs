use elrond_wasm::api::EllipticCurveApi;
//use crate::ArwenApiImpl;
use super::ArwenBigUint;
use elrond_wasm::elrond_codec::*;
use crate::String;
use elrond_wasm::types::BoxedBytes;

extern "C" {
	fn bigIntNew(value: i64) -> i32;

	fn addEC(
		xResultHandle: i32,
		yResultHandle: i32,
        ecHandle: i32,
		fstPointXHandle: i32,
		fstPointYHandle: i32,
		sndPointXHandle: i32,
		sndPointYHandle: i32,
	);

	fn doubleEC(
		xResultHandle: i32,
		yResultHandle: i32,
		ecHandle: i32,
		pointXHandle: i32,
		pointYHandle: i32,
	);

	fn isOnCurveEC(
		ecHandle: i32,
		pointXHandle: i32,
		pointYHandle: i32,
	) -> i32;

	fn scalarMultEC(
		xResultHandle: i32,
		yResultHandle: i32,
		ecHandle: i32,
		pointXHandle: i32,
		pointYHandle: i32,
		dataOffset: *const u8,
		length: i32,
	) -> i32;

	fn scalarBaseMultEC(
		xResultHandle: i32,
		yResultHandle: i32,
		ecHandle: i32,
		dataOffset: *const u8,
		length: i32,
	) -> i32;

	fn marshalEC(
		xPairHandle: i32,
		yPairHandle: i32,
		ecHandle: i32,
		resultOffset: *mut u8,
	) -> i32;

	fn marshalCompressedEC(
		xPairHandle: i32,
		yPairHandle: i32,
		ecHandle: i32,
		resultOffset: *mut u8,
	) -> i32;

	fn unmarshalEC(
		xPairHandle: i32,
		yPairHandle: i32,
		ecHandle: i32,
		dataOffset: *const u8,
		length: i32,
	) -> i32;

	fn unmarshalCompressedEC(
		xPairHandle: i32,
		yPairHandle: i32,
		ecHandle: i32,
		dataOffset: *const u8,
		length: i32,
	) -> i32;

	fn generateKeyEC(
		xPubKeyHandle: i32,
		yPubKeyHandle: i32,
		ecHandle: i32,
		resultOffset: *mut u8,
	) -> i32;

    fn ellipticCurveNew(
        fieldOrderHandle: i32,
        basePointOrderHandle: i32,
        eqConstantHandle: i32,
        xBasePointHandle: i32,
        yBasePointHandle: i32,
        sizeOfField: i32
    ) -> i32;

    fn p224Ec() -> i32;

    fn p256Ec() -> i32;

    fn p384Ec() -> i32;

    fn p521Ec() -> i32;
}

// pub struct ArwenEllipticCurve {
//     field_order : ArwenBigUint,
//     base_point_order : ArwenBigUint,
//     eq_constant : ArwenBigUint,
//     x_base_point : ArwenBigUint,
//     y_base_point : ArwenBigUint,
//     size_of_field : u32,
// }

pub struct ArwenEllipticCurve{
    pub handle: i32,
}

impl elrond_wasm::abi::TypeAbi for ArwenEllipticCurve {
    fn type_name() -> String {
        String::from("EllipticCurve")
    }
}

impl EllipticCurveApi for ArwenEllipticCurve{
    type EllipticCurve = ArwenEllipticCurve;
    type BigUint = ArwenBigUint;

    fn new_elliptic_curve(field_order: Self::BigUint, base_point_order: Self::BigUint, eq_constant: Self::BigUint, x_base_point: Self::BigUint, y_base_point: Self::BigUint, size_of_field: i32) -> Self {
        unsafe {
            let handle = ellipticCurveNew(
                field_order.handle,
                base_point_order.handle,
                eq_constant.handle,
                x_base_point.handle,
                y_base_point.handle,
                size_of_field);
                ArwenEllipticCurve { handle }
        }
    }

    fn p224_ec() -> Self {
        unsafe {
            let handle = p224Ec();
            ArwenEllipticCurve{ handle }
        }
    }
    
    fn p256_ec() -> Self {
        let handle = p256Ec();
        ArwenEllipticCurve{ handle }    
    }

    fn p384_ec() -> Self {
        let handle = p384Ec();
        ArwenEllipticCurve{ handle }
    }

    fn p521_ec() -> Self {
        let handle = p521Ec();
        ArwenEllipticCurve{ handle }
    }

    fn add_ec(&self, x_first_point: Self::BigUint, y_first_point: Self::BigUint, x_second_point: Self::BigUint, y_second_point: Self::BigUint) ->(Self::BigUint,Self::BigUint) {
        unsafe {
			let x_result_handle = bigIntNew(0);
			let y_result_handle = bigIntNew(0);
			addEC(
				x_result_handle,
				y_result_handle,
				self.handle,
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

    fn double_ec(&self, x_point: Self::BigUint, y_point: Self::BigUint) -> (Self::BigUint, Self::BigUint) {
        unsafe {
            let x_result_handle = bigIntNew(0);
			let y_result_handle = bigIntNew(0);
			doubleEC(
				x_result_handle,
				y_result_handle,
				self.handle,
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

    fn is_on_curve_ec(&self, x_point: Self::BigUint, y_point: Self::BigUint) -> bool {
        unsafe {
			isOnCurveEC(
				self.handle,
				x_point.handle,
				y_point.handle,
			) == 1
		}
    }

    fn scalar_mult(&self, x_point: Self::BigUint, y_point: Self::BigUint, data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
        unsafe {
			let x_result_handle = bigIntNew(0);
			let y_result_handle = bigIntNew(0);
			scalarMultEC(
				x_result_handle,
				y_result_handle,
				self.handle,
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

    fn scalar_base_mult(&self, data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
        unsafe {
			let x_result_handle = bigIntNew(0);
			let y_result_handle = bigIntNew(0);
			scalarBaseMultEC(
				x_result_handle,
				y_result_handle,
				self.handle,
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

    fn marshal_ec(&self, x_pair: Self::BigUint, y_pair: Self::BigUint) -> BoxedBytes {
        unsafe {
			let mut result = BoxedBytes::allocate(1 + 2 * (curve.size_of_field as usize + 7) / 8);
			marshalEC(
				x_pair.handle,
				y_pair.handle,
				self.handle,
				result.as_mut_ptr(),
			);
			result
		}
    }

    fn marshal_compressed_ec(&self, x_pair: Self::BigUint, y_pair: Self::BigUint) -> BoxedBytes {
        unsafe {
			let mut result = BoxedBytes::allocate(1 + (curve.size_of_field as usize + 7) / 8);
			marshalCompressedEC(
				x_pair.handle,
				y_pair.handle,
				self.handle,
				result.as_mut_ptr(),
			);
			result
		}
    }

    fn unmarshal_ec(&self, data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
        unsafe {
			let x_pair_handle = bigIntNew(0);
			let y_pair_handle = bigIntNew(0);
			unmarshalEC(
				x_pair_handle,
				y_pair_handle,
				self.handle,
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

    fn unmarshal_compressed_ec(&self, data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
        unsafe {
			let x_pair_handle = bigIntNew(0);
			let y_pair_handle = bigIntNew(0);
			unmarshalEC(
				x_pair_handle,
				y_pair_handle,
				self.handle,
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

    fn generate_key_ec(&self) -> (Self::BigUint, Self::BigUint, BoxedBytes) {
        unsafe {
			let x_pub_key_handle = bigIntNew(0);
			let y_pub_key_handle = bigIntNew(0);
			let mut private_key =
				BoxedBytes::allocate(curve.base_point_order.byte_length() as usize);
			generateKeyEC(
				x_pub_key_handle,
				y_pub_key_handle,
				self.handle,
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
		}    }
}