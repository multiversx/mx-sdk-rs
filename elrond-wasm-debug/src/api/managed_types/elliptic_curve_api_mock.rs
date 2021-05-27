use core::panic;

use super::RustBigUint;
use elrond_wasm::elrond_codec::*;
pub struct EllipticCurveMock;

impl elrond_wasm::abi::TypeAbi for EllipticCurveMock {
    fn type_name() -> String {
        String::from("EllipticCurve")
    }
}

impl elrond_wasm::api::EllipticCurveApi for EllipticCurveMock{
    type EllipticCurve = EllipticCurveMock;
    type BigUint = RustBigUint;

    fn new_elliptic_curve(field_order: Self::BigUint, base_point_order: Self::BigUint, eq_constant: Self::BigUint, x_base_point: Self::BigUint, y_base_point: Self::BigUint, size_of_field: i32) -> Self {
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

    fn add_ec(&self, curve: &Self::EllipticCurve, x_first_point: Self::BigUint, y_first_point: Self::BigUint, x_second_point: Self::BigUint, y_second_point: Self::BigUint) ->(Self::BigUint,Self::BigUint) {
        panic!("add_ec not implemented yet!")
    }

    fn double_ec(&self, curve: &Self::EllipticCurve, x_point: Self::BigUint, y_point: Self::BigUint) -> (Self::BigUint, Self::BigUint) {
        panic!("double_ec not implemented yet!")
    }

    fn is_on_curve_ec(&self, curve: &Self::EllipticCurve, x_point: Self::BigUint, y_point: Self::BigUint) -> bool {
        panic!("is_on_curve_ec not implemented yet!")
    }

    fn scalar_mult(&self, curve: &Self::EllipticCurve, x_point: Self::BigUint, y_point: Self::BigUint, data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
        panic!("scalar_mult not implemented yet")
    }

    fn scalar_base_mult(&self, curve: &Self::EllipticCurve, data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
        panic!("scalar_base_mult not implemented yet!")
    }

    fn marshal_ec(&self, curve: &Self::EllipticCurve, x_pair: Self::BigUint, y_pair: Self::BigUint) -> BoxedBytes {
        panic!("marshal_ec not implemented yet!")
    }

    fn marshal_compressed_ec(&self, curve: &Self::EllipticCurve, x_pair: Self::BigUint, y_pair: Self::BigUint) -> BoxedBytes {
        panic!("marshal_compressed_ec not implemented yet!")
    }

    fn unmarshal_ec(&self, curve: &Self::EllipticCurve, data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
        panic!("unmarshal_ec not implemented yet!")
    }

    fn unmarshal_compressed_ec(&self, curve: &Self::EllipticCurve, data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
        panic!("unmarshal_compressed_ec not implemented yet!")
    }

    fn generate_key_ec(&self, curve: &Self::EllipticCurve) -> (Self::BigUint, Self::BigUint, BoxedBytes) {
        panic!("generate_key_ec not implemented yet!")
    }
}