#![allow(deprecated)]

multiversx_sc::imports!();

/// All elliptic curve functions provided by Arwen exposed here.
/// TODO: remove heap allocation and move to basic features.
#[multiversx_sc::module]
pub trait EllipticCurveFeatures {
    #[endpoint]
    fn compute_get_values(&self, curve_bitsize: u32) -> EllipticCurveComponents<CurrentApi> {
        match EllipticCurve::from_bitsize(curve_bitsize) {
            Some(ec) => ec.get_values(),
            None => (
                BaseBigUint::zero(),
                BaseBigUint::zero(),
                BaseBigUint::zero(),
                BaseBigUint::zero(),
                BaseBigUint::zero(),
                0,
            ),
        }
    }

    #[endpoint]
    fn compute_create_ec(&self, curve: &str) -> EllipticCurveComponents<CurrentApi> {
        EllipticCurve::from_name_str(curve).get_values()
    }

    #[endpoint]
    fn compute_get_ec_length(&self, curve_bitsize: u32) -> u32 {
        match EllipticCurve::from_bitsize(curve_bitsize) {
            Some(ec) => ec.get_curve_length(),
            None => curve_bitsize,
        }
    }

    #[endpoint]
    fn compute_get_priv_key_byte_length(&self, curve_bitsize: u32) -> u32 {
        match EllipticCurve::from_bitsize(curve_bitsize) {
            Some(ec) => ec.get_priv_key_byte_length(),
            None => 0,
        }
    }

    #[endpoint]
    fn compute_ec_add(
        &self,
        curve_bitsize: u32,
        x_first_point: BaseBigUint,
        y_first_point: BaseBigUint,
        x_second_point: BaseBigUint,
        y_second_point: BaseBigUint,
    ) -> MultiValue2<BaseBigUint, BaseBigUint> {
        match EllipticCurve::from_bitsize(curve_bitsize) {
            Some(ec) => ec
                .add(x_first_point, y_first_point, x_second_point, y_second_point)
                .into(),
            None => (BaseBigUint::zero(), BaseBigUint::zero()).into(),
        }
    }

    #[endpoint]
    fn compute_ec_double(
        &self,
        curve_bitsize: u32,
        x_point: BaseBigUint,
        y_point: BaseBigUint,
    ) -> MultiValue2<BaseBigUint, BaseBigUint> {
        match EllipticCurve::from_bitsize(curve_bitsize) {
            Some(ec) => ec.double(x_point, y_point).into(),
            None => (BaseBigUint::zero(), BaseBigUint::zero()).into(),
        }
    }

    #[endpoint]
    fn compute_is_on_curve_ec(
        &self,
        curve_bitsize: u32,
        x_point: BaseBigUint,
        y_point: BaseBigUint,
    ) -> bool {
        match EllipticCurve::from_bitsize(curve_bitsize) {
            Some(ec) => ec.is_on_curve(x_point, y_point),
            None => false,
        }
    }

    #[endpoint]
    fn compute_scalar_mult(
        &self,
        curve_bitsize: u32,
        x_point: BaseBigUint,
        y_point: BaseBigUint,
        data: &[u8],
    ) -> MultiValue2<BaseBigUint, BaseBigUint> {
        match EllipticCurve::from_bitsize(curve_bitsize) {
            Some(ec) => ec.scalar_mult_legacy(x_point, y_point, data).into(),
            None => (BaseBigUint::zero(), BaseBigUint::zero()).into(),
        }
    }

    #[endpoint]
    fn compute_scalar_base_mult(
        &self,
        curve_bitsize: u32,
        data: &[u8],
    ) -> MultiValue2<BaseBigUint, BaseBigUint> {
        match EllipticCurve::from_bitsize(curve_bitsize) {
            Some(ec) => ec.scalar_base_mult_legacy(data).into(),
            None => (BaseBigUint::zero(), BaseBigUint::zero()).into(),
        }
    }

    #[endpoint]
    fn compute_marshal_ec(
        &self,
        curve_bitsize: u32,
        x_pair: BaseBigUint,
        y_pair: BaseBigUint,
    ) -> BoxedBytes {
        match EllipticCurve::from_bitsize(curve_bitsize) {
            Some(ec) => ec.marshal_legacy(x_pair, y_pair),
            None => BoxedBytes::zeros(0),
        }
    }

    #[endpoint]
    fn compute_marshal_compressed_ec(
        &self,
        curve_bitsize: u32,
        x_pair: BaseBigUint,
        y_pair: BaseBigUint,
    ) -> BoxedBytes {
        match EllipticCurve::from_bitsize(curve_bitsize) {
            Some(ec) => ec.marshal_compressed_legacy(x_pair, y_pair),
            None => BoxedBytes::zeros(0),
        }
    }

    #[endpoint]
    fn compute_unmarshal_ec(
        &self,
        curve_bitsize: u32,
        data: &[u8],
    ) -> MultiValue2<BaseBigUint, BaseBigUint> {
        match EllipticCurve::from_bitsize(curve_bitsize) {
            Some(ec) => ec.unmarshal_legacy(data).into(),
            None => (BaseBigUint::zero(), BaseBigUint::zero()).into(),
        }
    }

    #[endpoint]
    fn compute_unmarshal_compressed_ec(
        &self,
        curve_bitsize: u32,
        data: &[u8],
    ) -> MultiValue2<BaseBigUint, BaseBigUint> {
        match EllipticCurve::from_bitsize(curve_bitsize) {
            Some(ec) => ec.unmarshal_compressed_legacy(data).into(),
            None => (BaseBigUint::zero(), BaseBigUint::zero()).into(),
        }
    }

    #[endpoint]
    fn compute_generate_key_ec(
        &self,
        curve_bitsize: u32,
    ) -> MultiValue3<BaseBigUint, BaseBigUint, BoxedBytes> {
        match EllipticCurve::from_bitsize(curve_bitsize) {
            Some(ec) => ec.generate_key_legacy().into(),
            None => (BaseBigUint::zero(), BaseBigUint::zero(), BoxedBytes::zeros(0)).into(),
        }
    }
}
