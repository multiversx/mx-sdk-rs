elrond_wasm::imports!();
type EllipticCurveComponents<BigUint> = (BigUint, BigUint, BigUint, BigUint, BigUint, u32);

/// All elliptic curve functions provided by Arwen exposed here
#[elrond_wasm_derive::module]
pub trait EllipticCurveFeatures {
	#[endpoint]
	fn compute_new_elliptic_curve(
		&self,
		field_order: Self::BigUint,
		base_point_order: Self::BigUint,
		eq_constant: Self::BigUint,
		x_base_point: Self::BigUint,
		y_base_point: Self::BigUint,
		size_of_field: u32,
	) -> EllipticCurveComponents<Self::BigUint> {
		Self::EllipticCurve::new_elliptic_curve(
			field_order,
			base_point_order,
			eq_constant,
			x_base_point,
			y_base_point,
			size_of_field,
		).get_values()
	}

 	#[endpoint]
	fn compute_get_values(
		&self,
		curve: Self::EllipticCurve,
	) -> EllipticCurveComponents<Self::BigUint> {
		curve.get_values()
	}

	#[endpoint]
	fn compute_p224_ec(&self) -> EllipticCurveComponents<Self::BigUint> {
		Self::EllipticCurve::p224_ec().get_values()
	}

	#[endpoint]
	fn compute_p256_ec(&self) -> EllipticCurveComponents<Self::BigUint> {
		Self::EllipticCurve::p256_ec().get_values()
	}

	#[endpoint]
	fn compute_p384_ec(&self) -> EllipticCurveComponents<Self::BigUint> {
		Self::EllipticCurve::p384_ec().get_values()
	}

	#[endpoint]
	fn compute_p521_ec(&self) -> EllipticCurveComponents<Self::BigUint> {
		Self::EllipticCurve::p521_ec().get_values()
	}

    #[endpoint]
    fn compute_get_ec_length(&self, curve: Self::EllipticCurve) -> u32 {
        curve.get_ec_length()
    }

    #[endpoint]
    fn compute_get_priv_key_byte_length(&self, curve: Self::EllipticCurve) -> u32 {
        curve.get_priv_key_byte_length()
    }

	#[endpoint]
	fn compute_add_ec(
		&self,
		curve: Self::EllipticCurve,
		x_first_point: Self::BigUint,
		y_first_point: Self::BigUint,
		x_second_point: Self::BigUint,
		y_second_point: Self::BigUint,
	) -> MultiResult2<Self::BigUint,Self::BigUint> {
		curve.add_ec(x_first_point, y_first_point, x_second_point, y_second_point).into()
	}

	#[endpoint]
	fn compute_double_ec(
		&self,
		curve: Self::EllipticCurve,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
	) -> MultiResult2<Self::BigUint,Self::BigUint> {
		curve.double_ec(x_point, y_point).into()
	}

	#[endpoint]
	fn compute_is_on_curve_ec(
		&self,
		curve: Self::EllipticCurve,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
	) -> bool {
		curve.is_on_curve_ec(x_point, y_point)
	}

	#[endpoint]
	fn compute_scalar_mult(
		&self,
		curve: Self::EllipticCurve,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
		data: BoxedBytes,
	) -> MultiResult2<Self::BigUint,Self::BigUint> {
		curve.scalar_mult(x_point, y_point, data).into()
	}

	#[endpoint]
	fn compute_scalar_base_mult(
		&self,
		curve: Self::EllipticCurve,
		data: BoxedBytes,
	) -> MultiResult2<Self::BigUint,Self::BigUint> {
		curve.scalar_base_mult(data).into()
	}

	#[endpoint]
	fn compute_marshal_ec(
		&self,
		curve: Self::EllipticCurve,
		x_pair: Self::BigUint,
		y_pair: Self::BigUint,
	) -> BoxedBytes {
		curve.marshal_ec(x_pair, y_pair)
	}

	#[endpoint]
	fn compute_marshal_compressed_ec(
		&self,
		curve: Self::EllipticCurve,
		x_pair: Self::BigUint,
		y_pair: Self::BigUint,
	) -> BoxedBytes {
		curve.marshal_compressed_ec(x_pair, y_pair)
	}

	#[endpoint]
	fn compute_unmarshal_ec(
		&self,
		curve: Self::EllipticCurve,
		data: BoxedBytes,
	) -> MultiResult2<Self::BigUint,Self::BigUint> {
		curve.unmarshal_ec(data).into()
	}

	#[endpoint]
	fn compute_unmarshal_compressed_ec(
		&self,
		curve: Self::EllipticCurve,
		data: BoxedBytes,
	) -> MultiResult2<Self::BigUint,Self::BigUint> {
		curve.unmarshal_compressed_ec(data).into()
	}

	#[endpoint]
	fn compute_generate_key_ec(
		&self,
		curve: Self::EllipticCurve,
	) -> MultiResult3<Self::BigUint,Self::BigUint,BoxedBytes> {
		curve.generate_key_ec().into()
	}  
}
