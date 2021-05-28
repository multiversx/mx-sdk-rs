elrond_wasm::imports!();

/// All elliptic curve functions provided by Arwen exposed here
#[elrond_wasm_derive::module]
pub trait EllipticCurveFeatures {
	#[endpoint]
	fn compute_add_ec(
		&self,
		curve: Self::EllipticCurve,
		x_first_point: Self::BigUint,
		y_first_point: Self::BigUint,
		x_second_point: Self::BigUint,
		y_second_point: Self::BigUint,
	) -> (Self::BigUint, Self::BigUint) {
		curve.add_ec(x_first_point, y_first_point, x_second_point, y_second_point)
	}

	#[endpoint]
	fn compute_double_ec(
		&self,
		curve: Self::EllipticCurve,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
	) -> (Self::BigUint, Self::BigUint) {
		curve.double_ec(x_point, y_point)
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
	) -> (Self::BigUint, Self::BigUint) {
		self.elliptic_curve().scalar_mult(x_point, y_point, data)
	}

	#[endpoint]
	fn scalar_base_mult(
		&self,
		curve: Self::EllipticCurve,
		data: BoxedBytes,
	) -> (Self::BigUint, Self::BigUint) {
		curve.scalar_base_mult(data)
	}

	#[endpoint]
	fn marshal_ec(
		&self,
		curve: Self::EllipticCurve,
		x_pair: Self::BigUint,
		y_pair: Self::BigUint,
	) -> BoxedBytes {
		self.elliptic_curve().marshal_ec(x_pair, y_pair)
	}

	#[endpoint]
	fn marshal_compressed_ec(
		&self,
		curve: Self::EllipticCurve,
		x_pair: Self::BigUint,
		y_pair: Self::BigUint,
	) -> BoxedBytes {
		self.elliptic_curve().marshal_compressed_ec(x_pair, y_pair)
	}

	#[endpoint]
	fn unmarshal_ec(
		&self,
		curve: Self::EllipticCurve,
		data: BoxedBytes,
	) -> (Self::BigUint, Self::BigUint) {
		curve.unmarshal_ec(data)
	}

	#[endpoint]
	fn unmarshal_compressed_ec(
		&self,
		curve: Self::EllipticCurve,
		data: BoxedBytes,
	) -> (Self::BigUint, Self::BigUint) {
		curve.unmarshal_compressed_ec(data)
	}

	#[endpoint]
	fn generate_key_ec(
		&self,
		curve: Self::EllipticCurve,
	) -> (Self::BigUint, Self::BigUint, BoxedBytes) {
		curve.generate_key_ec()
	}
}
