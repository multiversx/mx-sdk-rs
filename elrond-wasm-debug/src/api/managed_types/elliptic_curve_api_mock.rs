use elrond_wasm::{
    api::{EllipticCurveApi, Handle},
    types::BoxedBytes,
};

use crate::DebugApi;

impl EllipticCurveApi for DebugApi {
    fn ec_create(&self, _name: &[u8]) -> Handle {
        panic!("ec_create not implemented")
    }

    fn ec_get_values(
        &self,
        _ec_handle: Handle,
        _field_order_handle: Handle,
        _base_point_order_handle: Handle,
        _eq_constant_handle: Handle,
        _x_base_point_handle: Handle,
        _y_base_point_handle: Handle,
    ) {
        panic!("ec_get_values not implemented")
    }

    fn ec_curve_length(&self, _ec_handle: Handle) -> u32 {
        panic!("ec_curve_length not implemented")
    }

    fn ec_private_key_byte_length(&self, _ec_handle: Handle) -> u32 {
        panic!("ec_private_key_byte_length not implemented")
    }

    fn ec_add(
        &self,
        _x_result_handle: Handle,
        _y_result_handle: Handle,
        _ec_handle: Handle,
        _x_first_point: Handle,
        _y_first_point: Handle,
        _x_second_point: Handle,
        _y_second_point: Handle,
    ) {
        panic!("ec_add not implemented")
    }

    fn ec_double(
        &self,
        _x_result_handle: Handle,
        _y_result_handle: Handle,
        _ec_handle: Handle,
        _x_point_handle: Handle,
        _y_point_handle: Handle,
    ) {
        panic!("ec_double not implemented")
    }

    fn ec_is_on_curve(
        &self,
        _ec_handle: Handle,
        _x_point_handle: Handle,
        _y_point_handle: Handle,
    ) -> bool {
        panic!("ec_is_on_curve not implemented")
    }

    fn ec_scalar_mult(
        &self,
        _x_result_handle: Handle,
        _y_result_handle: Handle,
        _ec_handle: Handle,
        _x_point_handle: Handle,
        _y_point_handle: Handle,
        _data: &[u8],
    ) {
        panic!("ec_scalar_mult not implemented")
    }

    fn ec_scalar_base_mult(
        &self,
        _x_result_handle: Handle,
        _y_result_handle: Handle,
        _ec_handle: Handle,
        _data: &[u8],
    ) {
        panic!("ec_scalar_base_mult not implemented")
    }

    fn ec_marshal(
        &self,
        _ec_handle: Handle,
        _x_pair_handle: Handle,
        _y_pair_handle: Handle,
    ) -> BoxedBytes {
        panic!("ec_marshal not implemented")
    }

    fn ec_marshal_compressed(
        &self,
        _ec_handle: Handle,
        _x_pair_handle: Handle,
        _y_pair_handle: Handle,
    ) -> BoxedBytes {
        panic!("ec_marshal_compressed not implemented")
    }

    fn ec_unmarshal(
        &self,
        _x_result_handle: Handle,
        _y_result_handle: Handle,
        _ec_handle: Handle,
        _data: &[u8],
    ) {
        panic!("ec_unmarshal not implemented")
    }

    fn ec_unmarshal_compressed(
        &self,
        _x_result_handle: Handle,
        _y_result_handle: Handle,
        _ec_handle: Handle,
        _data: &[u8],
    ) {
        panic!("ec_unmarshal_compressed not implemented")
    }

    fn ec_generate_key(
        &self,
        _x_pub_key_handle: Handle,
        _y_pub_key_handle: Handle,
        _ec_handle: Handle,
    ) -> BoxedBytes {
        panic!("ec_generate_key not implemented")
    }
}
